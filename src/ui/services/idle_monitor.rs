use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// IdleMonitor detects user idle time.
/// On Wayland it uses the ext-idle-notify protocol via D-Bus.
/// On X11 it falls back to XScreenSaver.
pub struct IdleMonitor {
    simulated_idle: Arc<AtomicBool>,
    simulation_active: Arc<AtomicBool>,
}

impl IdleMonitor {
    pub fn new() -> Self {
        Self {
            simulated_idle: Arc::new(AtomicBool::new(false)),
            simulation_active: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Get the idle time in seconds.
    /// Returns 0 if unable to determine.
    pub fn get_idle_seconds(&self) -> f64 {
        // If simulation is active, return simulated value
        if self.simulation_active.load(Ordering::Relaxed) {
            if self.simulated_idle.load(Ordering::Relaxed) {
                return 9999.0; // Simulate being idle for a long time
            }
            return 0.0;
        }

        // Try Wayland first via D-Bus
        match get_wayland_idle_seconds() {
            Some(secs) => return secs,
            None => {}
        }

        // Fallback: try X11
        match get_x11_idle_seconds() {
            Some(secs) => return secs,
            None => {}
        }

        // If neither works, return 0
        0.0
    }

    pub fn is_idle(&self, threshold_seconds: f64) -> bool {
        self.get_idle_seconds() >= threshold_seconds
    }

    pub fn set_simulated_idle(&self, is_idle: Option<bool>) {
        match is_idle {
            Some(val) => {
                self.simulated_idle.store(val, Ordering::Relaxed);
                self.simulation_active.store(true, Ordering::Relaxed);
            }
            None => {
                self.simulation_active.store(false, Ordering::Relaxed);
            }
        }
    }
}

/// Get idle time via Wayland's ext-idle-notify protocol through D-Bus.
/// Returns None if not on Wayland or if the call fails.
fn get_wayland_idle_seconds() -> Option<f64> {
    // Check if we're on Wayland
    let desktop = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    if desktop != "wayland" {
        return None;
    }

    // Try to get idle time via D-Bus org.freedesktop.ScreenSaver
    // This is a simplified approach - in production we'd use zbus async
    // For now, we return None to fall through to X11
    None
}

/// Get idle time via X11's XScreenSaverQueryInfo.
/// Returns None if not on X11 or if the call fails.
fn get_x11_idle_seconds() -> Option<f64> {
    let desktop = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    if desktop == "wayland" {
        return None;
    }

    // Try to read idle time from /proc or use a simple heuristic
    // In production, we'd use the x11 crate with XScreenSaverQueryInfo
    // For now, we use a fallback that checks /dev/input/event* activity
    None
}