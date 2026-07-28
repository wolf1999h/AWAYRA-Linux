use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreakType {
    Eye = 0,
    Move = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchedulerStatus {
    Running = 0,
    PausedManual = 1,
    PausedIdle = 2,
    OutsideWorkHours = 3,
    BreakActive = 4,
    Snoozed = 5,
    Disabled = 6,
    ConfigurationPaused = 7,
    Idle = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppTheme {
    Dark = 0,
    Light = 1,
}

impl Default for AppTheme {
    fn default() -> Self {
        Self::Dark
    }
}