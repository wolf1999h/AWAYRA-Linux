use gtk4::prelude::*;
use gtk4::{Window, Label, Button, Switch, SpinButton, ScrolledWindow, Orientation, Align};
use std::sync::{Arc, Mutex};

use crate::core::models::AppSettings;
use crate::core::services::break_scheduler::BreakScheduler;

pub struct SettingsWindow {
    window: Window,
}

impl SettingsWindow {
    pub fn new(settings_arc: Arc<Mutex<AppSettings>>, scheduler: Arc<Mutex<BreakScheduler>>) -> Self {
        let window = Window::new();
        window.set_title(Some("Awayra Settings"));
        window.set_default_size(720, 600);
        window.set_resizable(true);

        let vbox = gtk4::Box::new(Orientation::Vertical, 12);
        vbox.set_margin_top(16);
        vbox.set_margin_bottom(16);
        vbox.set_margin_start(16);
        vbox.set_margin_end(16);

        let title = Label::new(Some("Settings"));
        title.add_css_class("settings-title");
        vbox.append(&title);

        let subtitle = Label::new(Some("Configure reminder intervals, behavior, appearance, and system integration."));
        subtitle.add_css_class("settings-subtitle");
        vbox.append(&subtitle);

        let scroll = ScrolledWindow::new();
        scroll.set_vexpand(true);
        scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);

        let content = gtk4::Box::new(Orientation::Vertical, 16);
        content.set_margin_top(8);
        content.set_margin_bottom(8);

        let settings = settings_arc.lock().unwrap().clone();

        // Eye Reset section
        let eye_section = Self::create_section("Eye Reset");
        let eye_enabled = Switch::new();
        eye_enabled.set_active(settings.eye_reset_enabled);
        Self::add_switch_row(&eye_section, "Enabled", &eye_enabled);
        let eye_interval = SpinButton::with_range(1.0, 480.0, 1.0);
        eye_interval.set_value(settings.eye_reset_interval_minutes as f64);
        Self::add_spin_row(&eye_section, "Interval", "minutes", &eye_interval);
        let eye_duration = SpinButton::with_range(5.0, 600.0, 1.0);
        eye_duration.set_value(settings.eye_reset_duration_seconds as f64);
        Self::add_spin_row(&eye_section, "Duration", "seconds", &eye_duration);
        content.append(&eye_section);

        // Move Break section
        let move_section = Self::create_section("Move Break");
        let move_enabled = Switch::new();
        move_enabled.set_active(settings.move_break_enabled);
        Self::add_switch_row(&move_section, "Enabled", &move_enabled);
        let move_interval = SpinButton::with_range(1.0, 480.0, 1.0);
        move_interval.set_value(settings.move_break_interval_minutes as f64);
        Self::add_spin_row(&move_section, "Interval", "minutes", &move_interval);
        let move_duration = SpinButton::with_range(5.0, 600.0, 1.0);
        move_duration.set_value(settings.move_break_duration_seconds as f64);
        Self::add_spin_row(&move_section, "Duration", "seconds", &move_duration);
        content.append(&move_section);

        // Behavior section
        let behavior_section = Self::create_section("Reminder behavior");
        let allow_skip = Switch::new();
        allow_skip.set_active(settings.allow_skip);
        Self::add_switch_row(&behavior_section, "Allow skip", &allow_skip);
        let allow_snooze = Switch::new();
        allow_snooze.set_active(settings.allow_snooze);
        Self::add_switch_row(&behavior_section, "Allow snooze", &allow_snooze);
        let snooze_dur = SpinButton::with_range(1.0, 60.0, 1.0);
        snooze_dur.set_value(settings.snooze_duration_minutes as f64);
        Self::add_spin_row(&behavior_section, "Snooze duration", "minutes", &snooze_dur);
        content.append(&behavior_section);

        // Idle & Work Hours section
        let idle_section = Self::create_section("Idle and work hours");
        let pause_idle = Switch::new();
        pause_idle.set_active(settings.pause_while_idle);
        Self::add_switch_row(&idle_section, "Reset after idle", &pause_idle);
        let idle_thresh = SpinButton::with_range(1.0, 120.0, 1.0);
        idle_thresh.set_value(settings.idle_threshold_minutes as f64);
        Self::add_spin_row(&idle_section, "Idle threshold", "minutes", &idle_thresh);
        let work_hours = Switch::new();
        work_hours.set_active(settings.work_hours_enabled);
        Self::add_switch_row(&idle_section, "Enable work hours", &work_hours);
        content.append(&idle_section);

