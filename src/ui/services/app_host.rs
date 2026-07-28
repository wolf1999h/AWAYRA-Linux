use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use directories::ProjectDirs;
use tokio::time;

use crate::core::models::{AppSettings, SchedulerState, StatisticsData};
use crate::core::services::break_scheduler::{BreakScheduler, SystemClock};
use crate::core::services::statistics_service::StatisticsService;
use crate::core::services::settings_validator::SettingsValidator;
use crate::core::persistence::json_persistence::JsonStore;
use crate::core::localization::LocalizationService;
use crate::ui::services::idle_monitor::IdleMonitor;
use crate::ui::services::screenshot_service::ScreenshotService;

pub struct AppHost {
    pub scheduler: Arc<Mutex<BreakScheduler>>,
    pub statistics: Arc<Mutex<StatisticsService>>,
    pub settings: Arc<Mutex<AppSettings>>,
    pub localization: Arc<LocalizationService>,
    pub idle_monitor: Arc<IdleMonitor>,
    pub screenshot_service: Arc<ScreenshotService>,

    settings_store: JsonStore<AppSettings>,
    state_store: JsonStore<SchedulerState>,
    statistics_store: JsonStore<StatisticsData>,
    data_dir: PathBuf,
    tick_handle: Option<tokio::task::JoinHandle<()>>,
    idle_handle: Option<tokio::task::JoinHandle<()>>,
}

impl AppHost {
    pub fn new() -> Self {
        let data_dir = Self::get_data_dir();

        let settings_store: JsonStore<AppSettings> = JsonStore::<AppSettings>::new(data_dir.join("settings.json"));
        let state_store: JsonStore<SchedulerState> = JsonStore::<SchedulerState>::new(data_dir.join("state.json"));
        let statistics_store: JsonStore<StatisticsData> = JsonStore::<StatisticsData>::new(data_dir.join("statistics.json"));

        let localization = Arc::new(LocalizationService::new());
        let idle_monitor = Arc::new(IdleMonitor::new());
        let screenshot_service = Arc::new(ScreenshotService::new());

        // Load settings
        let settings = settings_store.load()
            .ok()
            .flatten()
            .filter(|s| SettingsValidator::is_valid(s))
            .unwrap_or_default();

        // Load state
        let state = state_store.load()
            .ok()
            .flatten()
            .unwrap_or_else(|| SchedulerState::create_default(chrono::Utc::now()));

        // Load statistics
        let stats_data = statistics_store.load()
            .ok()
            .flatten()
            .unwrap_or_else(StatisticsData::create_default);

        let scheduler = Arc::new(Mutex::new(
            BreakScheduler::new(
                Box::new(SystemClock),
                settings.clone(),
                Some(state),
            )
        ));

        let statistics = Arc::new(Mutex::new(
            StatisticsService::new(stats_data)
        ));

        Self {
            scheduler,
            statistics,
            settings: Arc::new(Mutex::new(settings)),
            localization,
            idle_monitor,
            screenshot_service,
            settings_store,
            state_store,
            statistics_store,
            data_dir,
            tick_handle: None,
            idle_handle: None,
        }
    }

    fn get_data_dir() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("com", "awayra", "Awayra") {
            proj_dirs.data_local_dir().to_path_buf()
        } else {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            PathBuf::from(home).join(".local").join("share").join("awayra")
        }
    }

    pub async fn initialize(&mut self) {
        // Ensure data directory exists
        std::fs::create_dir_all(&self.data_dir).ok();

        // Start tick timer (1 second interval)
        let scheduler = self.scheduler.clone();
        self.tick_handle = Some(tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(1));
            loop {
                interval.tick().await;
                if let Ok(mut sched) = scheduler.lock() {
                    sched.tick();
                }
            }
        }));

        // Start idle timer (5 second interval)
        let idle_monitor = self.idle_monitor.clone();
        let scheduler_for_idle = self.scheduler.clone();
        let settings_for_idle = self.settings.clone();
        self.idle_handle = Some(tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(5));
            loop {
                interval.tick().await;
                let is_idle = {
                    let settings = settings_for_idle.lock().unwrap();
                    let threshold = settings.idle_threshold_minutes as f64 * 60.0;
                    idle_monitor.is_idle(threshold)
                };
                if let Ok(mut sched) = scheduler_for_idle.lock() {
                    sched.set_idle(is_idle);
                }
            }
        }));

        log::info!("Awayra initialized. Data directory: {}", self.data_dir.display());
    }

    pub fn begin_configuration_session(&self) {
        if let Ok(mut sched) = self.scheduler.lock() {
            sched.enter_configuration_pause();
        }
    }

    pub fn end_configuration_session(&self, saved: bool) {
        if let Ok(mut sched) = self.scheduler.lock() {
            if !saved {
                sched.cancel_configuration_pause();
            }
        }
    }

    pub async fn save_configuration(&self, new_settings: AppSettings) -> Result<(), String> {
        if !SettingsValidator::is_valid(&new_settings) {
            return Err("Invalid settings".to_string());
        }

        let save_time = chrono::Utc::now();
        let mut old_settings = self.settings.lock().map_err(|e| e.to_string())?;
        let _original_capture = old_settings.capture_screenshot;
        *old_settings = new_settings.clone();
        drop(old_settings);

        // Apply to scheduler
        if let Ok(mut sched) = self.scheduler.lock() {
            sched.apply_configuration_save(new_settings.clone(), save_time);
        }

        // Update screenshot service
        self.screenshot_service.set_enabled(new_settings.capture_screenshot);

        // Persist
        self.settings_store.save(&new_settings)?;

        log::info!("Settings saved");
        Ok(())
    }

    pub fn apply_autostart(&self) {
        // Create a .desktop file in ~/.config/autostart/
        if let Ok(settings) = self.settings.lock() {
            if settings.run_at_startup {
                let autostart_dir = PathBuf::from(std::env::var("HOME").unwrap_or_default())
                    .join(".config")
                    .join("autostart");
                std::fs::create_dir_all(&autostart_dir).ok();

                let desktop_content = format!(
                    "[Desktop Entry]\nType=Application\nName=Awayra\nExec={}\nX-GNOME-Autostart-enabled=true\n",
                    std::env::current_exe().unwrap_or_default().display()
                );
                std::fs::write(autostart_dir.join("awayra.desktop"), desktop_content).ok();
            } else {
                let autostart_path = PathBuf::from(std::env::var("HOME").unwrap_or_default())
                    .join(".config")
                    .join("autostart")
                    .join("awayra.desktop");
                std::fs::remove_file(autostart_path).ok();
            }
        }
    }

    pub async fn persist_all(&self) -> Result<(), String> {
        // Persist state
        if let Ok(sched) = self.scheduler.lock() {
            self.state_store.save(sched.state())?;
        }

        // Persist statistics
        if let Ok(stats) = self.statistics.lock() {
            self.statistics_store.save(stats.data())?;
        }

        self.settings_store.save(&self.settings.lock().unwrap().clone())?;

        Ok(())
    }

    pub fn shutdown(&mut self) {
        if let Some(handle) = self.tick_handle.take() {
            handle.abort();
        }
        if let Some(handle) = self.idle_handle.take() {
            handle.abort();
        }
        log::info!("Awayra shutting down");
    }
}