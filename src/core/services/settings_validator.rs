use crate::core::models::AppSettings;

pub struct SettingsValidator;

impl SettingsValidator {
    pub fn is_valid(settings: &AppSettings) -> bool {
        if settings.eye_reset_enabled {
            if settings.eye_reset_interval_minutes < 1 || settings.eye_reset_interval_minutes > 480 {
                return false;
            }
            if settings.eye_reset_duration_seconds < 5 || settings.eye_reset_duration_seconds > 600 {
                return false;
            }
        }

        if settings.move_break_enabled {
            if settings.move_break_interval_minutes < 1 || settings.move_break_interval_minutes > 480 {
                return false;
            }
            if settings.move_break_duration_seconds < 5 || settings.move_break_duration_seconds > 600 {
                return false;
            }
        }

        if settings.allow_snooze {
            if settings.snooze_duration_minutes < 1 || settings.snooze_duration_minutes > 60 {
                return false;
            }
        }

        if settings.pause_while_idle {
            if settings.idle_threshold_minutes < 1 || settings.idle_threshold_minutes > 120 {
                return false;
            }
        }

        if settings.glass_clarity < 0 || settings.glass_clarity > 150 {
            return false;
        }

        if settings.work_hours_enabled {
            let start = settings.work_start_hour * 60 + settings.work_start_minute;
            let end = settings.work_end_hour * 60 + settings.work_end_minute;
            if start == end {
                return false;
            }
        }

        true
    }
}