        // Screenshot section
        let screenshot_section = Self::create_section("Overlay background");
        let capture_switch = Switch::new();
        capture_switch.set_active(settings.capture_screenshot);
        Self::add_switch_row(&screenshot_section, "Capture screenshot", &capture_switch);
        let note = Label::new(Some("If disabled, a solid dark background will be shown during breaks.\nOn Wayland, enabling this will ask for permission via the portal."));
        note.add_css_class("muted-text");
        note.set_wrap(true);
        screenshot_section.append(&note);
        content.append(&screenshot_section);

        scroll.set_child(Some(&content));
        vbox.append(&scroll);

        // Action bar
        let action_bar = gtk4::Box::new(Orientation::Horizontal, 8);
        action_bar.set_halign(Align::End);
        action_bar.set_margin_top(8);

        let save_btn = Button::with_label("Save");
        save_btn.add_css_class("primary-button");
        action_bar.append(&save_btn);

        let close_btn = Button::with_label("Close");
        close_btn.add_css_class("secondary-button");
        action_bar.append(&close_btn);

        vbox.append(&action_bar);
        window.set_child(Some(&vbox));

        // Save signal - update BOTH settings and scheduler
        let settings_arc_clone = settings_arc.clone();
        let scheduler_clone = scheduler.clone();
        save_btn.connect_clicked(move |_| {
            let new_settings = AppSettings {
                schema_version: crate::core::models::CURRENT_SCHEMA_VERSION,
                eye_reset_enabled: eye_enabled.is_active(),
                eye_reset_interval_minutes: eye_interval.value() as i32,
                eye_reset_duration_seconds: eye_duration.value() as i32,
                move_break_enabled: move_enabled.is_active(),
                move_break_interval_minutes: move_interval.value() as i32,
                move_break_duration_seconds: move_duration.value() as i32,
                allow_skip: allow_skip.is_active(),
                allow_snooze: allow_snooze.is_active(),
                snooze_duration_minutes: snooze_dur.value() as i32,
                pause_while_idle: pause_idle.is_active(),
                idle_threshold_minutes: idle_thresh.value() as i32,
                work_hours_enabled: work_hours.is_active(),
                work_start_hour: 9,
                work_start_minute: 0,
                work_end_hour: 18,
                work_end_minute: 0,
                run_at_startup: false,
                start_minimized: false,
                close_to_tray: true,
                glass_clarity: 75,
                reduced_motion: false,
                theme: crate::core::models::AppTheme::Dark,
                capture_screenshot: capture_switch.is_active(),
            };

            // Update settings storage
            if let Ok(mut settings) = settings_arc_clone.lock() {
                *settings = new_settings.clone();
            }

            // Update scheduler with new settings
            if let Ok(mut sched) = scheduler_clone.lock() {
                sched.update_settings(new_settings);
            }
        });

        // Close signal
        let w = window.clone();
        close_btn.connect_clicked(move |_| w.close());

        Self { window }
    }

    fn create_section(title: &str) -> gtk4::Box {
        let section = gtk4::Box::new(Orientation::Vertical, 8);
        section.set_margin_top(8);
        section.set_margin_bottom(8);
        section.set_margin_start(8);
        section.set_margin_end(8);
        section.add_css_class("settings-section");

        let section_title = Label::new(Some(title));
        section_title.add_css_class("section-title");
        section.append(&section_title);

        section
    }

    fn add_switch_row(parent: &gtk4::Box, label: &str, switch_widget: &Switch) {
        let row = gtk4::Box::new(Orientation::Horizontal, 8);
        let lbl = Label::new(Some(label));
        lbl.set_hexpand(true);
        lbl.set_halign(Align::Start);
        row.append(&lbl);
        row.append(switch_widget);
        parent.append(&row);
    }

    fn add_spin_row(parent: &gtk4::Box, label: &str, unit: &str, spin: &SpinButton) {
        let row = gtk4::Box::new(Orientation::Horizontal, 8);
        let lbl = Label::new(Some(label));
        lbl.set_size_request(130, -1);
        row.append(&lbl);
        row.append(spin);
        let unit_lbl = Label::new(Some(unit));
        row.append(&unit_lbl);
        parent.append(&row);
    }

    pub fn show(&self) {
        self.window.present();
    }
}