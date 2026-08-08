use gtk4::prelude::*;
use gtk4::glib;
use gtk4::{Window, Label, Button, Switch, SpinButton, Scale, DropDown, ScrolledWindow, Orientation, Align, StringList};
use std::sync::{Arc, Mutex};

use crate::core::models::{AppSettings, AppTheme, AppLanguage, BreakSoundTheme};
use crate::ui::services::app_host::AppHost;

pub struct SettingsWindow {
    window: Window,
}

impl SettingsWindow {
    pub fn new(host: Arc<Mutex<AppHost>>) -> Self {
        // Pause scheduler while configuration is open
        if let Ok(host_ref) = host.lock() {
            host_ref.enter_configuration_pause();
        }

        let host_lock = host.lock().unwrap();
        let loc = host_lock.localization.clone();
        let settings = host_lock.settings.lock().unwrap().clone();
        drop(host_lock);

        let window = Window::new();
        window.set_title(Some(&loc.get("SettingsTitle")));
        window.set_default_size(760, 680);
        window.set_resizable(true);

        let vbox = gtk4::Box::new(Orientation::Vertical, 12);
        vbox.set_margin_top(16);
        vbox.set_margin_bottom(16);
        vbox.set_margin_start(16);
        vbox.set_margin_end(16);

        let title = Label::new(Some(&loc.get("SettingsTitle")));
        title.add_css_class("settings-title");
        title.add_css_class("title-label");
        vbox.append(&title);

        let subtitle = Label::new(Some(&loc.get("SettingsSubtitle")));
        subtitle.add_css_class("settings-subtitle");
        subtitle.add_css_class("subtitle-label");
        vbox.append(&subtitle);

        let scroll = ScrolledWindow::new();
        scroll.set_vexpand(true);
        scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);

        let content = gtk4::Box::new(Orientation::Vertical, 16);
        content.set_margin_top(8);
        content.set_margin_bottom(8);

        // 1. Eye Reset section
        let eye_section = Self::create_section(&loc.get("SettingsEyeResetSection"));
        let eye_enabled = Switch::new();
        eye_enabled.set_active(settings.eye_reset_enabled);
        Self::add_switch_row(&eye_section, &loc.get("SettingsEnableEyeReset"), &eye_enabled);
        let eye_interval = SpinButton::with_range(1.0, 480.0, 1.0);
        eye_interval.set_value(settings.eye_reset_interval_minutes as f64);
        Self::add_spin_row(&eye_section, &loc.get("SettingsInterval"), &loc.get("MinutesUnit"), &eye_interval);
        let eye_duration = SpinButton::with_range(5.0, 600.0, 1.0);
        eye_duration.set_value(settings.eye_reset_duration_seconds as f64);
        Self::add_spin_row(&eye_section, &loc.get("SettingsDuration"), &loc.get("SecondsUnit"), &eye_duration);
        let eye_sound_enabled = Switch::new();
        eye_sound_enabled.set_active(settings.eye_break_sound_enabled);
        Self::add_switch_row(&eye_section, &loc.get("SettingsPlaySoundOnStart"), &eye_sound_enabled);
        content.append(&eye_section);

        // 2. Move Break section
        let move_section = Self::create_section(&loc.get("SettingsMoveBreakSection"));
        let move_enabled = Switch::new();
        move_enabled.set_active(settings.move_break_enabled);
        Self::add_switch_row(&move_section, &loc.get("SettingsEnableMoveBreak"), &move_enabled);
        let move_interval = SpinButton::with_range(1.0, 480.0, 1.0);
        move_interval.set_value(settings.move_break_interval_minutes as f64);
        Self::add_spin_row(&move_section, &loc.get("SettingsInterval"), &loc.get("MinutesUnit"), &move_interval);
        let move_duration = SpinButton::with_range(5.0, 600.0, 1.0);
        move_duration.set_value(settings.move_break_duration_seconds as f64);
        Self::add_spin_row(&move_section, &loc.get("SettingsDuration"), &loc.get("SecondsUnit"), &move_duration);
        let move_sound_enabled = Switch::new();
        move_sound_enabled.set_active(settings.move_break_sound_enabled);
        Self::add_switch_row(&move_section, &loc.get("SettingsPlaySoundOnStart"), &move_sound_enabled);
        content.append(&move_section);

