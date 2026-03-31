use std::sync::Arc;
use mlem_base::{base::mlem_interface::MlemInterface, interface::{param_drag_value::ParamDragValue, param_toggle::ParamToggle}};
use nih_plug::prelude::ParamSetter;
use nih_plug_egui::egui::{Context, Ui};

use crate::params::MarkovParams;

pub struct StretchInterface {
    params: Arc<MarkovParams>,
}

impl StretchInterface {
    pub fn new(params: Arc<MarkovParams>) -> Self {
        return Self {
            params,
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
    }
    
    fn update_center(&mut self, ui: &mut Ui, _ctx: &Context, setter: &ParamSetter) { 
        ui.horizontal(|ui| {
            ui.add(ParamDragValue::for_param(&self.params.word_count, setter));
        });
    }
}