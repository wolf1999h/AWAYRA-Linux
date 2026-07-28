use chrono::{DateTime, Duration, Utc};

use crate::core::models::*;
use crate::core::services::work_hours_evaluator::WorkHoursEvaluator;
use crate::core::services::settings_validator::SettingsValidator;
use crate::core::services::settings_schedule_changes::SettingsScheduleChanges;

pub const MOVE_ACTIVITY_COUNT: i32 = 5;

pub struct BreakScheduler {
    clock: Box<dyn Clock>,
    settings: AppSettings,
    state: SchedulerState,
    is_idle: bool,
    is_configuration_paused: bool,
    config_frozen_eye_remaining: Option<Duration>,
    config_frozen_move_remaining: Option<Duration>,
    config_frozen_eye_snooze_remaining: Option<Duration>,
    config_frozen_move_snooze_remaining: Option<Duration>,
    manual_frozen_eye_remaining: Option<Duration>,
    manual_frozen_move_remaining: Option<Duration>,
    outside_work_hours: bool,
    work_hours_frozen_eye_remaining: Option<Duration>,
    work_hours_frozen_move_remaining: Option<Duration>,
    idle_frozen_eye_remaining: Option<Duration>,
    idle_frozen_move_remaining: Option<Duration>,
    move_activity_index: i32,
    snooze_in_progress: bool,
}

pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

impl BreakScheduler {
    pub fn new(
        clock: Box<dyn Clock>,
        settings: AppSettings,
        persisted_state: Option<SchedulerState>,
    ) -> Self {
        let mut state = persisted_state.unwrap_or_else(|| SchedulerState::create_default(clock.now()));
        state.last_clock_check = clock.now();
        let mut s = Self {
            clock,
            settings,
            state,
            is_idle: false,
            is_configuration_paused: false,
            config_frozen_eye_remaining: None,
            config_frozen_move_remaining: None,
            config_frozen_eye_snooze_remaining: None,
            config_frozen_move_snooze_remaining: None,
            manual_frozen_eye_remaining: None,
            manual_frozen_move_remaining: None,
            outside_work_hours: false,
            work_hours_frozen_eye_remaining: None,
            work_hours_frozen_move_remaining: None,
            idle_frozen_eye_remaining: None,
            idle_frozen_move_remaining: None,
            move_activity_index: 0,
            snooze_in_progress: false,
        };
        s.normalize_state_on_load();
        s
    }

    pub fn settings(&self) -> &AppSettings {
        &self.settings
    }

    pub fn state(&self) -> &SchedulerState {
        &self.state
    }

    pub fn move_activity_index(&self) -> i32 {
        self.move_activity_index
    }

    pub fn get_snapshot(&self) -> SchedulerSnapshot {
        let now = self.clock.now();
        let status = self.compute_status(now);
        let eye_remaining = self.get_remaining(BreakType::Eye, now);
        let move_remaining = self.get_remaining(BreakType::Move, now);

        let active_break_remaining = match (self.state.active_break, self.state.break_ends_at) {
            (Some(_), Some(ends_at)) => {
                let rem = ends_at - now;
                if rem < Duration::zero() {
                    Some(Duration::zero())
                } else {
                    Some(rem)
                }
            }
            _ => None,
        };

        let next_break_due = if self.settings.eye_reset_enabled && self.settings.move_break_enabled {
            Some(std::cmp::min(self.state.eye_next_due, self.state.move_next_due))
        } else if self.settings.eye_reset_enabled {
            Some(self.state.eye_next_due)
        } else if self.settings.move_break_enabled {
            Some(self.state.move_next_due)
        } else {
            None
        };

        SchedulerSnapshot {
            status,
            is_paused_manual: self.state.is_paused_manual,
            eye_remaining,
            move_remaining,
            eye_enabled: self.settings.eye_reset_enabled,
            move_enabled: self.settings.move_break_enabled,
            active_break: self.state.active_break,
            queued_break: self.state.queued_break,
            active_break_remaining,
            next_break_due,
        }
    }