        // 3. Sound Configuration section
        let sound_section = Self::create_section(&loc.get("SettingsSoundSection"));

        let sound_themes = StringList::new(&[
            &loc.get("SoundThemeSoftBell"),
            &loc.get("SoundThemeGentleChime"),
            &loc.get("SoundThemeCalmDrop"),
            &loc.get("SoundThemeCalmPiano"),
            &loc.get("SoundThemeMorningDew"),
            &loc.get("SoundThemeStillWater"),
        ]);
        let sound_theme_dropdown = DropDown::new(Some(sound_themes), None::<&gtk4::Expression>);
        sound_theme_dropdown.set_selected(match settings.break_sound_theme {
            BreakSoundTheme::SoftBell => 0,
            BreakSoundTheme::GentleChime => 1,
            BreakSoundTheme::CalmDrop => 2,
            BreakSoundTheme::CalmPiano => 3,
            BreakSoundTheme::MorningDew => 4,
            BreakSoundTheme::StillWater => 5,
        });
        Self::add_widget_row(&sound_section, &loc.get("SettingsSoundTheme"), &sound_theme_dropdown);

        let sound_volume = SpinButton::with_range(0.0, 100.0, 5.0);
        sound_volume.set_value(settings.break_sound_volume as f64);
        Self::add_spin_row(&sound_section, &loc.get("SettingsVolume"), "%", &sound_volume);

        let sound_repeat = SpinButton::with_range(0.0, 60.0, 1.0);
        sound_repeat.set_value(settings.break_sound_repeat_seconds as f64);
        Self::add_spin_row(&sound_section, &loc.get("SettingsRepeatInterval"), &loc.get("SecondsUnit"), &sound_repeat);

        let preview_btn = Button::with_label(&loc.get("SettingsPreviewSound"));
        preview_btn.add_css_class("secondary-button");
        let audio_service = host.lock().unwrap().audio_service.clone();
        let theme_dropdown_clone = sound_theme_dropdown.clone();
        let volume_spin_clone = sound_volume.clone();
        preview_btn.connect_clicked(move |_| {
            let selected = theme_dropdown_clone.selected();
            let theme = match selected {
                0 => BreakSoundTheme::SoftBell,
                1 => BreakSoundTheme::GentleChime,
                2 => BreakSoundTheme::CalmDrop,
                3 => BreakSoundTheme::CalmPiano,
                4 => BreakSoundTheme::MorningDew,
                _ => BreakSoundTheme::StillWater,
            };
            let vol = volume_spin_clone.value() as i32;
            audio_service.preview_sound(theme, vol);
        });
        Self::add_widget_row(&sound_section, &loc.get("SettingsTestSound"), &preview_btn);
        content.append(&sound_section);

        // 4. Behavior section
        let behavior_section = Self::create_section(&loc.get("SettingsBehaviorSection"));
        let allow_skip = Switch::new();
        allow_skip.set_active(settings.allow_skip);
        Self::add_switch_row(&behavior_section, &loc.get("SettingsAllowSkip"), &allow_skip);
        let allow_snooze = Switch::new();
        allow_snooze.set_active(settings.allow_snooze);
        Self::add_switch_row(&behavior_section, &loc.get("SettingsAllowSnooze"), &allow_snooze);
        let snooze_dur = SpinButton::with_range(1.0, 60.0, 1.0);
        snooze_dur.set_value(settings.snooze_duration_minutes as f64);
        Self::add_spin_row(&behavior_section, &loc.get("SettingsSnoozeDuration"), &loc.get("MinutesUnit"), &snooze_dur);
        content.append(&behavior_section);

        // 5. Idle & Work Hours section
        let idle_section = Self::create_section(&loc.get("SettingsIdleSection"));
        let pause_idle = Switch::new();
        pause_idle.set_active(settings.pause_while_idle);
        Self::add_switch_row(&idle_section, &loc.get("SettingsPauseWhileIdle"), &pause_idle);
        let idle_thresh = SpinButton::with_range(1.0, 120.0, 1.0);
        idle_thresh.set_value(settings.idle_threshold_minutes as f64);
        Self::add_spin_row(&idle_section, &loc.get("SettingsIdleThreshold"), &loc.get("MinutesUnit"), &idle_thresh);

