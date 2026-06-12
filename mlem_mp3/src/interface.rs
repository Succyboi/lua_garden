use std::{f64::consts::TAU, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use mlem_base::{base::mlem_interface::MlemInterface, interface::{param_drag_value::{self, ParamDragValue}, param_toggle::ParamToggle, utils}};
use nih_plug::prelude::ParamSetter;
use nih_plug_egui::egui::{Color32, Context, Layout, Response, Ui, Vec2, Vec2b};

use crate::{params::MP3Params};

pub struct TidbitInterface {
    params: Arc<MP3Params>
}

impl TidbitInterface {
    pub fn new(params: Arc<MP3Params>) -> Self {
        return Self {
            params
        };
    }
}

impl MlemInterface<MP3Params> for TidbitInterface {
    fn params(&self) ->  Arc<MP3Params> {
        return self.params.clone();
    }

    fn build(&mut self, _ctx: &Context) { }
    
    fn update_bar(&mut self, ui: &mut Ui, _ctx: &Context, setter: &ParamSetter) {
        ui.with_layout(Layout::right_to_left(nih_plug_egui::egui::Align::Center), |ui| {
            ui.add(ParamDragValue::for_param(&self.params.mix, setter).without_label());
            utils::fill_seperator_available(ui);
        });
    }
    
    fn update_center(&mut self, ui: &mut Ui, _ctx: &Context, setter: &ParamSetter) {

    }
}