    pub fn tick(&mut self) {
        let now = self.clock.now();
        self.handle_clock_jump(now);
        self.state.last_clock_check = now;
        self.update_work_hours_freeze(now);

        if self.state.active_break.is_some() {
            if let Some(ends_at) = self.state.break_ends_at {
                if now >= ends_at {
                    self.complete_active_break();
                }
            }
            return;
        }

        if !self.can_deliver_reminders(now) {
            return;
        }

        self.try_start_due_break(now);
    }

    pub fn update_settings(&mut self, settings: AppSettings) {
        if !SettingsValidator::is_valid(&settings) {
            return;
        }

        let now = self.clock.now();
        let was_eye_enabled = self.settings.eye_reset_enabled;
        let was_move_enabled = self.settings.move_break_enabled;
        let old_eye_interval = self.settings.eye_reset_interval_minutes;
        let old_move_interval = self.settings.move_break_interval_minutes;
        self.settings = settings;

        if !was_eye_enabled && self.settings.eye_reset_enabled {
            self.state.eye_next_due = now + Duration::minutes(self.settings.eye_reset_interval_minutes as i64);
            self.clear_eye_freeze_state();
        } else if old_eye_interval != self.settings.eye_reset_interval_minutes || !self.settings.eye_reset_enabled {
            self.reschedule_on_interval_change(BreakType::Eye, now, self.settings.eye_reset_interval_minutes, self.settings.eye_reset_enabled);
        }

        if !was_move_enabled && self.settings.move_break_enabled {
            self.state.move_next_due = now + Duration::minutes(self.settings.move_break_interval_minutes as i64);
            self.clear_move_freeze_state();
        } else if old_move_interval != self.settings.move_break_interval_minutes || !self.settings.move_break_enabled {
            self.reschedule_on_interval_change(BreakType::Move, now, self.settings.move_break_interval_minutes, self.settings.move_break_enabled);
        }
    }

    pub fn enter_configuration_pause(&mut self) {
        let now = self.clock.now();
        self.config_frozen_eye_remaining = Some(self.get_raw_remaining(BreakType::Eye, now));
        self.config_frozen_move_remaining = Some(self.get_raw_remaining(BreakType::Move, now));

        if let Some(snooze_until) = self.state.eye_snooze_until {
            if now < snooze_until {
                self.config_frozen_eye_snooze_remaining = self.config_frozen_eye_remaining;
            }
        }
        if let Some(snooze_until) = self.state.move_snooze_until {
            if now < snooze_until {
                self.config_frozen_move_snooze_remaining = self.config_frozen_move_remaining;
            }
        }

        self.is_configuration_paused = true;
    }

    pub fn apply_configuration_save(&mut self, settings: AppSettings, save_time: DateTime<Utc>) {
        if !SettingsValidator::is_valid(&settings) {
            return;
        }

        let original_settings = std::mem::replace(&mut self.settings, settings);
        let eye_schedule_changed = SettingsScheduleChanges::eye_schedule_changed(&original_settings, &self.settings);
        let move_schedule_changed = SettingsScheduleChanges::move_schedule_changed(&original_settings, &self.settings);

        let frozen_eye = self.config_frozen_eye_remaining.take();
        let frozen_move = self.config_frozen_move_remaining.take();
        let frozen_eye_snooze = self.config_frozen_eye_snooze_remaining.take();
        let frozen_move_snooze = self.config_frozen_move_snooze_remaining.take();

        self.is_configuration_paused = false;

        if eye_schedule_changed {
            if self.settings.eye_reset_enabled {
                self.state.eye_next_due = save_time + Duration::minutes(self.settings.eye_reset_interval_minutes as i64);
            }
            self.state.eye_snooze_until = None;
        } else if let Some(frozen) = frozen_eye {
            if self.settings.eye_reset_enabled {
                self.state.eye_next_due = save_time + frozen;
                if frozen_eye_snooze.is_some() {
                    self.state.eye_snooze_until = Some(self.state.eye_next_due);
                }
            }
        }

        if move_schedule_changed {
            if self.settings.move_break_enabled {
                self.state.move_next_due = save_time + Duration::minutes(self.settings.move_break_interval_minutes as i64);
            }
            self.state.move_snooze_until = None;
        } else if let Some(frozen) = frozen_move {
            if self.settings.move_break_enabled {
                self.state.move_next_due = save_time + frozen;
                if frozen_move_snooze.is_some() {
                    self.state.move_snooze_until = Some(self.state.move_next_due);
                }
            }
        }

        if eye_schedule_changed || move_schedule_changed {
            self.state.queued_break = None;
        }
    }

