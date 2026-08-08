use gtk4::prelude::*;
use gtk4::{Window, Label, Button, Box as GtkBox, Orientation, Align};

pub struct AboutWindow {
    window: Window,
}

impl AboutWindow {
    pub fn new() -> Self {
        let window = Window::new();
        window.set_title(Some("About Awayra"));
        window.set_default_size(400, 480);
        window.set_resizable(false);

        let vbox = GtkBox::new(Orientation::Vertical, 16);
        vbox.set_margin_top(24);
        vbox.set_margin_bottom(24);
        vbox.set_margin_start(24);
        vbox.set_margin_end(24);
        vbox.set_halign(Align::Center);
        vbox.set_valign(Align::Center);

        let icon_label = Label::new(Some("🧘"));
        icon_label.add_css_class("dashboard-title");
        vbox.append(&icon_label);

        let app_name = Label::new(Some("Awayra"));
        app_name.add_css_class("title-label");
        vbox.append(&app_name);

        let version = Label::new(Some("Version 1.0.0 (Linux Rust Port)"));
        version.add_css_class("subtitle-label");
        vbox.append(&version);

        let desc = Label::new(Some("A calm, non-intrusive break reminder to help reduce eye strain and promote healthy movement habits during long computer sessions."));
        desc.set_wrap(true);
        desc.set_justify(gtk4::Justification::Center);
        desc.add_css_class("muted-text");
        vbox.append(&desc);

        let btn_box = GtkBox::new(Orientation::Horizontal, 12);
        btn_box.set_halign(Align::Center);

        let github_btn = Button::with_label("GitHub");
        github_btn.add_css_class("secondary-button");
        btn_box.append(&github_btn);

        let close_btn = Button::with_label("Close");
        close_btn.add_css_class("primary-btn");
        btn_box.append(&close_btn);

        vbox.append(&btn_box);
        window.set_child(Some(&vbox));

        github_btn.connect_clicked(|_| {
            let _ = gtk4::gio::AppInfo::launch_default_for_uri(
                "https://github.com",
                None::<&gtk4::gio::AppLaunchContext>,
            );
        });

        let w = window.clone();
        close_btn.connect_clicked(move |_| w.close());

        Self { window }
    }

    pub fn show(&self) {
        self.window.present();
    }
}
