use std::sync::{Arc, atomic::Ordering};
use mlem_base::{base::mlem_interface::MlemInterface, interface::{param_drag_value::ParamDragValue, param_toggle::{self, ParamToggle}, utils::{parameter_grid, parameter_label}}};
use nih_plug::prelude::ParamSetter;
use nih_plug_egui::egui::{Context, Ui};

use crate::params::MeterParams;

const STRETCH_STRING_SOURCE: &str = "Stretch! ";
const MAX_STRETCH_STRING_LENGTH: usize = 24;

pub struct MeterInterface {
    params: Arc<MeterParams>,
}

impl MeterInterface {
    pub fn new(params: Arc<MeterParams>) -> Self {
        return Self {
            params
        };
    }
}

impl MlemInterface<MeterParams> for MeterInterface {
    fn params(&self) ->  Arc<MeterParams> {
        return self.params.clone();
    }

    fn build(&mut self, ctx: &Context) { }
    
    fn update_bar(&mut self, ui: &mut Ui, ctx: &Context, setter: &ParamSetter) {
        let seconds = self.params.active_time_ms.load(Ordering::Relaxed) / 1000.0;
        let minutes = f32::floor(seconds / 60.0);
        
        if ui.button("Reset").clicked() {
            self.params.reset_meter.store(true, Ordering::Relaxed);
        }
        ui.monospace(format!("{minutes: >1.0}m{seconds: >1.0}s", minutes = minutes, seconds = seconds - minutes * 60.0));   
    }
    
    fn update_center(&mut self, ui: &mut Ui, ctx: &Context, setter: &ParamSetter) {
        parameter_grid(ui, "Meters", |ui| {
            parameter_label(ui, "Integrated", "Loudness total since reset.", |ui| {
                ui.monospace(format!("{: >6.2} lufs", self.params.lufs_global_loudness.load(Ordering::Relaxed)));
            });

            parameter_label(ui, "Momentary", "Loudness over a duration of 0.4 seconds.", |ui| {
                ui.monospace(format!("{: >6.2} lufs", self.params.lufs_momentary_loudness.load(Ordering::Relaxed)));
            });

            parameter_label(ui, "Short Term", "Loudness over a duration of 3 seconds.", |ui| {
                ui.monospace(format!("{: >6.2} lufs", self.params.lufs_shortterm_loudness.load(Ordering::Relaxed)));
            });

            parameter_label(ui, "Range", "Loudness range total since reset.", |ui| {
                ui.monospace(format!("{: >6.2} lufs", self.params.lufs_range_loudness.load(Ordering::Relaxed)));
            });

            parameter_label(ui, "Reset On Play", "Resets metering when starting play.", |ui| {
                ui.add(param_toggle::ParamToggle::for_param(&self.params.reset_on_play, setter, "Yes", "No"));
            });
        });
    }
}