    pub fn cancel_configuration_pause(&mut self) {
        let now = self.clock.now();
        if let Some(frozen) = self.config_frozen_eye_remaining {
            if self.settings.eye_reset_enabled {
                self.state.eye_next_due = now + frozen;
                if self.config_frozen_eye_snooze_remaining.is_some() {
                    self.state.eye_snooze_until = Some(self.state.eye_next_due);
                }
            }
        }
        if let Some(frozen) = self.config_frozen_move_remaining {
            if self.settings.move_break_enabled {
                self.state.move_next_due = now + frozen;
                if self.config_frozen_move_snooze_remaining.is_some() {
                    self.state.move_snooze_until = Some(self.state.move_next_due);
                }
            }
        }
        self.is_configuration_paused = false;
        self.config_frozen_eye_remaining = None;
        self.config_frozen_move_remaining = None;
        self.config_frozen_eye_snooze_remaining = None;
        self.config_frozen_move_snooze_remaining = None;
    }

    pub fn pause(&mut self) {
        let now = self.clock.now();
        self.manual_frozen_eye_remaining = Some(self.get_raw_remaining(BreakType::Eye, now));
        self.manual_frozen_move_remaining = Some(self.get_raw_remaining(BreakType::Move, now));
        self.state.is_paused_manual = true;
    }

    pub fn resume(&mut self) {
        let now = self.clock.now();
        if let Some(frozen) = self.manual_frozen_eye_remaining {
            if self.settings.eye_reset_enabled {
                self.state.eye_next_due = now + frozen;
            }
        }
        if let Some(frozen) = self.manual_frozen_move_remaining {
            if self.settings.move_break_enabled {
                self.state.move_next_due = now + frozen;
            }
        }
        self.manual_frozen_eye_remaining = None;
        self.manual_frozen_move_remaining = None;
        self.state.is_paused_manual = false;
    }

    pub fn set_idle(&mut self, is_idle: bool) {
        if self.is_idle == is_idle {
            return;
        }

        if is_idle && !self.is_idle && self.settings.pause_while_idle {
            let now = self.clock.now();
            self.idle_frozen_eye_remaining = Some(self.get_raw_remaining(BreakType::Eye, now));
            self.idle_frozen_move_remaining = Some(self.get_raw_remaining(BreakType::Move, now));
        }

        if !is_idle && self.is_idle && self.settings.pause_while_idle {
            self.reset_intervals_after_idle_return(self.clock.now());
        }

        self.is_idle = is_idle;
    }

    pub fn trigger_now(&mut self, break_type: BreakType) {
        if self.is_configuration_paused {
            return;
        }
        if !self.is_break_enabled(break_type) {
            return;
        }
        if self.state.active_break.is_some() {
            if self.state.queued_break.is_none() && self.state.active_break != Some(break_type) {
                self.state.queued_break = Some(break_type);
            }
            return;
        }
        self.start_break(break_type, true);
    }

    pub fn complete_active_break(&mut self) {
        let break_type = match self.state.active_break {
            Some(bt) => bt,
            None => return,
        };
        self.end_break(break_type, true, false, false);
        self.try_start_queued_or_due();
    }

    pub fn skip_active_break(&mut self) {
        let break_type = match self.state.active_break {
            Some(bt) => bt,
            None => return,
        };
        if !self.settings.allow_skip {
            return;
        }
        self.end_break(break_type, false, true, false);
        self.try_start_queued_or_due();
    }

