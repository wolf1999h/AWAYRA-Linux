use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// TrayService manages the system tray icon using ksni (KDE Status Notifier Item).
/// This uses D-Bus natively and avoids GTK3 dependencies.
pub struct TrayService {
    enabled: Arc<AtomicBool>,
}

impl TrayService {
    pub fn new() -> Self {
        Self {
            enabled: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }
}

impl ksni::Tray for TrayService {
    fn icon_name(&self) -> String {
        "awayra".to_string()
    }

    fn title(&self) -> String {
        "Awayra".to_string()
    }

    fn menu(&self) -> Vec<ksni::Item> {
        vec![
            ksni::Item::Standard {
                label: Some("Show Dashboard".to_string()),
                icon_name: Some("awayra".to_string()),
                activate: Box::new(|_| {
                    // Show dashboard window
                    // This is handled via the host
                }),
                ..Default::default()
            },
            ksni::Item::Standard {
                label: Some("Quit".to_string()),
                icon_name: Some("application-exit".to_string()),
                activate: Box::new(|_| {
                    std::process::exit(0);
                }),
                ..Default::default()
            },
        ]
    }
}

impl Drop for TrayService {
    fn drop(&mut self) {
        log::info!("Tray service shutting down");
    }
}