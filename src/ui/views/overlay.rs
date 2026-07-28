use gtk4::prelude::*;
use gtk4::{Window, Label, Button, Box as GtkBox, Orientation, Align};
use std::sync::{Arc, Mutex};

use crate::core::models::{BreakType, BreakStartedEventArgs};
use crate::ui::services::app_host::AppHost;

pub struct OverlayWindow {
    window: Window,
    title_label: Label,
    instruction_label: Label,
    countdown_label: Label,
    skip_btn: Button,
    snooze_btn: Button,
    complete_btn: Button,
    host: Arc<Mutex<AppHost>>,
}

impl OverlayWindow {
    pub fn new(host: Arc<Mutex<AppHost>>) -> Self {
        let window = Window::new();
        window.set_title(Some("Awayra Break"));
        window.set_default_size(800, 600);
        window.set_decorated(false);
        window.set_resizable(false);
        window.fullscreen();

        let vbox = GtkBox::new(Orientation::Vertical, 20);
        vbox.set_halign(Align::Center);
        vbox.set_valign(Align::Center);
        vbox.add_css_class("overlay-bg");

        let title_label = Label::new(Some("Break Time"));
        title_label.add_css_class("overlay-title");

        let countdown_label = Label::new(Some("0"));
        countdown_label.add_css_class("overlay-countdown");

        let instruction_label = Label::new(Some(""));
        instruction_label.add_css_class("overlay-instruction");

        // Buttons
        let btn_box = GtkBox::new(Orientation::Horizontal, 8);
        btn_box.set_halign(Align::Center);

        let skip_btn = Button::with_label("Skip");
        skip_btn.add_css_class("overlay-btn");

        let snooze_btn = Button::with_label("Snooze");
        snooze_btn.add_css_class("overlay-btn");

        let complete_btn = Button::with_label("Complete");
        complete_btn.add_css_class("overlay-btn-primary");

        btn_box.append(&skip_btn);
        btn_box.append(&snooze_btn);
        btn_box.append(&complete_btn);

        vbox.append(&title_label);
        vbox.append(&countdown_label);
        vbox.append(&instruction_label);
        vbox.append(&btn_box);

        window.set_child(Some(&vbox));

        // Connect signals
        let h = host.clone();
        skip_btn.connect_clicked(move |_| {
            if let Ok(host_ref) = h.lock() {
                if let Ok(mut sched) = host_ref.scheduler.lock() {
                    sched.skip_active_break();
                }
            }
        });

        let h2 = host.clone();
        snooze_btn.connect_clicked(move |_| {
            if let Ok(host_ref) = h2.lock() {
                if let Ok(mut sched) = host_ref.scheduler.lock() {
                    sched.snooze_active_break();
                }
            }
        });

        let h3 = host.clone();
        complete_btn.connect_clicked(move |_| {
            if let Ok(host_ref) = h3.lock() {
                if let Ok(mut sched) = host_ref.scheduler.lock() {
                    sched.complete_active_break();
                }
            }
        });

        Self {
            window,
            title_label,
            instruction_label,
            countdown_label,
            skip_btn,
            snooze_btn,
            complete_btn,
            host,
        }
    }

    pub fn show_break(&mut self, args: BreakStartedEventArgs) {
        if let Ok(host_lock) = self.host.lock() {
            let settings = host_lock.settings.lock().unwrap().clone();
            let loc = &host_lock.localization;

            match args.break_type {
                BreakType::Eye => {
                    self.title_label.set_text(&loc.get("EyeReset"));
                    self.instruction_label.set_text(&format!("{}\n{}",
                        loc.get("EyeResetInstructionDistance"),
                        loc.get("EyeResetInstructionBlink")));
                }
                BreakType::Move => {
                    self.title_label.set_text(&loc.get("MoveBreak"));
                    self.instruction_label.set_text(&loc.get_move_activity(args.activity_index));
                }
            }

            self.skip_btn.set_visible(settings.allow_skip);
            self.snooze_btn.set_visible(settings.allow_snooze);
        }

        self.countdown_label.set_text(&format!("{}", args.duration_seconds));
        self.window.present();
        self.window.fullscreen();
    }

    pub fn is_visible(&self) -> bool {
        self.window.is_visible()
    }

    pub fn close(&self) {
        self.window.close();
    }
}