    pub fn snooze_active_break(&mut self) {
        let break_type = match self.state.active_break {
            Some(bt) => bt,
            None => return,
        };
        if !self.settings.allow_snooze || self.snooze_in_progress {
            return;
        }

        self.snooze_in_progress = true;
        let now = self.clock.now();
        let snooze_end = now + Duration::minutes(self.settings.snooze_duration_minutes as i64);

        match break_type {
            BreakType::Eye => {
                self.state.eye_next_due = snooze_end;
                self.state.eye_snooze_until = Some(snooze_end);
            }
            BreakType::Move => {
                self.state.move_next_due = snooze_end;
                self.state.move_snooze_until = Some(snooze_end);
            }
        }

        self.end_break(break_type, false, false, true);
        self.snooze_in_progress = false;
    }

    pub fn restore_state(&mut self, state: SchedulerState) {
        self.state = state;
        self.normalize_state_on_load();
    }

    // --- Private helpers ---

    fn clear_eye_freeze_state(&mut self) {
        self.manual_frozen_eye_remaining = None;
        self.work_hours_frozen_eye_remaining = None;
        self.config_frozen_eye_remaining = None;
    }

    fn clear_move_freeze_state(&mut self) {
        self.manual_frozen_move_remaining = None;
        self.work_hours_frozen_move_remaining = None;
        self.config_frozen_move_remaining = None;
    }

    fn normalize_state_on_load(&mut self) {
        let now = self.clock.now();
        if self.state.eye_next_due == DateTime::<Utc>::default() {
            self.state.eye_next_due = now + Duration::minutes(self.settings.eye_reset_interval_minutes as i64);
        }
        if self.state.move_next_due == DateTime::<Utc>::default() {
            self.state.move_next_due = now + Duration::minutes(self.settings.move_break_interval_minutes as i64);
        }
        if self.state.last_clock_check == DateTime::<Utc>::default() {
            self.state.last_clock_check = now;
        }
        self.migrate_legacy_snooze_state();
    }

    fn migrate_legacy_snooze_state(&mut self) {
        // Legacy snooze_until field is not in our model, so this is a no-op.
        // In the C# version, it migrated from a single SnoozeUntil to per-break snooze.
    }

    fn is_any_break_snoozed(&self, now: DateTime<Utc>) -> bool {
        (self.state.eye_snooze_until.is_some() && now < self.state.eye_snooze_until.unwrap())
            || (self.state.move_snooze_until.is_some() && now < self.state.move_snooze_until.unwrap())
    }

    fn handle_clock_jump(&mut self, now: DateTime<Utc>) {
        let delta = now - self.state.last_clock_check;
        if delta < Duration::minutes(-1) {
            if self.state.eye_next_due < now {
                self.state.eye_next_due = now + Duration::minutes(self.settings.eye_reset_interval_minutes as i64);
            }
            if self.state.move_next_due < now {
                self.state.move_next_due = now + Duration::minutes(self.settings.move_break_interval_minutes as i64);
            }
        }
    }

    fn can_deliver_reminders(&self, now: DateTime<Utc>) -> bool {
        if self.state.is_paused_manual {
            return false;
        }
        if self.is_configuration_paused {
            return false;
        }
        if self.settings.pause_while_idle && self.is_idle {
            return false;
        }
        if !WorkHoursEvaluator::is_within_work_hours(
            now,
            self.settings.work_hours_enabled,
            self.settings.work_start_hour,
            self.settings.work_start_minute,
            self.settings.work_end_hour,
            self.settings.work_end_minute,
        ) {
            return false;
        }
        true
    }

    fn compute_status(&self, now: DateTime<Utc>) -> SchedulerStatus {
        if self.state.active_break.is_some() {
            return SchedulerStatus::BreakActive;
        }
        if self.is_any_break_snoozed(now) {
            return SchedulerStatus::Snoozed;
        }
        if !self.settings.eye_reset_enabled && !self.settings.move_break_enabled {
            return SchedulerStatus::Disabled;
        }
        if self.state.is_paused_manual {
            return SchedulerStatus::PausedManual;
        }
        if self.is_configuration_paused {
            return SchedulerStatus::ConfigurationPaused;
        }
        if self.settings.pause_while_idle && self.is_idle {
            return SchedulerStatus::Idle;
        }
        if !WorkHoursEvaluator::is_within_work_hours(
            now,
            self.settings.work_hours_enabled,
            self.settings.work_start_hour,
            self.settings.work_start_minute,
            self.settings.work_end_hour,
            self.settings.work_end_minute,
        ) {
            return SchedulerStatus::OutsideWorkHours;
        }
        SchedulerStatus::Running
    }

