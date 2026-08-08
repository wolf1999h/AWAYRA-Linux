use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Label, Button, Grid, Orientation, Box as GtkBox};
use gtk4::glib::timeout_add_seconds_local;
use std::sync::{Arc, Mutex};

use crate::core::models::BreakType;
use crate::ui::services::app_host::AppHost;

pub struct DashboardWindow {
    pub window: ApplicationWindow,
    host: Arc<Mutex<AppHost>>,
    status_label: Label,
    eye_countdown: Label,
    move_countdown: Label,
    eye_state: Label,
    move_state: Label,
    today_eye: Label,
    today_move: Label,
    today_skipped: Label,
    today_snoozed: Label,
    eye_card_title: Label,
    move_card_title: Label,
    eye_btn: Button,
    move_btn: Button,
    pause_btn: Button,
    settings_btn: Button,
    about_title: Label,
    about_sub: Label,
}

impl DashboardWindow {
    pub fn new(app: &gtk4::Application, host: Arc<Mutex<AppHost>>) -> Self {
        let host_lock = host.lock().unwrap();
        let loc = host_lock.localization.clone();
        drop(host_lock);

        let window = ApplicationWindow::new(app);
        window.set_title(Some(&loc.get("AppTitle")));
        window.set_default_size(460, 710);
        window.set_resizable(true);
        window.set_icon_name(Some("com.awayra.Awayra"));

        let main_box = GtkBox::new(Orientation::Vertical, 0);
        main_box.set_margin_top(22);
        main_box.set_margin_bottom(20);
        main_box.set_margin_start(24);
        main_box.set_margin_end(24);

        // Header Section
        let header_container = GtkBox::new(Orientation::Horizontal, 12);
        header_container.set_margin_bottom(20);

        // Icon
        let icon = gtk4::Image::from_file("resources/icons/awayra.png");
        icon.set_pixel_size(48);
        header_container.append(&icon);

        let header_text_box = GtkBox::new(Orientation::Vertical, 6);

        let title = Label::new(Some(&loc.get("AppTitle")));
        title.add_css_class("title-label");
        title.set_halign(gtk4::Align::Start);
        header_text_box.append(&title);

        let status_label = Label::new(Some(&loc.get("Starting")));
        status_label.add_css_class("subtitle-label");
        status_label.set_halign(gtk4::Align::Start);
        header_text_box.append(&status_label);

        header_container.append(&header_text_box);
        main_box.append(&header_container);

        // Countdown Cards Row (Eye Reset & Move Break)
        let cards_grid = Grid::new();
        cards_grid.set_column_homogeneous(true);
        cards_grid.set_column_spacing(12);
        cards_grid.set_margin_bottom(16);

        let (eye_countdown, eye_state, eye_card_title, eye_frame) = Self::make_card(&loc.get("CardEyeReset"), "card-accent-eye");
        cards_grid.attach(&eye_frame, 0, 0, 1, 1);

        let (move_countdown, move_state, move_card_title, move_frame) = Self::make_card(&loc.get("CardMoveBreak"), "card-accent-move");
        cards_grid.attach(&move_frame, 1, 0, 1, 1);

        main_box.append(&cards_grid);

        // Today Statistics Card
        let stats_card = GtkBox::new(Orientation::Vertical, 0);
        stats_card.add_css_class("card");
        stats_card.set_margin_bottom(16);

        let stats_grid = Grid::new();
        stats_grid.set_column_homogeneous(true);
        stats_grid.set_row_homogeneous(true);
        stats_grid.set_row_spacing(6);
        stats_grid.set_column_spacing(6);

        let today_eye = Label::new(Some(&format!("{}: 0", loc.get("TodayEyeCompleted"))));
        today_eye.add_css_class("stat-tile");
        stats_grid.attach(&today_eye, 0, 0, 1, 1);

        let today_move = Label::new(Some(&format!("{}: 0", loc.get("TodayMoveCompleted"))));
        today_move.add_css_class("stat-tile");
        stats_grid.attach(&today_move, 1, 0, 1, 1);

        let today_skipped = Label::new(Some(&format!("{}: 0", loc.get("TodaySkipped"))));
        today_skipped.add_css_class("stat-tile");
        stats_grid.attach(&today_skipped, 0, 1, 1, 1);

        let today_snoozed = Label::new(Some(&format!("{}: 0", loc.get("TodaySnoozed"))));
        today_snoozed.add_css_class("stat-tile");
        stats_grid.attach(&today_snoozed, 1, 1, 1, 1);

        stats_card.append(&stats_grid);
        main_box.append(&stats_card);

        // Spacer
        let spacer = GtkBox::new(Orientation::Vertical, 0);
        spacer.set_vexpand(true);
        main_box.append(&spacer);

        // Primary Action Buttons
        let action_grid = Grid::new();
        action_grid.set_column_homogeneous(true);
        action_grid.set_column_spacing(12);
        action_grid.set_margin_bottom(10);

        let eye_btn = Button::with_label(&loc.get("EyeResetNow"));
        eye_btn.add_css_class("primary-button");
        action_grid.attach(&eye_btn, 0, 0, 1, 1);

        let move_btn = Button::with_label(&loc.get("MoveBreakNow"));
        move_btn.add_css_class("primary-button");
        action_grid.attach(&move_btn, 1, 0, 1, 1);

        main_box.append(&action_grid);

        // Secondary Buttons
        let secondary_grid = Grid::new();
        secondary_grid.set_column_homogeneous(true);
        secondary_grid.set_column_spacing(12);
        secondary_grid.set_margin_bottom(12);

        let pause_btn = Button::with_label(&loc.get("Pause"));
        pause_btn.add_css_class("secondary-button");
        secondary_grid.attach(&pause_btn, 0, 0, 1, 1);

        let settings_btn = Button::with_label(&loc.get("Settings"));
        settings_btn.add_css_class("secondary-button");
        secondary_grid.attach(&settings_btn, 1, 0, 1, 1);

        main_box.append(&secondary_grid);

        // About & Support Awayra Bottom Button
        let about_card_btn = Button::new();
        about_card_btn.add_css_class("about-support-button");

        let about_vbox = GtkBox::new(Orientation::Vertical, 2);
        let about_title = Label::new(Some(&loc.get("AboutSupportTitle")));
        about_title.add_css_class("about-support-title");
        about_title.set_halign(gtk4::Align::Start);

        let about_sub = Label::new(Some(&loc.get("AboutSupportSub")));
        about_sub.add_css_class("about-support-sub");
        about_sub.set_halign(gtk4::Align::Start);
        about_sub.set_wrap(true);

        about_vbox.append(&about_title);
        about_vbox.append(&about_sub);
        about_card_btn.set_child(Some(&about_vbox));

        main_box.append(&about_card_btn);

        window.set_child(Some(&main_box));

        // Prevent window close; hide to tray instead
        window.connect_close_request(|_window| {
            _window.hide();
            gtk4::glib::Propagation::Stop
        });

        // Click handlers
        let h = host.clone();
        eye_btn.connect_clicked(move |_| {
            if let Ok(host_ref) = h.lock() {
                if let Ok(mut sched) = host_ref.scheduler.lock() {
                    sched.trigger_now(BreakType::Eye);
                }
            }
        });

        let h2 = host.clone();
        move_btn.connect_clicked(move |_| {
            if let Ok(host_ref) = h2.lock() {
                if let Ok(mut sched) = host_ref.scheduler.lock() {
                    sched.trigger_now(BreakType::Move);
                }
            }
        });

        let h3 = host.clone();
        pause_btn.connect_clicked(move |_| {
            if let Ok(host_ref) = h3.lock() {
                if let Ok(mut sched) = host_ref.scheduler.lock() {
                    let snapshot = sched.get_snapshot();
                    if snapshot.is_paused_manual {
                        sched.resume();
                    } else {
                        sched.pause();
                    }
                }
            }
        });

        let h4 = host.clone();
        settings_btn.connect_clicked(move |_| {
            let settings_win = crate::ui::views::settings::SettingsWindow::new(
                h4.clone(),
            );
            settings_win.show();
        });

        let h_about = host.clone();
        about_card_btn.connect_clicked(move |_| {
            let loc = h_about.lock().unwrap().localization.clone();
            let about_win = crate::ui::views::about::AboutWindow::new(loc);
            about_win.show();
        });

        let dashboard = Self {
            window,
            host,
            status_label,
            eye_countdown,
            move_countdown,
            eye_state,
            move_state,
            today_eye,
            today_move,
            today_skipped,
            today_snoozed,
            eye_card_title,
            move_card_title,
            eye_btn,
            move_btn,
            pause_btn,
            settings_btn,
            about_title,
            about_sub,
        };

        // Refresh timer
        let host_ref = dashboard.host.clone();
        let win_lbl = dashboard.window.clone();
        let status_lbl = dashboard.status_label.clone();
        let eye_cd = dashboard.eye_countdown.clone();
        let move_cd = dashboard.move_countdown.clone();
        let eye_st = dashboard.eye_state.clone();
        let move_st = dashboard.move_state.clone();
        let eye_ct = dashboard.eye_card_title.clone();
        let move_ct = dashboard.move_card_title.clone();
        let t_eye = dashboard.today_eye.clone();
        let t_move = dashboard.today_move.clone();
        let t_skip = dashboard.today_skipped.clone();
        let t_snooze = dashboard.today_snoozed.clone();
        let eye_b = dashboard.eye_btn.clone();
        let move_b = dashboard.move_btn.clone();
        let pause_b = dashboard.pause_btn.clone();
        let settings_b = dashboard.settings_btn.clone();
        let ab_title = dashboard.about_title.clone();
        let ab_sub = dashboard.about_sub.clone();

        timeout_add_seconds_local(1, move || {
            if let Ok(host_lock) = host_ref.lock() {
                let loc = &host_lock.localization;

                win_lbl.set_title(Some(&loc.get("AppTitle")));
                eye_ct.set_text(&loc.get("CardEyeReset"));
                move_ct.set_text(&loc.get("CardMoveBreak"));
                eye_b.set_label(&loc.get("EyeResetNow"));
                move_b.set_label(&loc.get("MoveBreakNow"));
                settings_b.set_label(&loc.get("Settings"));
                ab_title.set_text(&loc.get("AboutSupportTitle"));
                ab_sub.set_text(&loc.get("AboutSupportSub"));

                if let Ok(sched) = host_lock.scheduler.lock() {
                    let snapshot = sched.get_snapshot();

                    status_lbl.set_text(&loc.get_status(snapshot.status));

                    let eye_secs = snapshot.eye_remaining.num_seconds().max(0);
                    eye_cd.set_text(&format!("{:02}:{:02}", eye_secs / 60, eye_secs % 60));
                    eye_st.set_text(&loc.get(if snapshot.eye_enabled { "Enabled" } else { "Disabled" }));

                    let move_secs = snapshot.move_remaining.num_seconds().max(0);
                    move_cd.set_text(&format!("{:02}:{:02}", move_secs / 60, move_secs % 60));
                    move_st.set_text(&loc.get(if snapshot.move_enabled { "Enabled" } else { "Disabled" }));

                    pause_b.set_label(&loc.get(if snapshot.is_paused_manual { "Resume" } else { "Pause" }));
                }

                if let Ok(stats) = host_lock.statistics.lock() {
                    let today = stats.get_today();
                    t_eye.set_text(&format!("{}: {}", loc.get("TodayEyeCompleted"), today.eye_completed));
                    t_move.set_text(&format!("{}: {}", loc.get("TodayMoveCompleted"), today.move_completed));
                    t_skip.set_text(&format!("{}: {}", loc.get("TodaySkipped"), today.skipped));
                    t_snooze.set_text(&format!("{}: {}", loc.get("TodaySnoozed"), today.snoozed));
                }
            }
            gtk4::glib::ControlFlow::Continue
        });

        dashboard
    }

    fn make_card(title: &str, accent_class: &str) -> (Label, Label, Label, GtkBox) {
        let card = GtkBox::new(Orientation::Horizontal, 0);
        card.add_css_class("countdown-card");
        card.add_css_class("card");

        let accent_bar = GtkBox::new(Orientation::Vertical, 0);
        accent_bar.add_css_class(accent_class);
        card.append(&accent_bar);

        let vbox = GtkBox::new(Orientation::Vertical, 4);
        vbox.set_hexpand(true);

        let title_lbl = Label::new(Some(title));
        title_lbl.add_css_class("card-label");
        title_lbl.set_halign(gtk4::Align::Start);
        vbox.append(&title_lbl);

        let countdown = Label::new(Some("--:--"));
        countdown.add_css_class("countdown-value");
        countdown.set_halign(gtk4::Align::Start);
        vbox.append(&countdown);

        let state = Label::new(Some(""));
        state.add_css_class("muted-text");
        state.set_halign(gtk4::Align::Start);
        vbox.append(&state);

        card.append(&vbox);
        (countdown, state, title_lbl, card)
    }

    pub fn show(&self) {
        self.window.present();
    }
}
