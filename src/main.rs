#![allow(dead_code)]

mod core;
mod ui;

use gtk4::prelude::*;
use gtk4::Application;
use std::sync::{Arc, Mutex};

fn main() {
    env_logger::init();

    let app = Application::new(Some("com.awayra.Awayra"), Default::default());

    let host = Arc::new(Mutex::new(ui::services::app_host::AppHost::new()));

    // Initialize host
    {
        let rt = tokio::runtime::Runtime::new().expect("Tokio runtime");
        let mut host_ref = host.lock().unwrap();
        rt.block_on(host_ref.initialize());
    }

    app.connect_activate(move |app| {
        let host_clone = host.clone();

        // Create dashboard
        let dashboard = ui::views::dashboard::DashboardWindow::new(app, host_clone.clone());
        dashboard.show();

        // Create overlay
        let overlay = Arc::new(Mutex::new(ui::views::overlay::OverlayWindow::new(host_clone.clone())));
        let overlay_clone = overlay.clone();

        // Listen for break events
        gtk4::glib::timeout_add_seconds_local(1, move || {
            if let Ok(host_lock) = host_clone.lock() {
                if let Ok(sched) = host_lock.scheduler.lock() {
                    let snapshot = sched.get_snapshot();
                    if let Some(break_type) = snapshot.active_break {
                        if let Ok(mut ov) = overlay_clone.lock() {
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
                    } else if let Ok(ov) = overlay_clone.lock() {
                        if ov.is_visible() {
                            ov.close();
                        }
                    }
                }
            }
            gtk4::glib::ControlFlow::Continue
        });
    });

    app.run();
}