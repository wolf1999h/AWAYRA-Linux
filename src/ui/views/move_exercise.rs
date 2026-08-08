use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation, Align, ProgressBar};
use std::sync::Arc;
use crate::core::localization::LocalizationService;

pub struct MoveExerciseView {
    pub container: GtkBox,
    step_label: Label,
    instruction_label: Label,
    progress_bar: ProgressBar,
    loc: Arc<LocalizationService>,
    current_index: usize,
    elapsed: u32,
}

impl MoveExerciseView {
    pub fn new(activity_index: usize, loc: Arc<LocalizationService>) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 12);
        container.set_halign(Align::Center);
        container.set_valign(Align::Center);

        let step_label = Label::new(None);
        step_label.add_css_class("subtitle-label");
        step_label.set_halign(Align::Center);
        container.append(&step_label);

        let icon = Label::new(Some("🚶‍♂️"));
        icon.add_css_class("dashboard-title");
        container.append(&icon);

        let instruction_label = Label::new(None);
        instruction_label.add_css_class("overlay-instruction");
        instruction_label.set_wrap(true);
        instruction_label.set_justify(gtk4::Justification::Center);
        instruction_label.set_halign(Align::Center);
        container.append(&instruction_label);

        let progress_bar = ProgressBar::new();
        progress_bar.set_size_request(280, 6);
        progress_bar.set_halign(Align::Center);
        container.append(&progress_bar);

        let idx = activity_index % 4;

        let mut view = Self {
            container,
            step_label,
            instruction_label,
            progress_bar,
            loc,
            current_index: idx,
            elapsed: 0,
        };

        view.update_display();
        view
    }

    fn get_activity_info(&self, index: usize) -> (String, String, u32) {
        match index {
            0 => (
                self.loc.get("MoveStep1Title"),
                self.loc.get("MoveStep1Desc"),
                60,
            ),
            1 => (
                self.loc.get("MoveStep2Title"),
                self.loc.get("MoveStep2Desc"),
                60,
            ),
            2 => (
                self.loc.get("MoveStep3Title"),
                self.loc.get("MoveStep3Desc"),
                60,
            ),
            _ => (
                self.loc.get("MoveStep4Title"),
                self.loc.get("MoveStep4Desc"),
                60,
            ),
        }
    }

    pub fn tick(&mut self) {
        self.elapsed += 1;
        self.update_display();
    }

    fn update_display(&mut self) {
        let (name, desc, duration) = self.get_activity_info(self.current_index);
        self.step_label.set_text(&name);
        self.instruction_label.set_text(&desc);
        let fraction = (self.elapsed as f64) / (duration as f64);
        self.progress_bar.set_fraction(fraction.clamp(0.0, 1.0));
    }
}
