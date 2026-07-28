use chrono::{DateTime, Utc, Timelike};

pub struct WorkHoursEvaluator;

impl WorkHoursEvaluator {
    pub fn is_within_work_hours(
        now: DateTime<Utc>,
        work_hours_enabled: bool,
        start_hour: i32,
        start_minute: i32,
        end_hour: i32,
        end_minute: i32,
    ) -> bool {
        if !work_hours_enabled {
            return true;
        }

        let local = now.with_timezone(&chrono::Local);
        let current_minutes = local.hour() as i32 * 60 + local.minute() as i32;
        let start_minutes = start_hour * 60 + start_minute;
        let end_minutes = end_hour * 60 + end_minute;

        if start_minutes <= end_minutes {
            current_minutes >= start_minutes && current_minutes < end_minutes
        } else {
            // Overnight shift (e.g., 22:00 - 06:00)
            current_minutes >= start_minutes || current_minutes < end_minutes
        }
    }
}