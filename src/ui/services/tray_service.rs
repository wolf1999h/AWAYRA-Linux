use std::time::Duration;

use gtk4::{Application, ApplicationWindow};
use gtk4::prelude::*;
use gtk4::glib;
use gtk4::glib::WeakRef;
use tray_icon::{TrayIcon, TrayIconBuilder, Icon, menu::{Menu, MenuItem}};

pub struct TrayService {
    _tray: TrayIcon,
}

impl TrayService {
    pub fn run(dashboard: WeakRef<ApplicationWindow>, app: WeakRef<Application>) -> Self {
        let icon = Icon::from_rgba(vec![255u8, 0, 160, 255], 1, 1)
            .expect("valid rgba icon");
        let menu = Menu::new();

        let show = MenuItem::new("Show Dashboard", true, None);
        let quit = MenuItem::new("Quit", true, None);

        let show_id = show.id().clone();
        let quit_id = quit.id().clone();

        menu.append(&show);
        menu.append(&quit);

        let tray = TrayIconBuilder::new()
            .with_icon(icon)
            .with_title("Awayra")
            .with_menu(Box::new(menu))
            .build()
            .expect("tray icon");

        let _ = tray;

        // Use GTK's thread-local timer to listen for menu events on the main thread
        gtk4::glib::timeout_add_local(Duration::from_millis(200), move || {
            let receiver = tray_icon::menu::MenuEvent::receiver();
            while let Ok(event) = receiver.try_recv() {
                if event.id == show_id {
                    if let Some(d) = dashboard.upgrade() {
                        d.show();
                        d.present();
                    }
                } else if event.id == quit_id {
                    if let Some(a) = app.upgrade() {
                        a.quit();
                    }
                }
            }
            gtk4::glib::ControlFlow::Continue
        });

        Self { _tray: tray }
    }
}