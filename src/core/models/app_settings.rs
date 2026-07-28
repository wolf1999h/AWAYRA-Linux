use serde::{Deserialize, Serialize};
use crate::core::models::AppTheme;

pub const DEFAULT_EYE_INTERVAL_MINUTES: i32 = 20;
pub const DEFAULT_EYE_DURATION_SECONDS: i32 = 20;
pub const DEFAULT_MOVE_INTERVAL_MINUTES: i32 = 45;
pub const DEFAULT_MOVE_DURATION_SECONDS: i32 = 60;
pub const DEFAULT_SNOOZE_DURATION_MINUTES: i32 = 5;
pub const DEFAULT_IDLE_THRESHOLD_MINUTES: i32 = 5;
pub const DEFAULT_GLASS_CLARITY: i32 = 75;
pub const CURRENT_SCHEMA_VERSION: i32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub schema_version: i32,

    pub eye_reset_enabled: bool,
    pub eye_reset_interval_minutes: i32,
    pub eye_reset_duration_seconds: i32,

    pub move_break_enabled: bool,
    pub move_break_interval_minutes: i32,
    pub move_break_duration_seconds: i32,

    pub allow_skip: bool,
    pub allow_snooze: bool,
    pub snooze_duration_minutes: i32,

    pub pause_while_idle: bool,
    pub idle_threshold_minutes: i32,

    pub work_hours_enabled: bool,
    pub work_start_hour: i32,
    pub work_start_minute: i32,
    pub work_end_hour: i32,
    pub work_end_minute: i32,

    pub run_at_startup: bool,
    pub start_minimized: bool,
    pub close_to_tray: bool,

    pub glass_clarity: i32,
    pub reduced_motion: bool,
    pub theme: AppTheme,

    /// Whether to capture a screenshot for the overlay background.
    /// If false, uses a solid dark/plain background instead.
    pub capture_screenshot: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,

            eye_reset_enabled: true,
            eye_reset_interval_minutes: DEFAULT_EYE_INTERVAL_MINUTES,
            eye_reset_duration_seconds: DEFAULT_EYE_DURATION_SECONDS,

            move_break_enabled: true,
            move_break_interval_minutes: DEFAULT_MOVE_INTERVAL_MINUTES,
            move_break_duration_seconds: DEFAULT_MOVE_DURATION_SECONDS,

            allow_skip: true,
            allow_snooze: true,
            snooze_duration_minutes: DEFAULT_SNOOZE_DURATION_MINUTES,

            pause_while_idle: true,
            idle_threshold_minutes: DEFAULT_IDLE_THRESHOLD_MINUTES,

            work_hours_enabled: false,
            work_start_hour: 9,
            work_start_minute: 0,
            work_end_hour: 18,
            work_end_minute: 0,

            run_at_startup: false,
            start_minimized: false,
            close_to_tray: true,

            glass_clarity: DEFAULT_GLASS_CLARITY,
            reduced_motion: false,
            theme: AppTheme::Dark,

            capture_screenshot: true,
        }
    }
}