use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
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
    pub audio_service: Arc<crate::core::services::audio_service::AudioService>,
    pub idle_monitor: Arc<IdleMonitor>,
    pub screenshot_service: Arc<ScreenshotService>,

    settings_store: JsonStore<AppSettings>,
    state_store: JsonStore<SchedulerState>,
    statistics_store: JsonStore<StatisticsData>,
    data_dir: PathBuf,
    tick_handle: Option<tokio::task::JoinHandle<()>>,
    idle_handle: Option<tokio::task::JoinHandle<()>>,
    event_tx: std::sync::mpsc::Sender<crate::core::models::SchedulerEvent>,
}

impl AppHost {
    pub fn new(event_tx: Option<std::sync::mpsc::Sender<crate::core::models::SchedulerEvent>>) -> Self {
        let data_dir = Self::get_data_dir();

        let settings_store: JsonStore<AppSettings> = JsonStore::<AppSettings>::new(data_dir.join("settings.json"));
        let state_store: JsonStore<SchedulerState> = JsonStore::<SchedulerState>::new(data_dir.join("state.json"));
        let statistics_store: JsonStore<StatisticsData> = JsonStore::<StatisticsData>::new(data_dir.join("statistics.json"));

        // Load settings with fallback to default
        let settings = settings_store.load()
            .ok()
            .flatten()
            .filter(|s| SettingsValidator::is_valid(s))
            .unwrap_or_else(|| {
                let default = AppSettings::default();
                let _ = settings_store.save(&default);
                default
            });

        // Attempt to save defaults back to disk if file was missing/corrupted
        let _ = settings_store.save(&settings);

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

        let event_tx = event_tx.unwrap_or_else(|| {
            let (tx, _) = std::sync::mpsc::channel();
            tx
        });

        let scheduler = Arc::new(Mutex::new(
            BreakScheduler::new(
                Box::new(SystemClock),
                settings.clone(),
                Some(state),
                event_tx.clone(),
            )
        ));

        let statistics = Arc::new(Mutex::new(
            StatisticsService::new(stats_data)
        ));

        let localization = std::sync::Arc::new(LocalizationService::new());
        let audio_service = std::sync::Arc::new(crate::core::services::audio_service::AudioService::new());
        let idle_monitor = std::sync::Arc::new(IdleMonitor::new());
        let screenshot_service = std::sync::Arc::new(ScreenshotService::new());

        Self {
            scheduler,
            statistics,
            settings: Arc::new(Mutex::new(settings)),
            localization,
            audio_service,
            idle_monitor,
            screenshot_service,
            settings_store,
            state_store,
            statistics_store,
            data_dir,
            tick_handle: None,
            idle_handle: None,
            event_tx,
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

    pub fn enter_configuration_pause(&self) {
        if let Ok(mut sched) = self.scheduler.lock() {
            sched.enter_configuration_pause();
        }
    }

    pub fn exit_configuration_pause(&self) {
        if let Ok(mut sched) = self.scheduler.lock() {
            sched.cancel_configuration_pause();
        }
    }

    pub fn save_configuration(&self, new_settings: AppSettings) -> Result<(), String> {
        if !SettingsValidator::is_valid(&new_settings) {
            return Err("Invalid settings".to_string());
        }

        let save_time = chrono::Utc::now();
        let mut old_settings = self.settings.lock().map_err(|e| e.to_string())?;
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

    /// Copy a user-selected background image into the app data directory.
    /// Returns the stable internal path, or None if the copy failed.
    pub fn store_custom_background(&self, source: &Path) -> Option<PathBuf> {
        let ext = source
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .filter(|e| { let e = e.as_str(); matches!(e, "png"|"jpg"|"jpeg"|"webp"|"bmp"|"svg"|"gif"|"tif"|"tiff"|"avif") })
            .unwrap_or_else(|| "png".to_string());

        let dest = self.data_dir.join(format!("custom_background.{}", ext));
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            std::fs::create_dir_all(&self.data_dir).ok();
            std::fs::copy(source, &dest).ok()
        }));

        if let Ok(Some(_)) = res {
            Some(dest)
        } else {
            None
        }
    }

    pub fn persist_all(&self) -> Result<(), String> {
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