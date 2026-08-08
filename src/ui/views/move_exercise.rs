use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation, Align, ProgressBar};

pub struct MoveExerciseView {
    pub container: GtkBox,
    step_label: Label,
    instruction_label: Label,
    progress_bar: ProgressBar,
    activities: Vec<(&'static str, &'static str, u32)>,
    current_index: usize,
    elapsed: u32,
}

impl MoveExerciseView {
    pub fn new(activity_index: usize) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 12);
        container.set_halign(Align::Center);
        container.set_valign(Align::Center);

        let step_label = Label::new(Some("Activity"));
        step_label.add_css_class("subtitle-label");
        step_label.set_halign(Align::Center);
        container.append(&step_label);

        let icon = Label::new(Some("🚶‍♂️"));
        icon.add_css_class("dashboard-title");
        container.append(&icon);

        let instruction_label = Label::new(Some("Stand up and stretch your arms above your head."));
        instruction_label.add_css_class("overlay-instruction");
        instruction_label.set_wrap(true);
        instruction_label.set_justify(gtk4::Justification::Center);
        instruction_label.set_halign(Align::Center);
        container.append(&instruction_label);

        let progress_bar = ProgressBar::new();
        progress_bar.set_size_request(280, 6);
        progress_bar.set_halign(Align::Center);
        container.append(&progress_bar);

        let activities = vec![
            ("Full Body Stretch", "Stand up straight, reach both arms up toward the ceiling, and stretch your legs.", 60),
            ("Neck & Shoulder Rolls", "Roll your shoulders backward 5 times, then slowly tilt your neck side to side.", 60),
            ("Hydration Break", "Walk to get a fresh glass of water and take deep breaths.", 60),
            ("Torso Twists", "Stand up, place hands on hips, and gently twist your upper body left and right.", 60),
        ];

        let idx = activity_index % activities.len();

        let mut view = Self {
            container,
            step_label,
            instruction_label,
            progress_bar,
            activities,
            current_index: idx,
            elapsed: 0,
        };

        view.update_display();
        view
    }

    pub fn tick(&mut self) {
        self.elapsed += 1;
        self.update_display();
    }

    fn update_display(&mut self) {
        if let Some((name, desc, duration)) = self.activities.get(self.current_index) {
            self.step_label.set_text(name);
            self.instruction_label.set_text(desc);
            let fraction = (self.elapsed as f64) / (*duration as f64);
            self.progress_bar.set_fraction(fraction.clamp(0.0, 1.0));
        }
    }
}
