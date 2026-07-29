use gtk4::prelude::*;
use gtk4::{Window, Label, Button, Box as GtkBox, Orientation, Align, Picture};
use std::sync::{Arc, Mutex};

use crate::core::models::{BreakType, BreakStartedEventArgs};
use crate::ui::services::app_host::AppHost;
use crate::ui::services::screenshot_service::ScreenshotResult;

pub struct OverlayWindow {
    window: Window,
    title_label: Label,
    instruction_label: Label,
    countdown_label: Label,
    skip_btn: Button,
    snooze_btn: Button,
    complete_btn: Button,
    host: Arc<Mutex<AppHost>>,
    bg_picture: Picture,
}

impl OverlayWindow {
    pub fn new(host: Arc<Mutex<AppHost>>) -> Self {
        let window = Window::new();
        window.set_title(Some("Awayra Break"));
        window.set_default_size(800, 600);
        window.set_decorated(false);
        window.set_resizable(false);
        window.fullscreen();

        let bg_picture = Picture::new();
        bg_picture.set_size_request(800, 600);
        bg_picture.add_css_class("overlay-bg");

        let vbox = GtkBox::new(Orientation::Vertical, 20);
        vbox.set_halign(Align::Center);
        vbox.set_valign(Align::Center);
        vbox.add_css_class("glass-overlay");

        let title_label = Label::new(Some("Break Time"));
        title_label.add_css_class("overlay-title");
        title_label.add_css_class("title-label");

        let countdown_label = Label::new(Some("0"));
        countdown_label.add_css_class("overlay-countdown");
        countdown_label.add_css_class("timer-display");

        let instruction_label = Label::new(Some(""));
        instruction_label.add_css_class("overlay-instruction");

        // Buttons
        let btn_box = GtkBox::new(Orientation::Horizontal, 8);
        btn_box.set_halign(Align::Center);

        let skip_btn = Button::with_label("Skip");
        skip_btn.add_css_class("overlay-btn");
        skip_btn.add_css_class("secondary-button");

        let snooze_btn = Button::with_label("Snooze");
        snooze_btn.add_css_class("overlay-btn");
        snooze_btn.add_css_class("secondary-button");

        let complete_btn = Button::with_label("Complete");
        complete_btn.add_css_class("overlay-btn-primary");
        complete_btn.add_css_class("primary-btn");

        btn_box.append(&skip_btn);
        btn_box.append(&snooze_btn);
        btn_box.append(&complete_btn);

        // Create an overlay stack: picture at back, content in front
        let overlay_stack = gtk4::Overlay::new();
        overlay_stack.add_overlay(&bg_picture);
        overlay_stack.set_child(Some(&vbox));

        vbox.append(&title_label);
        vbox.append(&countdown_label);
        vbox.append(&instruction_label);
        vbox.append(&btn_box);

        window.set_child(Some(&overlay_stack));

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
            bg_picture,
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

            // Capture screenshot if enabled
            if settings.capture_screenshot {
                if let Some(result) = host_lock.screenshot_service.capture() {
                    self.set_screenshot_background(result);
                } else {
                    self.clear_screenshot_background();
                }
            } else {
                self.clear_screenshot_background();
            }
        }

        self.countdown_label.set_text(&format!("{}", args.duration_seconds));
        self.window.present();
        self.window.fullscreen();
    }

    pub fn is_visible(&self) -> bool {
        self.window.is_visible()
    }

    fn set_screenshot_background(&self, result: ScreenshotResult) {
        // Convert RGBA bytes to a GPixbuf and display in the Picture
        let bytes = glib::Bytes::from(result.data);
        let pixbuf = gtk4::gdk_pixbuf::Pixbuf::from_bytes(
            &bytes,
            gtk4::gdk_pixbuf::Colorspace::RGB,
            true,
            8,
            result.width,
            result.height,
            result.width * 4,
        );
        let texture = gtk4::gdk::Texture::for_pixbuf(&pixbuf);
        self.bg_picture.set_paintable(Some(&texture));
    }

    fn clear_screenshot_background(&self) {
        self.bg_picture.set_paintable(gtk4::gdk::Paintable::NONE);
    }

    pub fn close(&self) {
        self.window.close();
    }
}
