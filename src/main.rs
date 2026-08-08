mod core;
mod ui;

use gtk4::prelude::*;
use gtk4::glib;

static RUNTIME: std::sync::OnceLock<tokio::runtime::Runtime> = std::sync::OnceLock::new();

fn main() -> glib::ExitCode {
    // 1. Initialize logger
    env_logger::init();

    // 2. EXPLICITLY initialize GTK4 before touching any library, channel, or service!
    if let Err(err) = gtk4::init() {
        eprintln!("CRITICAL: Failed to initialize GTK4: {}. Please check if your X11/Wayland DISPLAY environment variable is set and graphical desktop is accessible.", err);
        std::process::exit(1);
    }

    // 3. Create channels and services ONLY AFTER gtk4::init() succeeds
    let (event_tx, event_rx) = std::sync::mpsc::channel::<crate::core::models::SchedulerEvent>();
    let (tray_tx, tray_rx) = std::sync::mpsc::channel::<ui::services::tray_service::TrayCommand>();
    let tray_rx = std::sync::Arc::new(std::sync::Mutex::new(tray_rx));

    let host = std::sync::Arc::new(std::sync::Mutex::new(ui::services::app_host::AppHost::new(Some(event_tx.clone()))));

    // Initialize async host runtime and keep it alive for app lifetime
    let runtime = RUNTIME.get_or_init(|| tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime"));
    {
        let host_ref = host.clone();
        runtime.block_on(async move {
            let mut h = host_ref.lock().unwrap();
            h.initialize().await;
        });
    }

    // Spawn system tray using ksni (D-Bus, GTK3-free)
    let tray_service = ui::services::tray_service::TrayService::new(Some(tray_tx.clone()));
    let tray_status = tray_service.status();
    std::thread::spawn(move || {
        let handle = ksni::TrayService::new(tray_service).spawn();
        std::thread::park();
        let _ = handle;
    });

    let overlay = std::sync::Arc::new(std::sync::Mutex::new(None::<ui::views::overlay::OverlayWindow>));
    let event_rx = std::sync::Arc::new(std::sync::Mutex::new(event_rx));
    let app = gtk4::Application::new(Some("com.awayra.Awayra"), Default::default());

    let host_for_shutdown = host.clone();

    // 4. Connect Activate (UI instantiation strictly inside here)
    app.connect_activate(move |app| {
        let host_clone = host.clone();
        let app_for_tray = app.clone();

        // Keep the application alive while all windows are hidden (system tray
        // background mode). Without hold(), GTK quits when the last window hides,
        // which made the app exit whenever the break overlay was closed.
        // The guard must stay alive for the process lifetime.
        std::mem::forget(app.hold());

        // Enable system-wide dark theme
        if let Some(gtk_settings) = gtk4::Settings::default() {
            gtk_settings.set_gtk_application_prefer_dark_theme(true);
        }

        // Load custom CSS from resources/style.css embedded directly into the binary
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(include_str!("../resources/style.css"));
        if let Some(display) = gtk4::gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_USER,
            );
        }

        // Create dashboard
        let dashboard = ui::views::dashboard::DashboardWindow::new(app, host_clone.clone());
        let start_minimized = {
            let host_ref = host_clone.lock().unwrap();
            let settings = host_ref.settings.lock().unwrap().clone();
            settings.start_minimized
        };
        if !start_minimized {
            dashboard.show();
        }

        // Poll tray commands in GLib loop
        let dashboard_win = dashboard.window.clone();
        let host_for_tray_cmd = host_clone.clone();
        let tray_rx_clone = tray_rx.clone();
        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            if let Ok(rx) = tray_rx_clone.lock() {
                while let Ok(cmd) = rx.try_recv() {
                    match cmd {
                        ui::services::tray_service::TrayCommand::ShowDashboard => {
                            dashboard_win.present();
                        }
                        ui::services::tray_service::TrayCommand::PauseFor { minutes } => {
                            if let Ok(host_ref) = host_for_tray_cmd.lock() {
                                if let Ok(mut sched) = host_ref.scheduler.lock() {
                                    sched.pause_for_minutes(minutes);
                                }
                            }
                        }
                        ui::services::tray_service::TrayCommand::PauseUntilTomorrow => {
                            if let Ok(host_ref) = host_for_tray_cmd.lock() {
                                if let Ok(mut sched) = host_ref.scheduler.lock() {
                                    sched.pause_until_tomorrow();
                                }
                            }
                        }
                        ui::services::tray_service::TrayCommand::TogglePause => {
                            if let Ok(host_ref) = host_for_tray_cmd.lock() {
                                if let Ok(mut sched) = host_ref.scheduler.lock() {
                                    let snapshot = sched.get_snapshot();
                                    if snapshot.is_paused_manual {
                                        sched.resume();
                                    } else {
                                        sched.pause();
                                    }
                                }
                            }
                        }
                        ui::services::tray_service::TrayCommand::EyeNow => {
                            if let Ok(host_ref) = host_for_tray_cmd.lock() {
                                if let Ok(mut sched) = host_ref.scheduler.lock() {
                                    sched.trigger_now(crate::core::models::BreakType::Eye);
                                }
                            }
                        }
                        ui::services::tray_service::TrayCommand::MoveNow => {
                            if let Ok(host_ref) = host_for_tray_cmd.lock() {
                                if let Ok(mut sched) = host_ref.scheduler.lock() {
                                    sched.trigger_now(crate::core::models::BreakType::Move);
                                }
                            }
                        }
                        ui::services::tray_service::TrayCommand::ShowSettings => {
                            let settings_win = ui::views::settings::SettingsWindow::new(
                                host_for_tray_cmd.clone(),
                            );
                            settings_win.show();
                        }
                        ui::services::tray_service::TrayCommand::Quit => {
                            // Graceful quit: persist state & statistics, then exit.
                            // The app 'shutdown' signal handler does the actual persistence,
                            // so we simply request the application to quit.
                            dashboard_win.close();
                            app_for_tray.quit();
                        }
                    }
                }
            }
            gtk4::glib::ControlFlow::Continue
        });

        // Create overlay
        let overlay_win = ui::views::overlay::OverlayWindow::new(app, host_clone.clone());
        *overlay.lock().unwrap() = Some(overlay_win);

        // Register UI polling timers
        let rx_timer = event_rx.clone();
        let overlay_timer = overlay.clone();
        let host_for_events = host_clone.clone();
        let _ = gtk4::glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
            if let Ok(rx_lock) = rx_timer.lock() {
                while let Ok(event) = rx_lock.try_recv() {
                    match event {
                        crate::core::models::SchedulerEvent::TriggerBreak { break_type, duration_seconds, activity_index } => {
                            // Play break start sound
                            if let Ok(host_ref) = host_for_events.lock() {
                                let settings = host_ref.settings.lock().unwrap().clone();
                                host_ref.audio_service.play_break_start(break_type, &settings);
                            }
                            if let Ok(mut ov_lock) = overlay_timer.lock() {
                                if let Some(overlay_window) = ov_lock.as_mut() {
                                    let args = crate::core::models::BreakStartedEventArgs { break_type, duration_seconds, activity_index };
                                    overlay_window.show_break(args);
                                }
                            }
                        }
                        crate::core::models::SchedulerEvent::BreakEnded { break_type, completed, skipped, snoozed } => {
                            if let Ok(mut ov_lock) = overlay_timer.lock() {
                                if let Some(overlay_window) = ov_lock.as_mut() {
                                    overlay_window.close();
                                }
                            }
                            // Stop repeating break sound & play completion chime if completed
                            if let Ok(host_ref) = host_for_events.lock() {
                                host_ref.audio_service.stop_repeating();
                                if completed {
                                    let settings = host_ref.settings.lock().unwrap().clone();
                                    host_ref.audio_service.play_break_end(&settings);
                                }
                            }
                            // Record statistics
                            if let Ok(host_ref) = host_for_events.lock() {
                                if let Ok(mut stats) = host_ref.statistics.lock() {
                                    if completed {
                                        stats.record_completion(break_type);
                                    } else if skipped {
                                        stats.record_skip();
                                    } else if snoozed {
                                        stats.record_snooze();
                                    }
                                }
                            }
                            // Persist state and statistics after every break
                            if let Ok(host_ref) = host_for_events.lock() {
                                let _ = host_ref.persist_all();
                            }
                        }
                    }
                }
            }
            gtk4::glib::ControlFlow::Continue
        });

        let overlay_clone2 = overlay.clone();
        let tray_status_clone = tray_status.clone();
        gtk4::glib::timeout_add_seconds_local(1, move || {
            // Scheduler snapshot polling logic & live countdown update
            if let Ok(host_lock) = host_clone.lock() {
                if let Ok(sched) = host_lock.scheduler.lock() {
                    let snapshot = sched.get_snapshot();

                    // Update tray status
                    let is_paused = snapshot.is_paused_manual || sched.is_configuration_paused();
                    let status_text = if is_paused {
                        "Awayra (Paused)".to_string()
                    } else {
                        "Awayra".to_string()
                    };
                    let hint = if let Some(break_type) = snapshot.active_break {
                        format!("{:?} break active", break_type)
                    } else if is_paused {
                        "Reminders paused".to_string()
                    } else {
                        let eye_secs = snapshot.eye_remaining.num_seconds().max(0);
                        let move_secs = snapshot.move_remaining.num_seconds().max(0);
                        let next_secs = if snapshot.eye_enabled && snapshot.move_enabled {
                            eye_secs.min(move_secs)
                        } else if snapshot.eye_enabled {
                            eye_secs
                        } else if snapshot.move_enabled {
                            move_secs
                        } else {
                            0
                        };
                        let m = next_secs / 60;
                        let s = next_secs % 60;
                        format!("Next break in {:02}:{:02}", m, s)
                    };
                    tray_status_clone.update(is_paused, status_text, hint);

                    if let Some(break_type) = snapshot.active_break {
                        if let Ok(mut ov_lock) = overlay_clone2.lock() {
                            if let Some(ov) = ov_lock.as_mut() {
                                if !ov.is_visible() {
                                    let settings = sched.settings();
                                    let args = crate::core::models::BreakStartedEventArgs {
                                        break_type,
                                        duration_seconds: match break_type {
                                            crate::core::models::BreakType::Eye => settings.eye_reset_duration_seconds,
                                            crate::core::models::BreakType::Move => settings.move_break_duration_seconds,
                                        },
                                        activity_index: sched.move_activity_index(),
                                    };
                                    ov.show_break(args);
                                }
                                if let Some(rem) = snapshot.active_break_remaining {
                                    ov.update_remaining(rem.num_seconds());
                                }
                            }
                        }
                    } else if let Ok(mut ov_lock) = overlay_clone2.lock() {
                        if let Some(ov) = ov_lock.as_mut() {
                            if ov.is_visible() {
                                ov.close();
                            }
                        }
                    }
                }
            }
            gtk4::glib::ControlFlow::Continue
        });
    });

    // 5. Persist state on graceful shutdown (tray Quit, window close via app quit)
    app.connect_shutdown(move |_app| {
        if let Ok(mut host_ref) = host_for_shutdown.lock() {
            host_ref.shutdown();
            let _ = host_ref.persist_all();
        }
        log::info!("Awayra shutdown complete");
    });

    // 6. Run application
    app.run()
}
