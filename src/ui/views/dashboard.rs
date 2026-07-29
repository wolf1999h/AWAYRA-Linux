use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Label, Button, Grid, Orientation};
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
    pause_btn: Button,
    settings_btn: Button,
}

impl DashboardWindow {
    pub fn new(app: &gtk4::Application, host: Arc<Mutex<AppHost>>) -> Self {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("Awayra"));
        window.set_default_size(460, 660);

        // CSS
        let css = include_str!("../../../resources/style.css");
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(css);
        if let Some(display) = gtk4::gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
        }

        let main_box = gtk4::Box::new(Orientation::Vertical, 12);
        main_box.add_css_class("card");
        main_box.set_margin_top(22);
        main_box.set_margin_bottom(20);
        main_box.set_margin_start(24);
        main_box.set_margin_end(24);

        // Title
        let title = Label::new(Some("Awayra"));
        title.add_css_class("dashboard-title");
        title.add_css_class("title-label");
        main_box.append(&title);

        // Status
        let status_label = Label::new(Some("Starting..."));
        status_label.add_css_class("dashboard-status");
        status_label.add_css_class("subtitle-label");
        main_box.append(&status_label);

        // Countdown cards
        let cards_grid = Grid::new();
        cards_grid.set_column_homogeneous(true);
        cards_grid.set_column_spacing(6);

        let (eye_countdown, eye_state, eye_frame) = Self::make_card("Eye Reset");
        eye_frame.add_css_class("card");
        cards_grid.attach(&eye_frame, 0, 0, 1, 1);

        let (move_countdown, move_state, move_frame) = Self::make_card("Move Break");
        move_frame.add_css_class("card");
        cards_grid.attach(&move_frame, 1, 0, 1, 1);

        main_box.append(&cards_grid);

        // Today statistics
        let stats_grid = Grid::new();
        stats_grid.set_column_homogeneous(true);
        stats_grid.set_row_homogeneous(true);
        stats_grid.set_row_spacing(5);
        stats_grid.set_column_spacing(5);

        let today_eye = Label::new(Some("Eye: 0"));
        today_eye.add_css_class("stat-tile");
        stats_grid.attach(&today_eye, 0, 0, 1, 1);

        let today_move = Label::new(Some("Move: 0"));
        today_move.add_css_class("stat-tile");
        stats_grid.attach(&today_move, 1, 0, 1, 1);

        let today_skipped = Label::new(Some("Skipped: 0"));
        today_skipped.add_css_class("stat-tile");
        stats_grid.attach(&today_skipped, 0, 1, 1, 1);

        let today_snoozed = Label::new(Some("Snoozed: 0"));
        today_snoozed.add_css_class("stat-tile");
        stats_grid.attach(&today_snoozed, 1, 1, 1, 1);

        main_box.append(&stats_grid);

        // Spacer
        let spacer = gtk4::Box::new(Orientation::Vertical, 0);
        spacer.set_hexpand(true);
        spacer.set_vexpand(true);
        main_box.append(&spacer);

        // Action buttons
        let action_grid = Grid::new();
        action_grid.set_column_homogeneous(true);
        action_grid.set_column_spacing(6);
        action_grid.set_margin_top(16);
        action_grid.set_margin_bottom(10);

        let eye_btn = Button::with_label("Eye Reset Now");
        eye_btn.add_css_class("primary-button");
        eye_btn.add_css_class("primary-btn");
        action_grid.attach(&eye_btn, 0, 0, 1, 1);

        let move_btn = Button::with_label("Move Break Now");
        move_btn.add_css_class("primary-button");
        move_btn.add_css_class("primary-btn");
        action_grid.attach(&move_btn, 1, 0, 1, 1);

        main_box.append(&action_grid);

        // Secondary buttons
        let secondary_grid = Grid::new();
        secondary_grid.set_column_homogeneous(true);
        secondary_grid.set_column_spacing(6);

        let pause_btn = Button::with_label("Pause");
        pause_btn.add_css_class("secondary-button");
        pause_btn.add_css_class("primary-btn");
        secondary_grid.attach(&pause_btn, 0, 0, 1, 1);

        let settings_btn = Button::with_label("Settings");
        settings_btn.add_css_class("secondary-button");
        settings_btn.add_css_class("primary-btn");
        secondary_grid.attach(&settings_btn, 1, 0, 1, 1);

        main_box.append(&secondary_grid);

        window.set_child(Some(&main_box));

        // Prevent window from actually closing; hide to tray instead
        window.connect_close_request(|_window| {
            _window.hide();
            gtk4::glib::Propagation::Stop
        });

        // Signals
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
            if let Ok(host_ref) = h4.lock() {
                let settings_win = crate::ui::views::settings::SettingsWindow::new(
                    host_ref.clone(),
                );
                settings_win.show();
            }
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
            pause_btn,
            settings_btn,
        };

        // Start auto-refresh timer
        let host_ref = dashboard.host.clone();
        let status_lbl = dashboard.status_label.clone();
        let eye_cd = dashboard.eye_countdown.clone();
        let move_cd = dashboard.move_countdown.clone();
        let eye_st = dashboard.eye_state.clone();
        let move_st = dashboard.move_state.clone();
        let t_eye = dashboard.today_eye.clone();
        let t_move = dashboard.today_move.clone();
        let t_skip = dashboard.today_skipped.clone();
        let t_snooze = dashboard.today_snoozed.clone();
        let pause = dashboard.pause_btn.clone();

        timeout_add_seconds_local(1, move || {
            if let Ok(host_lock) = host_ref.lock() {
                if let Ok(sched) = host_lock.scheduler.lock() {
                    let snapshot = sched.get_snapshot();
                    let loc = &host_lock.localization;

                    // Status
                    let status_text = loc.get_status(snapshot.status);
                    status_lbl.set_text(&status_text);

                    // Eye countdown
                    let eye_secs = snapshot.eye_remaining.num_seconds().max(0);
                    eye_cd.set_text(&format!("{:02}:{:02}", eye_secs / 60, eye_secs % 60));
                    eye_st.set_text(if snapshot.eye_enabled { "Enabled" } else { "Disabled" });

                    // Move countdown
                    let move_secs = snapshot.move_remaining.num_seconds().max(0);
                    move_cd.set_text(&format!("{:02}:{:02}", move_secs / 60, move_secs % 60));
                    move_st.set_text(if snapshot.move_enabled { "Enabled" } else { "Disabled" });

                    // Pause button
                    pause.set_label(if snapshot.is_paused_manual { "Resume" } else { "Pause" });
                }

                if let Ok(stats) = host_lock.statistics.lock() {
                    let today = stats.get_today();
                    t_eye.set_text(&format!("Eye: {}", today.eye_completed));
                    t_move.set_text(&format!("Move: {}", today.move_completed));
                    t_skip.set_text(&format!("Skipped: {}", today.skipped));
                    t_snooze.set_text(&format!("Snoozed: {}", today.snoozed));
                }
            }
            gtk4::glib::ControlFlow::Continue
        });

        dashboard
    }

    fn make_card(title: &str) -> (Label, Label, gtk4::Frame) {
        let frame = gtk4::Frame::new(None);
        frame.add_css_class("countdown-card");
        frame.add_css_class("card");
        let vbox = gtk4::Box::new(Orientation::Vertical, 4);
        vbox.set_margin_top(12);
        vbox.set_margin_bottom(12);
        vbox.set_margin_start(12);
        vbox.set_margin_end(12);
        let title_lbl = Label::new(Some(title));
        title_lbl.add_css_class("card-label");
        title_lbl.add_css_class("title-label");
        vbox.append(&title_lbl);
        let countdown = Label::new(Some("--:--"));
        countdown.add_css_class("countdown-value");
        countdown.add_css_class("timer-display");
        vbox.append(&countdown);
        let state = Label::new(Some(""));
        state.add_css_class("muted-text");
        state.add_css_class("subtitle-label");
        vbox.append(&state);
        frame.set_child(Some(&vbox));
        (countdown, state, frame)
    }

    pub fn show(&self) {
        self.window.present();
    }
}