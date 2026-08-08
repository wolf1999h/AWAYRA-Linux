use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;

pub struct AutostartService;

impl AutostartService {
    fn get_autostart_dir() -> Option<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "awayra", "Awayra")?;
        let config_dir = proj_dirs.config_dir().parent()?; // ~/.config
        Some(config_dir.join("autostart"))
    }

    fn get_desktop_file_path() -> Option<PathBuf> {
        let dir = Self::get_autostart_dir()?;
        Some(dir.join("com.awayra.Awayra.desktop"))
    }

    pub fn set_autostart(enabled: bool) -> Result<(), std::io::Error> {
        let file_path = match Self::get_desktop_file_path() {
            Some(path) => path,
            None => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Config dir not found")),
        };

        if enabled {
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let exec_path = std::env::current_exe()
                .unwrap_or_else(|_| PathBuf::from("/usr/bin/awayra"));

            let content = format!(
                "[Desktop Entry]\n\
                 Type=Application\n\
                 Name=Awayra\n\
                 Comment=A calm break reminder for healthier computer use\n\
                 Exec={}\n\
                 Icon=awayra\n\
                 Terminal=false\n\
                 Categories=Utility;\n\
                 X-GNOME-Autostart-enabled=true\n",
                exec_path.to_string_lossy()
            );

            fs::write(file_path, content)?;
        } else if file_path.exists() {
            fs::remove_file(file_path)?;
        }

        Ok(())
    }

    pub fn is_autostart_enabled() -> bool {
        Self::get_desktop_file_path()
            .map(|p| p.exists())
            .unwrap_or(false)
    }
}