        let work_hours = Switch::new();
        work_hours.set_active(settings.work_hours_enabled);
        Self::add_switch_row(&idle_section, &loc.get("SettingsWorkHoursEnabled"), &work_hours);

        let work_start_h = SpinButton::with_range(0.0, 23.0, 1.0);
        work_start_h.set_value(settings.work_start_hour as f64);
        let work_start_m = SpinButton::with_range(0.0, 59.0, 1.0);
        work_start_m.set_value(settings.work_start_minute as f64);
        Self::add_time_row(&idle_section, &loc.get("SettingsWorkStart"), &work_start_h, &work_start_m, &loc.get("HoursUnit"), &loc.get("MinutesUnit"));

        let work_end_h = SpinButton::with_range(0.0, 23.0, 1.0);
        work_end_h.set_value(settings.work_end_hour as f64);
        let work_end_m = SpinButton::with_range(0.0, 59.0, 1.0);
        work_end_m.set_value(settings.work_end_minute as f64);
        Self::add_time_row(&idle_section, &loc.get("SettingsWorkEnd"), &work_end_h, &work_end_m, &loc.get("HoursUnit"), &loc.get("MinutesUnit"));
        content.append(&idle_section);

        // 6. Appearance & Language section
        let appearance_section = Self::create_section(&loc.get("SettingsAppearanceSection"));

        let languages = StringList::new(&["English", "فارسی"]);
        let language_dropdown = DropDown::new(Some(languages), None::<&gtk4::Expression>);
        language_dropdown.set_selected(match settings.language {
            AppLanguage::English => 0,
            AppLanguage::Persian => 1,
        });
        Self::add_widget_row(&appearance_section, &loc.get("SettingsLanguage"), &language_dropdown);

