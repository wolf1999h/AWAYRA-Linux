/// ScreenshotService captures the screen for the overlay background.
/// On Wayland, it uses xdg-desktop-portal via ashpd.
/// On X11, it uses XGetImage via x11rb (no prompt needed).
///
/// If capture fails or user denies permission on Wayland,
/// we return None and the overlay shows a solid dark background.
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{ImageFormat, get_image};

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

    /// Capture a screenshot of the current screen/monitors.
    /// Returns the image data as RGBA bytes, or None if unavailable/permission denied.
    pub fn capture(&self) -> Option<ScreenshotResult> {
        if !self.is_enabled() {
            return None;
        }

        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // 1. Fast CLI tools (grim, gdbus GNOME D-Bus, spectacle, maim, scrot, import, gnome-screenshot)
            if let Some(result) = capture_via_cli() {
                return Some(result);
            }

            let is_wayland = std::env::var("XDG_SESSION_TYPE").unwrap_or_default() == "wayland";

            // 2. X11 capture (x11rb). On Wayland sessions this must be skipped:
            //    the XWayland root window is empty/black, so it would return a
            //    blank "screenshot" instead of the real desktop.
            if !is_wayland {
                if let Ok(Some(result)) = capture_via_x11() {
                    return Some(result);
                }
            }

            // 3. XDG Desktop Portal (ashpd) - last resort, works on any desktop
            if let Ok(Some(result)) = capture_via_portal() {
                return Some(result);
            }

            None
        }))
        .ok()
        .flatten()
    }
}

#[derive(Debug, Clone)]
pub struct ScreenshotResult {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>, // RGBA pixels
}

/// Capture via fast CLI tools if available on user system
fn capture_via_cli() -> Option<ScreenshotResult> {
    let tmp_path = std::env::temp_dir().join("awayra_screenshot.png");
    let _ = std::fs::remove_file(&tmp_path);

    let tmp_str = tmp_path.to_str()?;

    let commands: &[(&str, Vec<&str>)] = &[
        ("grim", vec![tmp_str]),
        ("gdbus", vec![
            "call", "--session",
            "--dest", "org.gnome.Shell.Screenshot",
            "--object-path", "/org/gnome/Shell/Screenshot",
            "--method", "org.gnome.Shell.Screenshot.Screenshot",
            "false", "false", tmp_str,
        ]),
        ("maim", vec![tmp_str]),
        ("scrot", vec!["-z", tmp_str]),
        ("import", vec!["-window", "root", tmp_str]),
        ("gnome-screenshot", vec!["-f", tmp_str]),
    ];

    for (cmd, args) in commands {
        let res = std::panic::catch_unwind(|| {
            std::process::Command::new(cmd)
                .args(args)
                .status()
                .ok()
        });

        if let Ok(Some(status)) = res {
            if status.success() && tmp_path.exists() {
                if let Ok(img) = image::open(&tmp_path) {
                    let rgba = img.to_rgba8();
                    let (w, h) = rgba.dimensions();
                    let _ = std::fs::remove_file(&tmp_path);
                    return Some(ScreenshotResult {
                        width: w as i32,
                        height: h as i32,
                        data: rgba.into_raw(),
                    });
                }
            }
        }
    }
    let _ = std::fs::remove_file(&tmp_path);
    None
}

/// Capture via xdg-desktop-portal (Wayland) using ashpd with timeout and panic guard
fn capture_via_portal() -> Result<Option<ScreenshotResult>, String> {
    let result = std::panic::catch_unwind(|| {
        let handle = std::thread::spawn(|| {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .ok()?;
            rt.block_on(async {
                use ashpd::desktop::screenshot::Screenshot;
                let req = Screenshot::request().interactive(false);
                let response = tokio::time::timeout(
                    std::time::Duration::from_secs(2),
                    req.send(),
                )
                .await
                .ok()?
                .ok()?
                .response()
                .ok()?;

                let uri = response.uri();
                let uri_str = uri.as_str();
                let path_str = uri_str.strip_prefix("file://").unwrap_or(uri_str);
                let img = image::open(path_str).ok()?.to_rgba8();
                let (w, h) = img.dimensions();
                Some(ScreenshotResult {
                    width: w as i32,
                    height: h as i32,
                    data: img.into_raw(),
                })
            })
        });
        handle.join().ok().flatten()
    });

    match result {
        Ok(res) => Ok(res),
        Err(_) => Err("Portal panicked".to_string()),
    }
}

/// Capture via X11 XGetImage
fn capture_via_x11() -> Result<Option<ScreenshotResult>, String> {
    let (conn, screen_num) = x11rb::connect(None).map_err(|e| e.to_string())?;
    let screen = &conn.setup().roots[screen_num];
    let width = screen.width_in_pixels as usize;
    let height = screen.height_in_pixels as usize;

    let reply = get_image(&conn, ImageFormat::Z_PIXMAP, screen.root, 0, 0, width as u16, height as u16, u32::MAX)
        .map_err(|e| e.to_string())?
        .reply()
        .map_err(|e| e.to_string())?;

    let raw = reply.data;
    let expected_pixels = width * height;
    let expected_bytes_4 = expected_pixels * 4;
    let expected_bytes_3 = expected_pixels * 3;

    let mut rgba = Vec::with_capacity(expected_bytes_4);

    if raw.len() >= expected_bytes_4 {
        for chunk in raw.chunks_exact(4) {
            if rgba.len() >= expected_bytes_4 {
                break;
            }
            let b = chunk[0];
            let g = chunk[1];
            let r = chunk[2];
            rgba.extend_from_slice(&[r, g, b, 255u8]);
        }
    } else if raw.len() >= expected_bytes_3 {
        for chunk in raw.chunks_exact(3) {
            if rgba.len() >= expected_bytes_4 {
                break;
            }
            let b = chunk[0];
            let g = chunk[1];
            let r = chunk[2];
            rgba.extend_from_slice(&[r, g, b, 255u8]);
        }
    } else {
        return Ok(None);
    }

    if rgba.len() != expected_bytes_4 {
        return Ok(None);
    }

    Ok(Some(ScreenshotResult {
        width: width as i32,
        height: height as i32,
        data: rgba,
    }))
}
