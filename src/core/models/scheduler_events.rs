use crate::core::models::{BreakType, SchedulerStatus};

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct BreakStartedEventArgs {
    pub break_type: BreakType,
    pub duration_seconds: i32,
    pub activity_index: i32,
}

#[derive(Debug, Clone)]
pub struct BreakEndedEventArgs {
    pub break_type: BreakType,
    pub completed: bool,
    pub skipped: bool,
    pub snoozed: bool,
}

#[derive(Debug, Clone)]
pub struct SchedulerSnapshot {
    pub status: SchedulerStatus,
    pub is_paused_manual: bool,
    pub eye_remaining: chrono::Duration,
    pub move_remaining: chrono::Duration,
    pub eye_enabled: bool,
    pub move_enabled: bool,
    pub active_break: Option<BreakType>,
    pub queued_break: Option<BreakType>,
    pub active_break_remaining: Option<chrono::Duration>,
    pub next_break_due: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct SchedulerDiagnostics {
    pub status: SchedulerStatus,
    pub eye_remaining_seconds: i64,
    pub move_remaining_seconds: i64,
    pub eye_next_due: DateTime<Utc>,
    pub move_next_due: DateTime<Utc>,
    pub eye_snooze_until: Option<DateTime<Utc>>,
    pub move_snooze_until: Option<DateTime<Utc>>,
    pub is_paused_manual: bool,
    pub is_idle_paused: bool,
    pub is_configuration_paused: bool,
    pub is_outside_work_hours: bool,
    pub active_break: Option<BreakType>,
    pub queued_break: Option<BreakType>,
    pub glass_clarity: i32,
    pub background_tint_opacity: f64,
    pub blur_radius: f64,
    pub idle_seconds: f64,
    pub snapshot_captured: bool,
    pub eye_completed: i32,
    pub move_completed: i32,
    pub skipped: i32,
    pub snoozed: i32,
}