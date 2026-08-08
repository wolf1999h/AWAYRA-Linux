use gtk4::prelude::*;
use gtk4::glib;
use gtk4::{ApplicationWindow, Label, Button, Box as GtkBox, Orientation, Align, Picture};
use std::sync::{Arc, Mutex};

use crate::core::models::{BreakType, BreakStartedEventArgs};
use crate::ui::services::app_host::AppHost;
use crate::ui::services::screenshot_service::ScreenshotResult;

pub struct OverlayWindow {
    windows: Vec<ApplicationWindow>,
    monitor_sizes: Vec<(i32, i32)>,
    title_label: Label,
    instruction_label: Label,
    countdown_label: Label,
    sound_btn: Button,
    skip_btn: Button,
    snooze_btn: Button,
    complete_btn: Button,
    host: Arc<Mutex<AppHost>>,
    bg_pictures: Vec<Picture>,
    eye_view: Option<super::eye_exercise::EyeExerciseView>,
    move_view: Option<super::move_exercise::MoveExerciseView>,
    exercise_box: GtkBox,
    current_break_type: Option<BreakType>,
}

impl OverlayWindow {
    pub fn new(app: &gtk4::Application, host: Arc<Mutex<AppHost>>) -> Self {
        let display = gtk4::gdk::Display::default().expect("No display found");
        let monitors = display.monitors();
        let monitor_count = monitors.n_items().max(1);

        let mut windows = Vec::new();
        let mut bg_pictures = Vec::new();
        let mut monitor_sizes = Vec::new();

        let title_label = Label::new(Some("Break Time"));
        title_label.add_css_class("overlay-title");

        let countdown_label = Label::new(Some("0"));
        countdown_label.add_css_class("timer-ring-text");

        let instruction_label = Label::new(Some(""));
        instruction_label.add_css_class("overlay-instruction-primary");
        instruction_label.set_justify(gtk4::Justification::Center);
        instruction_label.set_wrap(true);

        let exercise_box = GtkBox::new(Orientation::Vertical, 8);
        exercise_box.set_halign(Align::Center);

        // Buttons matching BreakOverlayWindow.xaml
        let btn_box = GtkBox::new(Orientation::Horizontal, 8);
        btn_box.set_halign(Align::Center);

        let sound_btn = Button::with_label("Mute");
        sound_btn.add_css_class("secondary-button");

        let skip_btn = Button::with_label("Skip");
        skip_btn.add_css_class("secondary-button");

        let snooze_btn = Button::with_label("Snooze");
        snooze_btn.add_css_class("secondary-button");

        let complete_btn = Button::with_label("Complete");
        complete_btn.add_css_class("primary-button");

        btn_box.append(&sound_btn);
        btn_box.append(&skip_btn);
        btn_box.append(&snooze_btn);
        btn_box.append(&complete_btn);

        for i in 0..monitor_count {
            let (mon_w, mon_h) = if let Some(mon_item) = monitors.item(i) {
                if let Ok(monitor) = mon_item.downcast::<gtk4::gdk::Monitor>() {
                    let geom = monitor.geometry();
                    (geom.width().max(800), geom.height().max(600))
                } else {
                    (1920, 1080)
                }
            } else {
                (1920, 1080)
            };
            monitor_sizes.push((mon_w, mon_h));

            let window = ApplicationWindow::new(app);
            window.set_title(Some("Awayra Break"));
            window.set_decorated(false);
            window.set_resizable(true);
            window.set_default_size(mon_w, mon_h);
            window.set_size_request(mon_w, mon_h);

            if let Some(mon_item) = monitors.item(i) {
                if let Ok(monitor) = mon_item.downcast::<gtk4::gdk::Monitor>() {
                    window.fullscreen_on_monitor(&monitor);
                } else {
                    window.fullscreen();
                }
            } else {
                window.fullscreen();
            }

            let root_box = GtkBox::new(Orientation::Vertical, 0);
            root_box.set_hexpand(true);
            root_box.set_vexpand(true);
            root_box.set_halign(Align::Fill);
            root_box.set_valign(Align::Fill);
            root_box.set_size_request(mon_w, mon_h);
            root_box.add_css_class("overlay-bg");

            let bg_picture = Picture::new();
            bg_picture.set_hexpand(true);
            bg_picture.set_vexpand(true);
            bg_picture.set_halign(Align::Fill);
            bg_picture.set_valign(Align::Fill);
            bg_picture.set_keep_aspect_ratio(false);
            bg_picture.set_can_shrink(true);
            bg_picture.set_can_target(false);

            let overlay_stack = gtk4::Overlay::new();
            overlay_stack.set_hexpand(true);
            overlay_stack.set_vexpand(true);
            overlay_stack.set_halign(Align::Fill);
            overlay_stack.set_valign(Align::Fill);
            overlay_stack.set_size_request(mon_w, mon_h);
            overlay_stack.set_child(Some(&bg_picture));

            if i == 0 {
                // Main monitor: WPF BreakOverlayWindow Card
                let card = GtkBox::new(Orientation::Vertical, 16);
                card.set_halign(Align::Center);
                card.set_valign(Align::Center);
                card.add_css_class("overlay-card");

                title_label.set_halign(Align::Center);
                card.append(&title_label);

                // Circular countdown ring container (150x150)
                let ring_box = GtkBox::new(Orientation::Vertical, 0);
                ring_box.add_css_class("timer-ring-box");
                ring_box.set_halign(Align::Center);
                ring_box.set_valign(Align::Center);
                countdown_label.set_hexpand(true);
                countdown_label.set_vexpand(true);
                countdown_label.set_halign(Align::Center);
                countdown_label.set_valign(Align::Center);
                ring_box.append(&countdown_label);
                card.append(&ring_box);

                instruction_label.set_halign(Align::Center);
                card.append(&instruction_label);
                card.append(&exercise_box);
                card.append(&btn_box);

                overlay_stack.add_overlay(&card);
            } else {
                // Secondary monitors: Peaceful Dim Card
                let card = GtkBox::new(Orientation::Vertical, 16);
                card.set_halign(Align::Center);
                card.set_valign(Align::Center);
                card.add_css_class("overlay-card");

                let sec_title = Label::new(Some("Awayra"));
                sec_title.add_css_class("overlay-title");

                let sec_sub = Label::new(Some("Take a break and rest your eyes"));
                sec_sub.add_css_class("overlay-instruction-primary");

                card.append(&sec_title);
                card.append(&sec_sub);
                overlay_stack.add_overlay(&card);
            }

            root_box.append(&overlay_stack);
            window.set_child(Some(&root_box));
            window.connect_close_request(|w| {
                w.hide();
                gtk4::glib::Propagation::Stop
            });

            windows.push(window);
            bg_pictures.push(bg_picture);
        }

        // Signals
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

        let h_sound = host.clone();
        let sound_btn_clone = sound_btn.clone();
        sound_btn.connect_clicked(move |_| {
            if let Ok(host_ref) = h_sound.lock() {
                host_ref.audio_service.stop_repeating();
                sound_btn_clone.set_label("Muted");
                sound_btn_clone.set_sensitive(false);
            }
        });

        Self {
            windows,
            monitor_sizes,
            title_label,
            instruction_label,
            countdown_label,
            sound_btn,
            skip_btn,
            snooze_btn,
            complete_btn,
            host,
            bg_pictures,
            eye_view: None,
            move_view: None,
            exercise_box,
            current_break_type: None,
        }
    }

