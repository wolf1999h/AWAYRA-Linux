use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use ksni::ToolTip;
use crate::core::localization::LocalizationService;

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
    status: Arc<TrayStatus>,
    loc: Arc<LocalizationService>,
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
    pub fn new(cmd_tx: Option<Sender<TrayCommand>>, loc: Arc<LocalizationService>) -> Self {
        Self {
            enabled: Arc::new(AtomicBool::new(true)),
            cmd_tx,
            status: Arc::new(TrayStatus::default()),
            loc,
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

fn load_tray_icon() -> Vec<ksni::Icon> {
    let bytes = include_bytes!("../../../resources/icons/awayra.png");
    if let Ok(img) = image::load_from_memory(bytes) {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let mut argb = Vec::with_capacity((width * height * 4) as usize);

        for pixel in rgba.pixels() {
            let [r, g, b, a] = pixel.0;
            argb.push(a);
            argb.push(r);
            argb.push(g);
            argb.push(b);
        }

        vec![ksni::Icon {
            width: width as i32,
            height: height as i32,
            data: argb,
        }]
    } else {
        vec![]
    }
}

impl ksni::Tray for TrayService {
    fn id(&self) -> String {
        "com.awayra.Awayra".to_string()
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        self.send(TrayCommand::ShowDashboard);
    }

    fn icon_name(&self) -> String {
        String::new()
    }

    fn icon_pixmap(&self) -> Vec<ksni::Icon> {
        load_tray_icon()
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
        let loc = &self.loc;

        vec![
            StandardItem {
                label: loc.get("OpenAwayra"),
                icon_name: "application-x-executable-symbolic".to_string(),
                activate: Box::new(|service: &mut Self| {
                    service.send(TrayCommand::ShowDashboard);
                }),
                ..Default::default()
            }.into(),
            StandardItem {
                label: loc.get("EyeResetNow"),
                icon_name: "eye-open-negative-filled-symbolic".to_string(),
                activate: Box::new(|service: &mut Self| {
                    service.send(TrayCommand::EyeNow);
                }),
                ..Default::default()
            }.into(),
            StandardItem {
                label: loc.get("MoveBreakNow"),
                icon_name: "media-playback-start-symbolic".to_string(),
                activate: Box::new(|service: &mut Self| {
                    service.send(TrayCommand::MoveNow);
                }),
                ..Default::default()
            }.into(),
            StandardItem {
                label: if paused { loc.get("TrayResumeReminders") } else { loc.get("TrayPauseReminders") },
                icon_name: if paused { "media-playback-start-symbolic".to_string() } else { "media-playback-pause-symbolic".to_string() },
                activate: Box::new(|service: &mut Self| {
                    service.send(TrayCommand::TogglePause);
                }),
                ..Default::default()
            }.into(),
            SubMenu {
                label: loc.get("TrayPauseFor"),
                icon_name: "alarm-symbolic".to_string(),
                submenu: vec![
                    StandardItem {
                        label: loc.get("TrayPause30m"),
                        icon_name: "alarm-symbolic".to_string(),
                        activate: Box::new(|service: &mut Self| {
                            service.send(TrayCommand::PauseFor { minutes: 30 });
                        }),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: loc.get("TrayPause1h"),
                        icon_name: "alarm-symbolic".to_string(),
                        activate: Box::new(|service: &mut Self| {
                            service.send(TrayCommand::PauseFor { minutes: 60 });
                        }),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: loc.get("TrayPauseTomorrow"),
                        icon_name: "alarm-symbolic".to_string(),
                        activate: Box::new(|service: &mut Self| {
                            service.send(TrayCommand::PauseUntilTomorrow);
                        }),
                        ..Default::default()
                    }.into(),
                ],
                ..Default::default()
            }.into(),
            StandardItem {
                label: loc.get("Settings"),
                icon_name: "emblem-system-symbolic".to_string(),
                activate: Box::new(|service: &mut Self| {
                    service.send(TrayCommand::ShowSettings);
                }),
                ..Default::default()
            }.into(),
            MenuItem::Separator,
            StandardItem {
                label: loc.get("Quit"),
                icon_name: "application-exit-symbolic".to_string(),
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
