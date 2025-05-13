use nih_plug_egui::egui::{ self, Ui };
use crate::runtime::parameter::Parameter;

const DEFAULT_DRAG_PIXEL_DISTANCE: f32 = 200.0;
const MIN_DECIMALS: usize = 0;
const MAX_DECIMALS: usize = 2; // TODO Should be variable to step size

impl Parameter {
    // TODO step size
    pub fn draw(&mut self, ui: &mut Ui) {
        let value = self.value.clone();
        ui.add(
            egui::DragValue::new(&mut self.value)
                .speed((self.max - self.min) * (1.0 / DEFAULT_DRAG_PIXEL_DISTANCE))
                .clamp_range(self.min..=self.max)
                .min_decimals(MIN_DECIMALS)
                .max_decimals(MAX_DECIMALS),
        ).on_hover_text(&self.name); // TODO better hover preview

        self.set_changed(value != self.value);
    }
}