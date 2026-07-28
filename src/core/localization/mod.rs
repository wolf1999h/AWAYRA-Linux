pub mod strings;

use strings::Strings;

pub struct LocalizationService {
    strings: Strings,
}

impl LocalizationService {
    pub fn new() -> Self {
        Self {
            strings: Strings::new(),
        }
    }

    pub fn get(&self, key: &str) -> String {
        self.strings.get(key)
    }

    pub fn get_status(&self, status: crate::core::models::SchedulerStatus) -> String {
        match status {
            crate::core::models::SchedulerStatus::Running => self.get("StatusRunning"),
            crate::core::models::SchedulerStatus::PausedManual => self.get("StatusPaused"),
            crate::core::models::SchedulerStatus::PausedIdle => self.get("StatusPausedIdle"),
            crate::core::models::SchedulerStatus::Idle => self.get("StatusIdle"),
            crate::core::models::SchedulerStatus::ConfigurationPaused => self.get("StatusConfigurationPaused"),
            crate::core::models::SchedulerStatus::OutsideWorkHours => self.get("StatusOutsideWorkHours"),
            crate::core::models::SchedulerStatus::BreakActive => self.get("StatusBreakActive"),
            crate::core::models::SchedulerStatus::Snoozed => self.get("StatusSnoozed"),
            crate::core::models::SchedulerStatus::Disabled => self.get("StatusDisabled"),
        }
    }

    pub fn get_move_activity(&self, index: i32) -> String {
        match index {
            0 => self.get("MoveActivityStand"),
            1 => self.get("MoveActivityWalk"),
            2 => self.get("MoveActivityShoulders"),
            3 => self.get("MoveActivityNeck"),
            _ => self.get("MoveActivityStretch"),
        }
    }
}

impl Default for LocalizationService {
    fn default() -> Self {
        Self::new()
    }
}