    fn get_remaining(&self, break_type: BreakType, now: DateTime<Utc>) -> Duration {
        if !self.is_break_enabled(break_type) {
            return Duration::zero();
        }

        if self.state.is_paused_manual {
            let frozen = match break_type {
                BreakType::Eye => self.manual_frozen_eye_remaining,
                BreakType::Move => self.manual_frozen_move_remaining,
            };
            if let Some(f) = frozen {
                return f;
            }
        }

        if self.is_configuration_paused {
            let frozen = match break_type {
                BreakType::Eye => self.config_frozen_eye_remaining,
                BreakType::Move => self.config_frozen_move_remaining,
            };
            if let Some(f) = frozen {
                return f;
            }
        }

        if self.settings.work_hours_enabled && self.outside_work_hours {
            let frozen = match break_type {
                BreakType::Eye => self.work_hours_frozen_eye_remaining,
                BreakType::Move => self.work_hours_frozen_move_remaining,
            };
            if let Some(f) = frozen {
                return f;
            }
        }

        if self.settings.pause_while_idle && self.is_idle {
            let frozen = match break_type {
                BreakType::Eye => self.idle_frozen_eye_remaining,
                BreakType::Move => self.idle_frozen_move_remaining,
            };
            if let Some(f) = frozen {
                return f;
            }
        }

        self.get_raw_remaining(break_type, now)
    }

    fn get_raw_remaining(&self, break_type: BreakType, now: DateTime<Utc>) -> Duration {
        let due = match break_type {
            BreakType::Eye => self.state.eye_next_due,
            BreakType::Move => self.state.move_next_due,
        };
        let remaining = due - now;
        if remaining < Duration::zero() {
            Duration::zero()
        } else {
            remaining
        }
    }

    fn is_break_enabled(&self, break_type: BreakType) -> bool {
        match break_type {
            BreakType::Eye => self.settings.eye_reset_enabled,
            BreakType::Move => self.settings.move_break_enabled,
        }
    }

    fn try_start_due_break(&mut self, now: DateTime<Utc>) {
        if self.is_any_break_snoozed(now) {
            return;
        }

        let mut due_breaks: Vec<(BreakType, DateTime<Utc>)> = Vec::new();
        if self.settings.eye_reset_enabled && now >= self.state.eye_next_due {
            due_breaks.push((BreakType::Eye, self.state.eye_next_due));
        }
        if self.settings.move_break_enabled && now >= self.state.move_next_due {
            due_breaks.push((BreakType::Move, self.state.move_next_due));
        }

        if due_breaks.is_empty() {
            return;
        }

        due_breaks.sort_by(|a, b| a.1.cmp(&b.1));
        let first = due_breaks[0].0;
        self.start_break(first, false);

        if due_breaks.len() > 1 && self.state.queued_break.is_none() {
            self.state.queued_break = Some(due_breaks[1].0);
        }
    }

    fn try_start_queued_or_due(&mut self) {
        if self.state.active_break.is_some() || self.is_any_break_snoozed(self.clock.now()) {
            return;
        }
        if !self.can_deliver_reminders(self.clock.now()) {
            return;
        }

        if let Some(queued) = self.state.queued_break.take() {
            self.start_break(queued, false);
            return;
        }

        self.try_start_due_break(self.clock.now());
    }

    fn start_break(&mut self, break_type: BreakType, _manual: bool) {
        if !self.is_break_enabled(break_type) {
            return;
        }

        let now = self.clock.now();
        let duration_seconds = match break_type {
            BreakType::Eye => self.settings.eye_reset_duration_seconds,
            BreakType::Move => self.settings.move_break_duration_seconds,
        };

        self.state.active_break = Some(break_type);
        self.state.break_ends_at = Some(now + Duration::seconds(duration_seconds as i64));
        self.state.queued_break = None;

        match break_type {
            BreakType::Eye => self.state.eye_snooze_until = None,
            BreakType::Move => self.state.move_snooze_until = None,
        }

        if break_type == BreakType::Move {
            self.move_activity_index = (self.move_activity_index + 1) % MOVE_ACTIVITY_COUNT;
        }
    }

