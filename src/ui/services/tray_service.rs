use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use ksni::ToolTip;

/// Action commands from Tray to GLib main thread
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayCommand {
    ShowDashboard,
    EyeNow,
    MoveNow,
    TogglePause,
    PauseFor { minutes: i64 },
    PauseUntilTomorrow,
    ShowSettings,
    Quit,
}

pub struct TrayService {
    enabled: Arc<AtomicBool>,
    cmd_tx: Option<Sender<TrayCommand>>,
    // Shared state enabling the tray menu/tooltip to reflect scheduler status.
    status: Arc<TrayStatus>,
}

/// Shared, pollable status the GLib main loop refreshes for the tray.
#[derive(Debug, Default)]
pub struct TrayStatus {
    pub paused: std::sync::atomic::AtomicBool,
    pub status_text: std::sync::Mutex<String>,
    /// "Next break in mm:ss" hint shown in the tooltip (empty when unknown).
    pub tooltip_hint: std::sync::Mutex<String>,
}

impl TrayStatus {
    pub fn tool_break_hint(&self) -> String {
        if let Ok(guard) = self.tooltip_hint.lock() {
            guard.clone()
        } else {
            String::new()
        }
    }

    /// Refresh paused/status/hint from a scheduler snapshot. Cheap; called
    /// once per second from the GLib main loop.
    pub fn update(
        &self,
        paused: bool,
        status_text: String,
        next_break_hint: String,
    ) {
        self.paused.store(paused, Ordering::Relaxed);
        if let Ok(mut guard) = self.status_text.lock() {
            *guard = status_text;
        }
        if let Ok(mut guard) = self.tooltip_hint.lock() {
            *guard = next_break_hint;
        }
    }
}

impl TrayService {
    pub fn new(cmd_tx: Option<Sender<TrayCommand>>) -> Self {
        Self {
            enabled: Arc::new(AtomicBool::new(true)),
            cmd_tx,
            status: Arc::new(TrayStatus::default()),
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    pub fn status(&self) -> Arc<TrayStatus> {
        self.status.clone()
    }

    fn send(&self, cmd: TrayCommand) {
        if let Some(tx) = &self.cmd_tx {
            let _ = tx.send(cmd);
        }
    }
}

impl ksni::Tray for TrayService {
    fn icon_name(&self) -> String {
        "awayra".to_string()
    }

    fn title(&self) -> String {
        let status = self.status.status_text.lock()
            .map(|g| g.clone())
            .unwrap_or_default();
        let hint = self.status.tool_break_hint();
        if hint.is_empty() {
            status
        } else {
            format!("{} - {}", status, hint)
        }
    }

    fn tool_tip(&self) -> ToolTip {
        ToolTip {
            title: self.title(),
            ..Default::default()
        }
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        let paused = self.status.paused.load(Ordering::Relaxed);

        vec![
            StandardItem {
                label: "Open Awayra".to_string(),
                icon_name: "awayra".to_string(),
                activate: Box::new(|service: &mut Self| {
                    service.send(TrayCommand::ShowDashboard);
                }),
                ..Default::default()
            }.into(),
            StandardItem {
                label: "Eye Reset Now".to_string(),
                icon_name: "eye-piece".to_string(),
                activate: Box::new(|service: &mut Self| {
                    service.send(TrayCommand::EyeNow);
                }),
                ..Default::default()
            }.into(),
            StandardItem {
                label: "Move Break Now".to_string(),
                icon_name: "walk".to_string(),
                activate: Box::new(|service: &mut Self| {
                    service.send(TrayCommand::MoveNow);
                }),
                ..Default::default()
            }.into(),
            StandardItem {
                label: if paused { "Resume Reminders".into() } else { "Pause Reminders".into() },
                icon_name: "media-playback-pause".to_string(),
                activate: Box::new(|service: &mut Self| {
                    service.send(TrayCommand::TogglePause);
                }),
                ..Default::default()
            }.into(),
            SubMenu {
                label: "Pause for…".to_string(),
                icon_name: "appointment-new".to_string(),
                submenu: vec![
                    StandardItem {
                        label: "Pause for 30 minutes".to_string(),
                        activate: Box::new(|service: &mut Self| {
                            service.send(TrayCommand::PauseFor { minutes: 30 });
                        }),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Pause for 1 hour".to_string(),
                        activate: Box::new(|service: &mut Self| {
                            service.send(TrayCommand::PauseFor { minutes: 60 });
                        }),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Pause until tomorrow".to_string(),
                        activate: Box::new(|service: &mut Self| {
                            service.send(TrayCommand::PauseUntilTomorrow);
                        }),
                        ..Default::default()
                    }.into(),
                ],
                ..Default::default()
            }.into(),
            StandardItem {
                label: "Settings".to_string(),
                icon_name: "preferences-system".to_string(),
                activate: Box::new(|service: &mut Self| {
                    service.send(TrayCommand::ShowSettings);
                }),
                ..Default::default()
            }.into(),
            MenuItem::Separator,
            StandardItem {
                label: "Quit".to_string(),
                icon_name: "application-exit".to_string(),
                activate: Box::new(|service: &mut Self| {
                    service.send(TrayCommand::Quit);
                }),
                ..Default::default()
            }.into(),
        ]
    }
}

impl Drop for TrayService {
    fn drop(&mut self) {
        log::info!("Tray service shutting down");
    }
}