    pub fn show_break(&mut self, args: BreakStartedEventArgs) {
        self.current_break_type = Some(args.break_type);

        // Remove previous exercise container widgets
        while let Some(child) = self.exercise_box.first_child() {
            self.exercise_box.remove(&child);
        }

        if let Ok(host_lock) = self.host.lock() {
            let settings = host_lock.settings.lock().unwrap().clone();
            let loc = &host_lock.localization;

            match args.break_type {
                BreakType::Eye => {
                    self.title_label.set_text(&loc.get("EyeReset"));
                    self.instruction_label.set_text(&format!("{}\n{}",
                        loc.get("EyeResetInstructionDistance"),
                        loc.get("EyeResetInstructionBlink")));

                    let eye = super::eye_exercise::EyeExerciseView::new();
                    self.exercise_box.append(&eye.container);
                    self.eye_view = Some(eye);
                    self.move_view = None;
                }
                BreakType::Move => {
                    self.title_label.set_text(&loc.get("MoveBreak"));
                    self.instruction_label.set_text(&loc.get_move_activity(args.activity_index));

                    let mv = super::move_exercise::MoveExerciseView::new(args.activity_index as usize);
                    self.exercise_box.append(&mv.container);
                    self.move_view = Some(mv);
                    self.eye_view = None;
                }
            }

            self.skip_btn.set_visible(settings.allow_skip);
            self.snooze_btn.set_visible(settings.allow_snooze);
            self.sound_btn.set_label("Mute");
            self.sound_btn.set_sensitive(true);

            // Custom background image takes priority over screenshot capture.
            let mut custom_applied = false;
            if let Some(bg_path) = &settings.custom_background_path {
                let trimmed = bg_path.trim();
                if !trimmed.is_empty() {
                    let path = std::path::Path::new(trimmed);
                    if path.exists() && path.is_file() {
                        custom_applied = Self::apply_custom_background(
                            &self.bg_pictures,
                            &self.monitor_sizes,
                            path,
                        );
                    }
                }
            }

            if custom_applied {
                // No screenshot capture needed; background is ready.
            } else if settings.capture_screenshot {
                // Capture and apply BEFORE the overlay window is shown, so the
                // screenshot contains only the desktop (not the overlay itself)
                // and the texture is ready for the first map with no flicker.
                // Any failure (X11/Wayland/portal) falls back to the clear
                // background instead of crashing the app.
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    host_lock.screenshot_service.capture()
                }))
                .ok()
                .flatten();
                if let Some(r) = result {
                    Self::apply_screenshot_background(
                        &self.bg_pictures,
                        &self.monitor_sizes,
                        r,
                    );
                } else {
                    Self::apply_clear_background(&self.bg_pictures);
                }
            } else {
                Self::apply_clear_background(&self.bg_pictures);
            }
        }

        self.countdown_label.set_text(&format!("{}", args.duration_seconds));
        for (idx, window) in self.windows.iter().enumerate() {
            // Re-assert exact fullscreen geometry on every show. Without this,
            // a window whose content measures ~0x0 (empty screenshot Picture)
            // can map at a tiny floating size instead of covering the monitor.
            if let Some(mon_item) = gtk4::gdk::Display::default()
                .and_then(|d| d.monitors().item(idx as u32))
            {
                if let Ok(monitor) = mon_item.downcast::<gtk4::gdk::Monitor>() {
                    window.fullscreen_on_monitor(&monitor);
                } else {
                    window.fullscreen();
                }
            } else {
                window.fullscreen();
            }
            window.present();
            window.set_visible(true);
            window.fullscreen();
            window.present();
        }
    }

    pub fn update_remaining(&mut self, remaining_seconds: i64) {
        let secs = remaining_seconds.max(0);
        self.countdown_label.set_text(&format!("{}", secs));

        // Advance exercise animation view every second
        if let Some(eye) = self.eye_view.as_mut() {
            eye.tick();
        }
        if let Some(mv) = self.move_view.as_mut() {
            mv.tick();
        }
    }

    pub fn is_visible(&self) -> bool {
        self.windows.first().map(|w| w.is_visible()).unwrap_or(false)
    }

    fn apply_custom_background(
        bg_pictures: &[Picture],
        monitor_sizes: &[(i32, i32)],
        path: &std::path::Path,
    ) -> bool {
        if !path.exists() || !path.is_file() {
            eprintln!("[Awayra] Custom background file not found: {:?}", path);
            return false;
        }

        eprintln!("[Awayra] Loading custom background image: {:?}", path);

        // Load the image synchronously with GDK's built-in loader, scaled to
        // the monitor size at decode time. This avoids the async race where
        // set_file() would load after the window is mapped and never render.
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            gtk4::gdk_pixbuf::Pixbuf::from_file_at_scale(path, -1, -1, false)
        }));

        let orig_pixbuf = match res {
            Ok(Ok(p)) => p,
            Ok(Err(err)) => {
                eprintln!("[Awayra] Failed to load custom background via pixbuf: {}", err);
                // last resort: GTK's async set_file
                let file = gtk4::gio::File::for_path(path);
                for bg in bg_pictures {
                    bg.set_file(Some(&file));
                    bg.set_keep_aspect_ratio(false);
                    bg.set_can_target(false);
                    bg.queue_draw();
                }
                return true;
            }
            Err(_) => {
                eprintln!("[Awayra] Panic loading custom background image");
                return false;
            }
        };

        let mut loaded_any = false;
        for (i, bg) in bg_pictures.iter().enumerate() {
            let (mon_w, mon_h) = monitor_sizes.get(i).copied().unwrap_or((1920, 1080));
            let w = mon_w.max(1);
            let h = mon_h.max(1);

            let scaled = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                orig_pixbuf.scale_simple(w, h, gtk4::gdk_pixbuf::InterpType::Bilinear)
            })) {
                Ok(Some(p)) => p,
                _ => orig_pixbuf.clone(),
            };

            let texture = gtk4::gdk::Texture::for_pixbuf(&scaled);
            bg.set_file(None::<&gtk4::gio::File>);
            bg.set_paintable(Some(&texture));
            bg.set_keep_aspect_ratio(false);
            bg.set_can_target(false);
            bg.queue_draw();
            loaded_any = true;
        }
        loaded_any
    }

    fn apply_screenshot_background(
        bg_pictures: &[Picture],
        monitor_sizes: &[(i32, i32)],
        result: ScreenshotResult,
    ) {
        let expected_len = (result.width as usize)
            .checked_mul(result.height as usize)
            .and_then(|area| area.checked_mul(4));
        if result.data.len() != expected_len.unwrap_or(0) {
            Self::apply_clear_background(bg_pictures);
            return;
        }

        let bytes = glib::Bytes::from(&result.data);
        let orig_pixbuf = match gtk4::gdk_pixbuf::Pixbuf::from_bytes(
            &bytes,
            gtk4::gdk_pixbuf::Colorspace::Rgb,
            true,
            8,
            result.width,
            result.height,
            result.width * 4,
        ) {
            pix => pix,
        };

        for (i, bg) in bg_pictures.iter().enumerate() {
            let (mon_w, mon_h) = monitor_sizes.get(i).copied().unwrap_or((800, 600));

            // Scale pixbuf to match exact monitor resolution if dimensions differ
            if mon_w > 0 && mon_h > 0 && (result.width != mon_w || result.height != mon_h) {
                if let Some(scaled) = orig_pixbuf.scale_simple(
                    mon_w,
                    mon_h,
                    gtk4::gdk_pixbuf::InterpType::Bilinear,
                ) {
                    let texture = gtk4::gdk::Texture::for_pixbuf(&scaled);
                    bg.set_file(None::<&gtk4::gio::File>);
                    bg.set_paintable(Some(&texture));
                    bg.queue_draw();
                    continue;
                }
            }

            let texture = gtk4::gdk::Texture::for_pixbuf(&orig_pixbuf);
            bg.set_file(None::<&gtk4::gio::File>);
            bg.set_paintable(Some(&texture));
            bg.queue_draw();
        }
    }

    fn apply_clear_background(bg_pictures: &[Picture]) {
        for bg in bg_pictures {
            bg.set_file(None::<&gtk4::gio::File>);
            bg.set_paintable(gtk4::gdk::Paintable::NONE);
            bg.queue_draw();
        }
    }

    pub fn close(&mut self) {
        self.current_break_type = None;
        self.eye_view = None;
        self.move_view = None;
        while let Some(child) = self.exercise_box.first_child() {
            self.exercise_box.remove(&child);
        }
        for window in &self.windows {
            window.hide();
        }
    }
}
