use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation, Align, ProgressBar};

pub struct EyeExerciseView {
    pub container: GtkBox,
    step_label: Label,
    instruction_label: Label,
    progress_bar: ProgressBar,
    steps: Vec<(&'static str, &'static str, u32)>,
    current_step: usize,
    step_elapsed: u32,
}

impl EyeExerciseView {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 12);
        container.set_halign(Align::Center);
        container.set_valign(Align::Center);

        let step_label = Label::new(Some("Step 1 of 4"));
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

        let instruction_label = Label::new(Some("Look away at an object at least 20 feet (6 meters) away."));
        instruction_label.add_css_class("overlay-instruction");
        instruction_label.set_wrap(true);
        instruction_label.set_justify(gtk4::Justification::Center);
        instruction_label.set_halign(Align::Center);
        container.append(&instruction_label);

        let progress_bar = ProgressBar::new();
        progress_bar.set_size_request(280, 6);
        progress_bar.set_halign(Align::Center);
        container.append(&progress_bar);

        let steps = vec![
            ("Distance Focus", "Look at a distant object 20 feet away to relax your eye focusing muscles.", 5),
            ("Blink Consciously", "Blink slowly 5-10 times to moisten and soothe your eyes.", 5),
            ("Side to Side", "Slowly roll your eyes left to right 3 times.", 5),
            ("Palming", "Rub hands together to generate warmth and gently cup them over your closed eyes.", 5),
        ];

        let mut view = Self {
            container,
            step_label,
            instruction_label,
            progress_bar,
            steps,
            current_step: 0,
            step_elapsed: 0,
        };

        view.update_display();
        view
    }

    pub fn tick(&mut self) {
        if self.current_step >= self.steps.len() {
            return;
        }

        let duration = self.steps[self.current_step].2;
        self.step_elapsed += 1;

        if self.step_elapsed >= duration {
            self.step_elapsed = 0;
            if self.current_step + 1 < self.steps.len() {
                self.current_step += 1;
            } else {
                self.step_elapsed = duration;
            }
        }

        self.update_display();
    }

    fn update_display(&mut self) {
        if let Some((name, desc, duration)) = self.steps.get(self.current_step) {
            self.step_label.set_text(&format!("Step {} of {}: {}", self.current_step + 1, self.steps.len(), name));
            self.instruction_label.set_text(desc);
            let fraction = (self.step_elapsed as f64) / (*duration as f64);
            self.progress_bar.set_fraction(fraction.clamp(0.0, 1.0));
        }
    }
}
