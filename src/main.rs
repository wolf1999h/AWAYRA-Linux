#![allow(dead_code)]

mod core;
mod ui;

use gtk4::prelude::*;
use gtk4::Application;
use std::sync::{Arc, Mutex};

fn main() {
    env_logger::init();

    let app = Application::new(Some("com.awayra.Awayra"), Default::default());

    let (event_tx, event_rx) = std::sync::mpsc::channel::<crate::core::models::SchedulerEvent>();

    let audio_service = Arc::new(crate::core::services::audio_service::AudioService::new());

    let host = Arc::new(Mutex::new(ui::services::app_host::AppHost::new(Some(event_tx.clone()))));

    // Initialize host
    {
        let rt = tokio::runtime::Runtime::new().expect("Tokio runtime");
        let mut host_ref = host.lock().unwrap();
        rt.block_on(host_ref.initialize());
    }

    // Share overlay globally so the GTK main loop can show/hide it
    let overlay = Arc::new(Mutex::new(None::<ui::views::overlay::OverlayWindow>));

    // Wrap event_rx in Arc<Mutex<...>> to satisfy Fn closure requirements
    let event_rx = Arc::new(Mutex::new(event_rx));
    let event_rx_clone = event_rx.clone();

    app.connect_activate(move |app| {
        let host_clone = host.clone();
        let overlay_clone = overlay.clone();

        // Create dashboard
        let dashboard = ui::views::dashboard::DashboardWindow::new(app, host_clone.clone());
        dashboard.show();

        let dashboard_window = dashboard.window.clone();

        // Create tray service
        let _ = ui::services::tray_service::TrayService::run(
            gtk4::glib::clone::Downgrade::downgrade(&dashboard_window),
            gtk4::glib::clone::Downgrade::downgrade(&app),
        );

        // Create overlay
        let overlay_window = ui::views::overlay::OverlayWindow::new(host_clone.clone());
        *overlay_clone.lock().unwrap() = Some(overlay_window);

        let rx_timer = event_rx_clone.clone();
        let overlay_timer = overlay_clone.clone();
        let _ = gtk4::glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
            if let Ok(rx_lock) = rx_timer.lock() {
                while let Ok(event) = rx_lock.try_recv() {
                    match event {
                        crate::core::models::SchedulerEvent::TriggerBreak { break_type, duration_seconds, activity_index } => {
                            if let Ok(mut ov_lock) = overlay_timer.lock() {
                                if let Some(overlay_window) = ov_lock.as_mut() {
                                    let args = crate::core::models::BreakStartedEventArgs {
                                        break_type,
                                        duration_seconds,
                                        activity_index,
                                    };
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

        // Listen for break events
        let overlay_clone2 = overlay_clone.clone();
        gtk4::glib::timeout_add_seconds_local(1, move || {
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

    app.run();
}