        let glass_slider = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 1.0);
        glass_slider.set_value(settings.glass_clarity as f64);
        glass_slider.set_hexpand(true);
        let clarity_val_label = Label::new(Some(&format!("{}%", settings.glass_clarity)));
        clarity_val_label.set_size_request(45, -1);

        let glass_row = gtk4::Box::new(Orientation::Horizontal, 8);
        let glass_lbl = Label::new(Some(&loc.get("SettingsGlassClarity")));
        glass_lbl.set_size_request(160, -1);
        glass_lbl.set_halign(Align::Start);
        glass_row.append(&glass_lbl);
        glass_row.append(&glass_slider);
        glass_row.append(&clarity_val_label);
        appearance_section.append(&glass_row);

        let clarity_label_clone = clarity_val_label.clone();
        glass_slider.connect_value_changed(move |scale| {
            clarity_label_clone.set_text(&format!("{}%", scale.value() as i32));
        });

        let reduced_motion = Switch::new();
        reduced_motion.set_active(settings.reduced_motion);
        Self::add_switch_row(&appearance_section, &loc.get("SettingsReducedMotion"), &reduced_motion);

        let capture_switch = Switch::new();
        capture_switch.set_active(settings.capture_screenshot);
        Self::add_switch_row(&appearance_section, &loc.get("SettingsTransparentScreenshot"), &capture_switch);

        // Custom background image picker
        let custom_bg_path: Option<String> = settings.custom_background_path.clone();
        let bg_row = gtk4::Box::new(Orientation::Horizontal, 8);
        let bg_lbl = Label::new(Some(&loc.get("SettingsCustomBg")));
        bg_lbl.set_size_request(160, -1);
        bg_lbl.set_halign(Align::Start);
        bg_row.append(&bg_lbl);

        let bg_entry = gtk4::Entry::new();
        if let Some(p) = &custom_bg_path {
            bg_entry.set_text(p);
        }
        bg_entry.set_hexpand(true);
        bg_entry.set_placeholder_text(Some(&loc.get("SettingsChooseImage")));
        bg_row.append(&bg_entry);

        let browse_btn = Button::with_label(&loc.get("SettingsBrowse"));
        browse_btn.add_css_class("secondary-button");
        let entry_clone = bg_entry.clone();
        let host_for_browse = host.clone();
        browse_btn.connect_clicked(move |_| {
            let chooser = gtk4::FileChooserNative::new(
                Some("Select Background Image"),
                None::<&gtk4::Window>,
                gtk4::FileChooserAction::Open,
                Some("Select"),
                Some("Cancel"),
            );
            let filter = gtk4::FileFilter::new();
            filter.set_name(Some("Images"));
            filter.add_mime_type("image/png");
            filter.add_mime_type("image/jpeg");
            filter.add_mime_type("image/webp");
            filter.add_pattern("*.png");
            filter.add_pattern("*.jpg");
            filter.add_pattern("*.jpeg");
            filter.add_pattern("*.webp");
            chooser.add_filter(&filter);

            let entry_target = entry_clone.clone();
            let host_target = host_for_browse.clone();
            chooser.connect_response(move |dialog, response| {
                if response == gtk4::ResponseType::Accept {
                    if let Some(file) = dialog.file() {
                        if let Some(path) = file.path() {
                            // Automatically copy chosen background file into app's data directory
                            let stored = if let Ok(h) = host_target.lock() {
                                h.store_custom_background(&path)
                            } else {
                                None
                            };

                            let final_path = stored.unwrap_or(path);
                            entry_target.set_text(&final_path.to_string_lossy());
                        }
                    }
                }
                dialog.destroy();
            });
            chooser.show();
        });
        bg_row.append(&browse_btn);
        appearance_section.append(&bg_row);

        let note = Label::new(Some(&loc.get("SettingsAppearanceNote")));
        note.add_css_class("muted-text");
        note.set_wrap(true);
        appearance_section.append(&note);
        content.append(&appearance_section);

        // 7. System Integration section
        let system_section = Self::create_section(&loc.get("SettingsSystemSection"));
        let autostart_switch = Switch::new();
        autostart_switch.set_active(crate::core::services::autostart_service::AutostartService::is_autostart_enabled());
        Self::add_switch_row(&system_section, &loc.get("SettingsRunAtStartup"), &autostart_switch);

        let start_minimized = Switch::new();
        start_minimized.set_active(settings.start_minimized);
        Self::add_switch_row(&system_section, &loc.get("SettingsStartMinimized"), &start_minimized);

        let close_to_tray = Switch::new();
        close_to_tray.set_active(settings.close_to_tray);
        Self::add_switch_row(&system_section, &loc.get("SettingsCloseToTray"), &close_to_tray);
        content.append(&system_section);

        scroll.set_child(Some(&content));
        vbox.append(&scroll);

        // Action bar
        let action_bar = gtk4::Box::new(Orientation::Horizontal, 8);
        action_bar.set_hexpand(true);
        action_bar.set_margin_top(8);

        let reset_btn = Button::with_label(&loc.get("SettingsResetDefaults"));
        reset_btn.add_css_class("secondary-button");
        reset_btn.set_halign(Align::Start);
        reset_btn.set_hexpand(true);
        action_bar.append(&reset_btn);

        let save_btn = Button::with_label(&loc.get("SettingsSave"));
        save_btn.add_css_class("primary-button");
        save_btn.add_css_class("primary-btn");
        action_bar.append(&save_btn);

        let cancel_btn = Button::with_label(&loc.get("SettingsCancel"));
        cancel_btn.add_css_class("secondary-button");
        action_bar.append(&cancel_btn);

        vbox.append(&action_bar);

        // Reset signal - restore input controls to default settings values
        let eye_enabled_reset = eye_enabled.clone();
        let eye_interval_reset = eye_interval.clone();
        let eye_duration_reset = eye_duration.clone();
        let eye_sound_enabled_reset = eye_sound_enabled.clone();
        let move_enabled_reset = move_enabled.clone();
        let move_interval_reset = move_interval.clone();
        let move_duration_reset = move_duration.clone();
        let move_sound_enabled_reset = move_sound_enabled.clone();
        let sound_theme_dropdown_reset = sound_theme_dropdown.clone();
        let sound_volume_reset = sound_volume.clone();
        let sound_repeat_reset = sound_repeat.clone();
        let allow_skip_reset = allow_skip.clone();
        let allow_snooze_reset = allow_snooze.clone();
        let snooze_dur_reset = snooze_dur.clone();
        let pause_idle_reset = pause_idle.clone();
        let idle_thresh_reset = idle_thresh.clone();
        let work_hours_reset = work_hours.clone();
        let work_start_h_reset = work_start_h.clone();
        let work_start_m_reset = work_start_m.clone();
        let work_end_h_reset = work_end_h.clone();
        let work_end_m_reset = work_end_m.clone();
        let language_dropdown_reset = language_dropdown.clone();
        let glass_slider_reset = glass_slider.clone();
        let reduced_motion_reset = reduced_motion.clone();
        let capture_switch_reset = capture_switch.clone();
        let bg_entry_reset = bg_entry.clone();
        let autostart_switch_reset = autostart_switch.clone();
        let start_minimized_reset = start_minimized.clone();
        let close_to_tray_reset = close_to_tray.clone();

        reset_btn.connect_clicked(move |_| {
            let def = AppSettings::default();
            eye_enabled_reset.set_active(def.eye_reset_enabled);
            eye_interval_reset.set_value(def.eye_reset_interval_minutes as f64);
            eye_duration_reset.set_value(def.eye_reset_duration_seconds as f64);
            eye_sound_enabled_reset.set_active(def.eye_break_sound_enabled);

            move_enabled_reset.set_active(def.move_break_enabled);
            move_interval_reset.set_value(def.move_break_interval_minutes as f64);
            move_duration_reset.set_value(def.move_break_duration_seconds as f64);
            move_sound_enabled_reset.set_active(def.move_break_sound_enabled);

            sound_theme_dropdown_reset.set_selected(match def.break_sound_theme {
                BreakSoundTheme::SoftBell => 0,
                BreakSoundTheme::GentleChime => 1,
                BreakSoundTheme::CalmDrop => 2,
                BreakSoundTheme::CalmPiano => 3,
                BreakSoundTheme::MorningDew => 4,
                BreakSoundTheme::StillWater => 5,
            });
            sound_volume_reset.set_value(def.break_sound_volume as f64);
            sound_repeat_reset.set_value(def.break_sound_repeat_seconds as f64);

            allow_skip_reset.set_active(def.allow_skip);
            allow_snooze_reset.set_active(def.allow_snooze);
            snooze_dur_reset.set_value(def.snooze_duration_minutes as f64);

            pause_idle_reset.set_active(def.pause_while_idle);
            idle_thresh_reset.set_value(def.idle_threshold_minutes as f64);
            work_hours_reset.set_active(def.work_hours_enabled);
            work_start_h_reset.set_value(def.work_start_hour as f64);
            work_start_m_reset.set_value(def.work_start_minute as f64);
            work_end_h_reset.set_value(def.work_end_hour as f64);
            work_end_m_reset.set_value(def.work_end_minute as f64);

            language_dropdown_reset.set_selected(match def.language {
                AppLanguage::English => 0,
                AppLanguage::Persian => 1,
            });
            glass_slider_reset.set_value(def.glass_clarity as f64);
            reduced_motion_reset.set_active(def.reduced_motion);
            capture_switch_reset.set_active(def.capture_screenshot);
            bg_entry_reset.set_text("");

            autostart_switch_reset.set_active(def.run_at_startup);
            start_minimized_reset.set_active(def.start_minimized);
            close_to_tray_reset.set_active(def.close_to_tray);
        });
        window.set_child(Some(&vbox));

        // Save signal - persist settings and resume scheduler
        let host_for_save = host.clone();
        let win_for_save = window.clone();
        save_btn.connect_clicked(move |_| {
            let run_startup = autostart_switch.is_active();
            let _ = crate::core::services::autostart_service::AutostartService::set_autostart(run_startup);
            let custom_bg_path = {
                let text = bg_entry.text().trim().to_string();
                if text.is_empty() {
                    None
                } else {
                    let src_path = std::path::Path::new(&text);
                    if src_path.exists() && src_path.is_file() {
                        let stored = if let Ok(h) = host_for_save.lock() {
                            h.store_custom_background(src_path)
                        } else {
                            None
                        };
                        stored.map(|p| p.to_string_lossy().to_string()).or(Some(text))
                    } else {
                        Some(text)
                    }
                }
            };

            let selected_theme = match sound_theme_dropdown.selected() {
                0 => BreakSoundTheme::SoftBell,
                1 => BreakSoundTheme::GentleChime,
                2 => BreakSoundTheme::CalmDrop,
                3 => BreakSoundTheme::CalmPiano,
                4 => BreakSoundTheme::MorningDew,
                _ => BreakSoundTheme::StillWater,
            };

            let selected_language = match language_dropdown.selected() {
                1 => AppLanguage::Persian,
                _ => AppLanguage::English,
            };

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
                work_start_hour: work_start_h.value() as i32,
                work_start_minute: work_start_m.value() as i32,
                work_end_hour: work_end_h.value() as i32,
                work_end_minute: work_end_m.value() as i32,
                run_at_startup: run_startup,
                start_minimized: start_minimized.is_active(),
                close_to_tray: close_to_tray.is_active(),
                glass_clarity: glass_slider.value() as i32,
                reduced_motion: reduced_motion.is_active(),
                theme: AppTheme::Dark,
                language: selected_language,
                eye_break_sound_enabled: eye_sound_enabled.is_active(),
                move_break_sound_enabled: move_sound_enabled.is_active(),
                break_sound_theme: selected_theme,
                break_sound_volume: sound_volume.value() as i32,
                break_sound_repeat_seconds: sound_repeat.value() as i32,
                break_animation_enabled: !reduced_motion.is_active(),
                capture_screenshot: capture_switch.is_active(),
                custom_background_path: custom_bg_path,
            };

            if let Ok(host_ref) = host_for_save.lock() {
                let _ = host_ref.save_configuration(new_settings);
                host_ref.exit_configuration_pause();
            }
            win_for_save.close();
        });

        // Cancel signal
        let host_for_cancel = host.clone();
        let win_for_cancel = window.clone();
        cancel_btn.connect_clicked(move |_| {
            if let Ok(host_ref) = host_for_cancel.lock() {
                host_ref.exit_configuration_pause();
            }
            win_for_cancel.close();
        });

        // Window close-request signal (captures X button click)
        let host_for_close = host.clone();
        window.connect_close_request(move |_| {
            if let Ok(host_ref) = host_for_close.lock() {
                host_ref.exit_configuration_pause();
            }
            glib::Propagation::Proceed
        });

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
        section_title.set_halign(Align::Start);
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
        lbl.set_size_request(160, -1);
        lbl.set_halign(Align::Start);
        row.append(&lbl);
        row.append(spin);
        let unit_lbl = Label::new(Some(unit));
        row.append(&unit_lbl);
        parent.append(&row);
    }

    fn add_widget_row(parent: &gtk4::Box, label: &str, widget: &impl IsA<gtk4::Widget>) {
        let row = gtk4::Box::new(Orientation::Horizontal, 8);
        let lbl = Label::new(Some(label));
        lbl.set_size_request(160, -1);
        lbl.set_halign(Align::Start);
        row.append(&lbl);
        row.append(widget);
        parent.append(&row);
    }

    fn add_time_row(parent: &gtk4::Box, label: &str, spin_h: &SpinButton, spin_m: &SpinButton, unit_h: &str, unit_m: &str) {
        let row = gtk4::Box::new(Orientation::Horizontal, 8);
        let lbl = Label::new(Some(label));
        lbl.set_size_request(160, -1);
        lbl.set_halign(Align::Start);
        row.append(&lbl);
        row.append(spin_h);
        row.append(&Label::new(Some(unit_h)));
        row.append(spin_m);
        row.append(&Label::new(Some(unit_m)));
        parent.append(&row);
    }

    pub fn show(&self) {
        self.window.present();
    }
}
