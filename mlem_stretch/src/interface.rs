use std::sync::Arc;
use mlem_base::{base::mlem_interface::MlemInterface, interface::{param_drag_value::ParamDragValue, param_toggle::ParamToggle}};
use nih_plug::prelude::ParamSetter;
use nih_plug_egui::egui::{Context, Ui};

use crate::params::StretchParams;

const STRETCH_STRING_SOURCE: &str = "Stretch! ";
const MAX_STRETCH_STRING_LENGTH: usize = 24;

pub struct StretchInterface {
    params: Arc<StretchParams>,
    stretch_time: f32
}

impl StretchInterface {
    pub fn new(params: Arc<StretchParams>) -> Self {
        return Self {
            params,

            stretch_time: 0.0
        };
    }

    fn get_stretch_string(&mut self) -> String {
        if self.params.stretch.value() {
            self.stretch_time = self.stretch_time + self.params.speed.value() * 0.2;
        }
        
        let mut string = String::new();
        for c in 0..MAX_STRETCH_STRING_LENGTH {
            let char_i = (f32::floor(c as f32 * self.params.speed.value() + self.stretch_time) as usize) % STRETCH_STRING_SOURCE.len();
            let char = STRETCH_STRING_SOURCE.chars().nth(char_i).expect("Out of bounds");

            string.push(char);
        }

        return string;
    }
}

impl MlemInterface<StretchParams> for StretchInterface {
    fn params(&self) ->  Arc<StretchParams> {
        return self.params.clone();
    }

    fn build(&mut self, ctx: &Context) { }
    
    fn update_bar(&mut self, ui: &mut Ui, ctx: &Context, setter: &ParamSetter) { }
    
    fn update_center(&mut self, ui: &mut Ui, ctx: &Context, setter: &ParamSetter) {
        ui.horizontal(|ui| {
            ui.add(ParamToggle::for_param(&self.params.stretch, setter, "Stretch", "Stretch"));
            ui.add(ParamDragValue::for_param(&self.params.speed, setter));
        });
        
        ui.add_space(8.0);
        
        ui.horizontal(|ui| {
            ui.add(ParamDragValue::for_param(&self.params.variance, setter));
            ui.add(ParamDragValue::for_param(&self.params.window, setter));
        });
        
        ui.horizontal(|ui| {
            ui.add(ParamDragValue::for_param(&self.params.pitch, setter).with_decimals(0));
            ui.add_enabled_ui(!*&self.params.stretch.value(), |ui| {
                ui.add(ParamDragValue::for_param(&self.params.buffer, setter).with_decimals(0));
            });
        });
        
        ui.separator();
        ui.horizontal(|ui| {
            ui.add_enabled_ui(false, |ui| {
                let stretch_string = &self.get_stretch_string();
                ui.monospace(stretch_string);
            });
        });
    }
}