#![allow(dead_code)]

mod core;
mod ui;

use gtk4::prelude::*;
use gtk4::glib;

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

    let host = std::sync::Arc::new(std::sync::Mutex::new(ui::services::app_host::AppHost::new(Some(event_tx.clone()))));

    // Initialize async host runtime
    {
        let host_ref = host.clone();
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async move {
            let mut h = host_ref.lock().unwrap();
            h.initialize().await;
        });
    }

    let overlay = std::sync::Arc::new(std::sync::Mutex::new(None::<ui::views::overlay::OverlayWindow>));
    let event_rx = std::sync::Arc::new(std::sync::Mutex::new(event_rx));
    let app = gtk4::Application::new(Some("com.awayra.Awayra"), Default::default());

    // 4. Connect Activate (UI instantiation strictly inside here)
    app.connect_activate(move |app| {
        let host_clone = host.clone();

        // Create dashboard
        let dashboard = ui::views::dashboard::DashboardWindow::new(app, host_clone.clone());
        dashboard.show();

        // Create overlay
        let overlay_win = ui::views::overlay::OverlayWindow::new(host_clone.clone());
        *overlay.lock().unwrap() = Some(overlay_win);

        // Register UI polling timers
        let rx_timer = event_rx.clone();
        let overlay_timer = overlay.clone();
        let _ = gtk4::glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
            if let Ok(rx_lock) = rx_timer.lock() {
                while let Ok(event) = rx_lock.try_recv() {
                    match event {
                        crate::core::models::SchedulerEvent::TriggerBreak { break_type, duration_seconds, activity_index } => {
                            if let Ok(mut ov_lock) = overlay_timer.lock() {
                                if let Some(overlay_window) = ov_lock.as_mut() {
                                    let args = crate::core::models::BreakStartedEventArgs { break_type, duration_seconds, activity_index };
                                    overlay_window.show_break(args);
                                }
                            }
                        }
                        crate::core::models::SchedulerEvent::BreakEnded { break_type: _, completed: _, skipped: _, snoozed: _ } => {
                            if let Ok(ov_lock) = overlay_timer.lock() {
                                if let Some(overlay_window) = ov_lock.as_ref() {
                                    overlay_window.close();
                                }
                            }
                        }
                    }
                }
            }
            gtk4::glib::ControlFlow::Continue
        });

        let overlay_clone2 = overlay.clone();
        gtk4::glib::timeout_add_seconds_local(1, move || {
            // Scheduler snapshot polling logic
            if let Ok(host_lock) = host_clone.lock() {
                if let Ok(sched) = host_lock.scheduler.lock() {
                    let snapshot = sched.get_snapshot();
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
                            }
                        }
                    } else if let Ok(ov_lock) = overlay_clone2.lock() {
                        if let Some(ov) = ov_lock.as_ref() {
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

    // 5. Run application
    app.run()
}
