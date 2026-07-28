use crate::core::models::AppSettings;

pub struct SettingsScheduleChanges;

impl SettingsScheduleChanges {
    pub fn eye_schedule_changed(old: &AppSettings, new: &AppSettings) -> bool {
        old.eye_reset_enabled != new.eye_reset_enabled
            || old.eye_reset_interval_minutes != new.eye_reset_interval_minutes
            || old.eye_reset_duration_seconds != new.eye_reset_duration_seconds
    }

    pub fn move_schedule_changed(old: &AppSettings, new: &AppSettings) -> bool {
        old.move_break_enabled != new.move_break_enabled
            || old.move_break_interval_minutes != new.move_break_interval_minutes
            || old.move_break_duration_seconds != new.move_break_duration_seconds
    }
}