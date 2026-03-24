use nih_plug::prelude::ParamSetter;
use nih_plug_egui::egui::{Context, Ui};

pub trait MlemInterface {
    fn build(&self, ctx: &Context);
    fn update_center(&self, ui: &mut Ui, ctx: &Context, setter: &ParamSetter);
    fn update_bar(&self, ui: &mut Ui, ctx: &Context, setter: &ParamSetter);
}