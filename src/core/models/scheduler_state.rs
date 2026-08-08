use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::core::models::BreakType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerState {
    pub eye_next_due: DateTime<Utc>,
    pub move_next_due: DateTime<Utc>,
    pub eye_last_completed: Option<DateTime<Utc>>,
    pub move_last_completed: Option<DateTime<Utc>>,
    pub eye_snooze_until: Option<DateTime<Utc>>,
    pub move_snooze_until: Option<DateTime<Utc>>,
    pub is_paused_manual: bool,
    /// When a fixed-duration pause (or "until tomorrow") ends, auto-resume.
    /// None while not paused or when paused indefinitely.
    pub paused_until: Option<DateTime<Utc>>,
    pub active_break: Option<BreakType>,
    pub break_ends_at: Option<DateTime<Utc>>,
    pub queued_break: Option<BreakType>,
    pub last_clock_check: DateTime<Utc>,
}

impl SchedulerState {
    pub fn create_default(now: DateTime<Utc>) -> Self {
        Self {
            eye_next_due: now + chrono::Duration::minutes(20),
            move_next_due: now + chrono::Duration::minutes(45),
            eye_last_completed: None,
            move_last_completed: None,
            eye_snooze_until: None,
            move_snooze_until: None,
            is_paused_manual: false,
            paused_until: None,
            active_break: None,
            break_ends_at: None,
            queued_break: None,
            last_clock_check: now,
        }
    }
}