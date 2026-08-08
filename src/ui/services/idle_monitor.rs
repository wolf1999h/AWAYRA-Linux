use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use x11rb::connection::Connection;
use x11rb::protocol::screensaver;

/// IdleMonitor detects user idle time on Linux.
/// On Wayland, it uses D-Bus (Mutter IdleMonitor / org.gnome.Mutter.IdleMonitor/Core GetIdletime,
/// falling back to org.freedesktop.ScreenSaver / GetSessionIdleTime).
/// On X11, it uses x11rb XScreenSaver QueryInfo (ms_since_user_input).
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
    pub fn get_idle_seconds(&self) -> Result<u64, String> {
        if self.simulation_active.load(Ordering::Relaxed) {
            if self.simulated_idle.load(Ordering::Relaxed) {
                return Ok(9999);
            }
            return Ok(0);
        }

        // Try Wayland / D-Bus first
        if let Ok(secs) = get_wayland_idle_seconds() {
            if secs > 0 {
                return Ok(secs);
            }
        }

        // Fallback or primary X11 check
        if let Ok(secs) = get_x11_idle_seconds() {
            return Ok(secs);
        }

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

/// Query idle time on Wayland via D-Bus org.gnome.Mutter.IdleMonitor or org.freedesktop.ScreenSaver
fn get_wayland_idle_seconds() -> Result<u64, String> {
    // Try Mutter IdleMonitor first (returns idle microseconds)
    if let Ok(output) = std::process::Command::new("gdbus")
        .args([
            "call", "--session",
            "--dest", "org.gnome.Mutter.IdleMonitor",
            "--object-path", "/org/gnome/Mutter/IdleMonitor/Core",
            "--method", "org.gnome.Mutter.IdleMonitor.GetIdletime",
        ])
        .output()
    {
        if output.status.success() {
            let out_str = String::from_utf8_lossy(&output.stdout);
            // Example stdout: (uint64 123456789,)
            if let Some(us_str) = extract_digit_token(&out_str) {
                if let Ok(us) = us_str.parse::<u64>() {
                    return Ok(us / 1_000_000);
                }
            }
        }
    }

    // Fallback: KDE / Freedesktop OrgKdeKScreenServer or org.freedesktop.ScreenSaver
    if let Ok(output) = std::process::Command::new("gdbus")
        .args([
            "call", "--session",
            "--dest", "org.freedesktop.ScreenSaver",
            "--object-path", "/org/freedesktop/ScreenSaver",
            "--method", "org.freedesktop.ScreenSaver.GetSessionIdleTime",
        ])
        .output()
    {
        if output.status.success() {
            let out_str = String::from_utf8_lossy(&output.stdout);
            if let Some(ms_str) = extract_digit_token(&out_str) {
                if let Ok(ms) = ms_str.parse::<u64>() {
                    return Ok(ms / 1000);
                }
            }
        }
    }

    Err("Wayland idle monitor DBus query unavailable".to_string())
}

/// Query idle time on X11 via x11rb XScreenSaver QueryInfo
fn get_x11_idle_seconds() -> Result<u64, String> {
    let (conn, screen_num) = x11rb::connect(None).map_err(|e| e.to_string())?;
    let screen = &conn.setup().roots[screen_num];

    let reply = screensaver::query_info(&conn, screen.root)
        .map_err(|e| e.to_string())?
        .reply()
        .map_err(|e| e.to_string())?;

    let ms = reply.ms_since_user_input;
    Ok((ms / 1000) as u64)
}

fn extract_digit_token(s: &str) -> Option<String> {
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        Some(digits)
    }
}
