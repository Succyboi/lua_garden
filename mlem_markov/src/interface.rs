use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use mlem_base::{base::mlem_interface::MlemInterface, interface::{param_drag_value::ParamDragValue, param_toggle::ParamToggle}};
use nih_plug::prelude::ParamSetter;
use nih_plug_egui::egui::{Context, Ui};

use crate::params::MarkovParams;

pub struct StretchInterface {
    params: Arc<MarkovParams>
}

impl StretchInterface {
    pub fn new(params: Arc<MarkovParams>) -> Self {
        return Self {
            params
        };
    }
}

impl MlemInterface<MarkovParams> for StretchInterface {
    fn params(&self) ->  Arc<MarkovParams> {
        return self.params.clone();
    }

    fn build(&mut self, _ctx: &Context) { }
    
    fn update_bar(&mut self, ui: &mut Ui, _ctx: &Context, setter: &ParamSetter) {
        ui.add(ParamDragValue::for_param(&self.params.volume, setter));

        if ui.button("???").clicked() {
            //TODO implement randomize all params button
        }
    }
    
    fn update_center(&mut self, ui: &mut Ui, _ctx: &Context, setter: &ParamSetter) { 
        ui.horizontal(|ui| {
            ui.add(ParamDragValue::for_param(&self.params.word_count, setter));
        });

        ui.horizontal(|ui| {
            let buffer_preview = self.params.buffer_preview.lock().unwrap();
            let preview_pos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as usize / 50 % buffer_preview.len(); //  TODO This warrants a proper time thing somewhere in base
            
            ui.monospace(buffer_preview[preview_pos].clone());
        });
    }
}