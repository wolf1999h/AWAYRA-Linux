/// ScreenshotService captures the screen for the overlay background.
/// On Wayland, it uses xdg-desktop-portal (which may prompt the user).
/// On X11, it uses XGetImage directly (no prompt needed).
///
/// If capture fails or user denies permission on Wayland,
/// we return None and the overlay shows a solid dark background.
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ScreenshotService {
    enabled: Arc<AtomicBool>,
}

impl ScreenshotService {
    pub fn new() -> Self {
        Self {
            enabled: Arc::new(AtomicBool::new(true)),
        }
    }

    /// Set whether screenshot capture is enabled.
    /// If disabled, capture() always returns None and a solid background is used.
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// Capture a screenshot of the current monitor.
    /// Returns the image data as RGBA bytes, or None if unavailable/permission denied.
    pub fn capture(&self) -> Option<ScreenshotResult> {
        if !self.is_enabled() {
            return None;
        }

        // Try Wayland portal first
        match capture_via_portal() {
            Some(result) => return Some(result),
            None => {}
        }

        // Fallback to X11
        match capture_via_x11() {
            Some(result) => return Some(result),
            None => {}
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct ScreenshotResult {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>, // RGBA pixels
}

/// Capture via xdg-desktop-portal (Wayland).
/// Returns None if on X11 or if portal is unavailable / permission denied.
fn capture_via_portal() -> Option<ScreenshotResult> {
    let desktop = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    if desktop != "wayland" {
        return None;
    }

    // In production, this would use ashpd to request a screenshot via
    // org.freedesktop.portal.Screenshot API.
    // For now, we return None as a placeholder.
    log::warn!("Wayland screenshot via portal not yet implemented");
    None
}

/// Capture via X11 XGetImage.
/// Returns None if on Wayland or if X11 display is unavailable.
fn capture_via_x11() -> Option<ScreenshotResult> {
    let desktop = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    if desktop == "wayland" {
        return None;
    }

    // In production, this would open an X11 display, use XGetImage,
    // and convert the pixel data to RGBA.
    // For now, we return None as a placeholder.
    log::warn!("X11 screenshot not yet implemented");
    None
}