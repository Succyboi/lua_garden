use std::sync::Arc;
use nih_plug::prelude::ParamSetter;
use nih_plug_egui::egui::{Context, Ui};
use crate::base::{mlem_params::MlemParams, mlem_plugin::MlemPlugin};

pub trait MlemInterface<T: MlemParams>: 'static + Send + Sync {
    fn params(&self) ->  Arc<T>;

    fn build(&mut self, ctx: &Context);
    fn update_center(&mut self, ui: &mut Ui, ctx: &Context, setter: &ParamSetter);
    fn update_bar(&mut self, ui: &mut Ui, ctx: &Context, setter: &ParamSetter);
}