    fn end_break(&mut self, break_type: BreakType, completed: bool, skipped: bool, _snoozed: bool) {
        let now = self.clock.now();
        self.state.active_break = None;
        self.state.break_ends_at = None;

        if completed || skipped {
            self.schedule_next_from_completion(break_type, now);
        }
        // If snoozed, per-break snooze due times were set by the caller.
    }

    fn schedule_next_from_completion(&mut self, break_type: BreakType, from: DateTime<Utc>) {
        let interval = match break_type {
            BreakType::Eye => self.settings.eye_reset_interval_minutes,
            BreakType::Move => self.settings.move_break_interval_minutes,
        };

        match break_type {
            BreakType::Eye => {
                self.state.eye_next_due = from + Duration::minutes(interval as i64);
                self.state.eye_last_completed = Some(from);
            }
            BreakType::Move => {
                self.state.move_next_due = from + Duration::minutes(interval as i64);
                self.state.move_last_completed = Some(from);
            }
        }
    }

    fn reschedule_on_interval_change(&mut self, break_type: BreakType, now: DateTime<Utc>, interval_minutes: i32, enabled: bool) {
        if !enabled {
            return;
        }

        let last_completed = match break_type {
            BreakType::Eye => self.state.eye_last_completed,
            BreakType::Move => self.state.move_last_completed,
        };
        let anchor = last_completed.unwrap_or(now);
        let mut next = anchor + Duration::minutes(interval_minutes as i64);
        if next < now {
            next = now;
        }

        match break_type {
            BreakType::Eye => self.state.eye_next_due = next,
            BreakType::Move => self.state.move_next_due = next,
        }
    }

    fn update_work_hours_freeze(&mut self, now: DateTime<Utc>) {
        if !self.settings.work_hours_enabled {
            if self.outside_work_hours {
                self.resume_from_work_hours_freeze(now);
            }
            self.outside_work_hours = false;
            return;
        }

        let inside = WorkHoursEvaluator::is_within_work_hours(
            now,
            true,
            self.settings.work_start_hour,
            self.settings.work_start_minute,
            self.settings.work_end_hour,
            self.settings.work_end_minute,
        );

        if !inside && !self.outside_work_hours {
            self.work_hours_frozen_eye_remaining = Some(self.get_raw_remaining(BreakType::Eye, now));
            self.work_hours_frozen_move_remaining = Some(self.get_raw_remaining(BreakType::Move, now));
            self.outside_work_hours = true;
        } else if inside && self.outside_work_hours {
            self.resume_from_work_hours_freeze(now);
            self.outside_work_hours = false;
        } else if !inside {
            self.outside_work_hours = true;
        } else {
            self.outside_work_hours = false;
        }
    }

    fn resume_from_work_hours_freeze(&mut self, now: DateTime<Utc>) {
        if let Some(frozen) = self.work_hours_frozen_eye_remaining {
            if self.settings.eye_reset_enabled {
                self.state.eye_next_due = now + frozen;
            }
        }
        if let Some(frozen) = self.work_hours_frozen_move_remaining {
            if self.settings.move_break_enabled {
                self.state.move_next_due = now + frozen;
            }
        }
        self.work_hours_frozen_eye_remaining = None;
        self.work_hours_frozen_move_remaining = None;
    }

    fn reset_intervals_after_idle_return(&mut self, now: DateTime<Utc>) {
        self.idle_frozen_eye_remaining = None;
        self.idle_frozen_move_remaining = None;

        if self.settings.eye_reset_enabled {
            self.state.eye_next_due = now + Duration::minutes(self.settings.eye_reset_interval_minutes as i64);
        }
        self.state.eye_snooze_until = None;

        if self.settings.move_break_enabled {
            self.state.move_next_due = now + Duration::minutes(self.settings.move_break_interval_minutes as i64);
        }
        self.state.move_snooze_until = None;
        self.state.queued_break = None;
    }
}