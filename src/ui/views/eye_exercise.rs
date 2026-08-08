use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation, Align, ProgressBar};
use std::sync::Arc;
use crate::core::localization::LocalizationService;

pub struct EyeExerciseView {
    pub container: GtkBox,
    step_label: Label,
    instruction_label: Label,
    progress_bar: ProgressBar,
    loc: Arc<LocalizationService>,
    current_step: usize,
    step_elapsed: u32,
}

impl EyeExerciseView {
    pub fn new(loc: Arc<LocalizationService>) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 12);
        container.set_halign(Align::Center);
        container.set_valign(Align::Center);

        let step_label = Label::new(None);
        step_label.add_css_class("subtitle-label");
        step_label.set_halign(Align::Center);
        container.append(&step_label);

        let animation_box = GtkBox::new(Orientation::Vertical, 0);
        animation_box.set_halign(Align::Center);
        animation_box.set_valign(Align::Center);

        let eye_icon = Label::new(Some("👀"));
        eye_icon.add_css_class("dashboard-title");
        animation_box.append(&eye_icon);
        container.append(&animation_box);

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

        let mut view = Self {
            container,
            step_label,
            instruction_label,
            progress_bar,
            loc,
            current_step: 0,
            step_elapsed: 0,
        };

        view.update_display();
        view
    }

    fn get_step_info(&self, index: usize) -> Option<(String, String, u32)> {
        match index {
            0 => Some((
                self.loc.get("EyeStep1Title"),
                self.loc.get("EyeStep1Desc"),
                5,
            )),
            1 => Some((
                self.loc.get("EyeStep2Title"),
                self.loc.get("EyeStep2Desc"),
                5,
            )),
            2 => Some((
                self.loc.get("EyeStep3Title"),
                self.loc.get("EyeStep3Desc"),
                5,
            )),
            3 => Some((
                self.loc.get("EyeStep4Title"),
                self.loc.get("EyeStep4Desc"),
                5,
            )),
            _ => None,
        }
    }

    pub fn tick(&mut self) {
        if self.current_step >= 4 {
            return;
        }

        let duration = self.get_step_info(self.current_step).map(|s| s.2).unwrap_or(5);
        self.step_elapsed += 1;

        if self.step_elapsed >= duration {
            self.step_elapsed = 0;
            if self.current_step + 1 < 4 {
                self.current_step += 1;
            } else {
                self.step_elapsed = duration;
            }
        }

        self.update_display();
    }

    fn update_display(&mut self) {
        if let Some((name, desc, duration)) = self.get_step_info(self.current_step) {
            let progress_tmpl = self.loc.get("StepProgress");
            let step_str = progress_tmpl
                .replace("{0}", &(self.current_step + 1).to_string())
                .replace("{1}", "4")
                .replace("{2}", &name);
            self.step_label.set_text(&step_str);
            self.instruction_label.set_text(&desc);
            let fraction = (self.step_elapsed as f64) / (duration as f64);
            self.progress_bar.set_fraction(fraction.clamp(0.0, 1.0));
        }
    }
}
