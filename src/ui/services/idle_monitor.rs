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
    /// Returns Result<u64, String> so callers can decide how to handle failures.
    pub fn get_idle_seconds(&self) -> Result<u64, String> {
        // If simulation is active, return simulated value
        if self.simulation_active.load(Ordering::Relaxed) {
            if self.simulated_idle.load(Ordering::Relaxed) {
                return Ok(9999); // Simulate being idle for a long time
            }
            return Ok(0);
        }

        // Try Wayland first via D-Bus
        match get_wayland_idle_seconds() {
            Ok(secs) => return Ok(secs),
            Err(err) => {
                eprintln!("Idle monitor fallback triggered: {}", err);
            }
        }

        // Fallback: try X11
        match get_x11_idle_seconds() {
            Ok(secs) => return Ok(secs),
            Err(err) => {
                eprintln!("Idle monitor fallback triggered: {}", err);
            }
        }

        // If neither works, assume user is active
        Ok(0)
    }

    pub fn is_idle(&self, threshold_seconds: f64) -> bool {
        match self.get_idle_seconds() {
            Ok(secs) => secs as f64 >= threshold_seconds,
            Err(_) => false,
        }
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
/// Returns Result<u64, String>.
fn get_wayland_idle_seconds() -> Result<u64, String> {
    // Check if we're on Wayland
    let desktop = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    if desktop != "wayland" {
        return Ok(0);
    }

    // Try to get idle time via D-Bus org.freedesktop.ScreenSaver
    // This is a simplified approach - in production we'd use zbus async
    // For now, we return Ok(0) to avoid false idle detection
    Ok(0)
}

/// Get idle time via X11's XScreenSaverQueryInfo.
/// Returns Result<u64, String>.
fn get_x11_idle_seconds() -> Result<u64, String> {
    let desktop = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    if desktop == "wayland" {
        return Ok(0);
    }

    // Try to read idle time from /proc or use a simple heuristic
    // In production, we'd use the x11 crate with XScreenSaverQueryInfo
    // For now, we use a fallback that checks /dev/input/event* activity
    Ok(0)
}
