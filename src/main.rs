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

    let host = Arc::new(Mutex::new(ui::services::app_host::AppHost::new(Some(event_tx.clone()))));

    // Initialize host
    {
        let rt = tokio::runtime::Runtime::new().expect("Tokio runtime");
        let mut host_ref = host.lock().unwrap();
        rt.block_on(host_ref.initialize());
    }

    // Share overlay between threads so the background event listener can show/hide it
    let overlay = Arc::new(Mutex::new(None::<ui::views::overlay::OverlayWindow>));

    // Spawn background event listener
    {
        let host_ev = host.clone();
        let overlay_ev = overlay.clone();
        std::thread::spawn(move || {
            while let Ok(event) = event_rx.recv() {
                match event {
                    crate::core::models::SchedulerEvent::TriggerBreak { break_type, duration_seconds, activity_index } => {
                        if let Ok(host_lock) = host_ev.lock() {
                            if let Ok(mut ov) = host_lock.scheduler.lock() {
                                // Scheduler already updated state; this is a signal only.
                            }
                        }

                        // Schedule showing the overlay on the GTK main thread
                        let ov = overlay_ev.clone();
                        let args = crate::core::models::BreakStartedEventArgs {
                            break_type,
                            duration_seconds,
                            activity_index,
                        };
                        gtk4::glib::idle_add_local(move || {
                            if let Ok(mut ov_lock) = ov.lock() {
                                if let Some(overlay_window) = ov_lock.as_mut() {
                                    overlay_window.show_break(args);
                                }
                            }
                            gtk4::glib::ControlFlow::Continue
                        });
                    }
                    crate::core::models::SchedulerEvent::BreakEnded { break_type, completed, skipped, snoozed } => {
                        if let Ok(host_lock) = host_ev.lock() {
                            if let Ok(mut ov) = host_lock.scheduler.lock() {
                                // Scheduler already updated state; this is a signal only.
                            }
                        }

                        // Schedule hiding the overlay on the GTK main thread
                        let ov = overlay_ev.clone();
                        gtk4::glib::idle_add_local(move || {
                            if let Ok(ov_lock) = ov.lock() {
                                if let Some(overlay_window) = ov_lock.as_ref() {
                                    overlay_window.close();
                                }
                            }
                            gtk4::glib::ControlFlow::Continue
                        });
                    }
                }
            }
        });
    }

    app.connect_activate(move |app| {
        let host_clone = host.clone();
        let _overlay_event_tx = event_tx.clone();
        let overlay_clone = overlay.clone();

        // Create dashboard
        let dashboard = ui::views::dashboard::DashboardWindow::new(app, host_clone.clone());
        dashboard.show();

        // Create overlay
        let overlay_window = ui::views::overlay::OverlayWindow::new(host_clone.clone());
        *overlay_clone.lock().unwrap() = Some(overlay_window);

        // Listen for break events
        gtk4::glib::timeout_add_seconds_local(1, move || {
            if let Ok(host_lock) = host_clone.lock() {
                if let Ok(sched) = host_lock.scheduler.lock() {
                    let snapshot = sched.get_snapshot();
                    if let Some(break_type) = snapshot.active_break {
                        if let Ok(mut ov_lock) = overlay_clone.lock() {
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
                    } else if let Ok(ov_lock) = overlay_clone.lock() {
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