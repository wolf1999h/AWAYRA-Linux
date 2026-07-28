use std::collections::HashMap;

pub struct Strings {
    map: HashMap<&'static str, &'static str>,
}

impl Strings {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("AppTitle", "Awayra");
        map.insert("StatusRunning", "Reminders active");
        map.insert("StatusPaused", "Paused");
        map.insert("StatusPausedIdle", "Paused while idle");
        map.insert("StatusIdle", "Idle");
        map.insert("StatusConfigurationPaused", "Settings open");
        map.insert("StatusOutsideWorkHours", "Outside work hours");
        map.insert("StatusBreakActive", "Break in progress");
        map.insert("StatusSnoozed", "Snoozed");
        map.insert("StatusDisabled", "Reminders disabled");
        map.insert("EyeReset", "Eye Reset");
        map.insert("MoveBreak", "Move Break");
        map.insert("Enabled", "Enabled");
        map.insert("Disabled", "Disabled");
        map.insert("Pause", "Pause");
        map.insert("Resume", "Resume");
        map.insert("EyeResetNow", "Eye Reset Now");
        map.insert("MoveBreakNow", "Move Break Now");
        map.insert("Settings", "Settings");
        map.insert("Quit", "Quit");
        map.insert("OpenAwayra", "Open Awayra");
        map.insert("TodayEyeCompleted", "Eye resets today");
        map.insert("TodayMoveCompleted", "Move breaks today");
        map.insert("TodaySkipped", "Skipped today");
        map.insert("TodaySnoozed", "Snoozed today");
        map.insert("EyeResetInstructionDistance", "Look at a distant object for a few breaths.");
        map.insert("EyeResetInstructionBlink", "Blink naturally and relax your face.");
        map.insert("Skip", "Skip");
        map.insert("Snooze", "Snooze");
        map.insert("Complete", "Complete");
        map.insert("SecondsRemaining", "seconds remaining");
        map.insert("MoveActivityStand", "Stand up and reset your posture.");
        map.insert("MoveActivityWalk", "Walk briefly away from your desk.");
        map.insert("MoveActivityShoulders", "Relax your shoulders and unclench your jaw.");
        map.insert("MoveActivityNeck", "Gently move your neck within a comfortable range.");
        map.insert("MoveActivityStretch", "Stretch your arms gently without forcing.");
        map.insert("TrayTooltipNextBreak", "Next break in {0}");
        map.insert("TrayPauseReminders", "Pause Reminders");
        map.insert("TrayResumeReminders", "Resume Reminders");
        map.insert("SettingsEyeReset", "Eye Reset");
        map.insert("SettingsMoveBreak", "Move Break");
        map.insert("SettingsBehavior", "Behavior");
        map.insert("SettingsAppearance", "Appearance");
        map.insert("SettingsEnabled", "Enabled");
        map.insert("SettingsIntervalMinutes", "Interval (minutes)");
        map.insert("SettingsDurationSeconds", "Duration (seconds)");
        map.insert("SettingsAllowSkip", "Allow skip");
        map.insert("SettingsAllowSnooze", "Allow snooze");
        map.insert("SettingsSnoozeDuration", "Snooze duration (minutes)");
        map.insert("SettingsPauseWhileIdle", "Reset reminders after idle");
        map.insert("SettingsIdleThreshold", "Idle threshold (minutes)");
        map.insert("SettingsWorkHoursEnabled", "Enable work hours");
        map.insert("SettingsWorkStart", "Work start");
        map.insert("SettingsWorkEnd", "Work end");
        map.insert("SettingsRunAtStartup", "Run at system startup");
        map.insert("SettingsStartMinimized", "Start minimized");
        map.insert("SettingsCloseToTray", "Close dashboard to tray");
        map.insert("SettingsGlassClarity", "Glass clarity");
        map.insert("SettingsReducedMotion", "Reduced motion");
        map.insert("SettingsTheme", "Theme");
        map.insert("SettingsSave", "Save");
        map.insert("SettingsClose", "Close");
        map.insert("SettingsCaptureScreenshot", "Capture screenshot for overlay background");
        map.insert("ValidationEyeResetIntervalInvalid", "Eye reset interval must be between 1 and 480 minutes.");
        map.insert("ValidationEyeResetDurationInvalid", "Eye reset duration must be between 5 and 600 seconds.");
        map.insert("ValidationMoveBreakIntervalInvalid", "Move break interval must be between 1 and 480 minutes.");
        map.insert("ValidationMoveBreakDurationInvalid", "Move break duration must be between 5 and 600 seconds.");
        map.insert("ValidationSnoozeDurationInvalid", "Snooze duration must be between 1 and 60 minutes.");
        map.insert("ValidationIdleThresholdInvalid", "Idle threshold must be between 1 and 120 minutes.");
        map.insert("ValidationGlassClarityInvalid", "Glass clarity must be between 0 and 150.");
        map.insert("ValidationWorkHoursRangeInvalid", "Work start and end cannot be the same time.");
        Self { map }
    }

    pub fn get(&self, key: &str) -> String {
        self.map.get(key).copied().unwrap_or(key).to_string()
    }
}