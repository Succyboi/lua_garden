use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use mlem_base::{base::mlem_interface::MlemInterface, interface::{param_drag_value::{self, ParamDragValue}, param_toggle::ParamToggle}};
use nih_plug::prelude::ParamSetter;
use nih_plug_egui::egui::{Context, Ui};

use crate::params::TidbitParams;

pub struct TidbitInterface {
    params: Arc<TidbitParams>
}

impl TidbitInterface {
    pub fn new(params: Arc<TidbitParams>) -> Self {
        return Self {
            params
        };
    }
}

impl MlemInterface<TidbitParams> for TidbitInterface {
    fn params(&self) ->  Arc<TidbitParams> {
        return self.params.clone();
    }

    fn build(&mut self, _ctx: &Context) { }
    
    fn update_bar(&mut self, ui: &mut Ui, _ctx: &Context, setter: &ParamSetter) {
        ui.add(ParamDragValue::for_param(&self.params.mix, setter));
    }
    
    fn update_center(&mut self, ui: &mut Ui, _ctx: &Context, setter: &ParamSetter) {
        ui.horizontal(|ui| {
            ui.add(ParamDragValue::for_param(&self.params.min_length, setter));
            ui.add(ParamDragValue::for_param(&self.params.max_length, setter));
        });
    }
}