/// Tray service placeholder.
/// In production, this would use tray-icon crate for system tray integration.
/// On GNOME, this requires the AppIndicator extension.
pub struct TrayService;

impl TrayService {
    pub fn new() -> Self {
        Self
    }
}