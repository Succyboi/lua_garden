use std::{f64::consts::TAU, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use mlem_base::{base::mlem_interface::MlemInterface, interface::{param_drag_value::{self, ParamDragValue}, param_toggle::ParamToggle}};
use nih_plug::prelude::ParamSetter;
use nih_plug_egui::egui::{Color32, Context, Response, Ui, Vec2, Vec2b};

use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::LineStyle;
use egui_plot::Plot;
use egui_plot::PlotPoints;

use crate::{consts::PLUGIN_METADATA, distortion::{chomper, clip_lerp, fold, humpback, quant, sine, stepper, waveshaper}, params::SixteenDistParams};

pub struct TidbitInterface {
    params: Arc<SixteenDistParams>,
    time: f64
}

impl TidbitInterface {
    pub fn new(params: Arc<SixteenDistParams>) -> Self {
        return Self {
            params,
            time: 0.0
        };
    }

    fn update_params(&mut self, ui: &mut Ui, _ctx: &Context, setter: &ParamSetter) {
        ui.horizontal(|ui| {
            ui.add(ParamDragValue::for_param(&self.params.one, setter).without_label());
            ui.add(ParamDragValue::for_param(&self.params.two, setter).without_label());
        });

        ui.horizontal(|ui| {
            ui.add(ParamDragValue::for_param(&self.params.three, setter).without_label());
            ui.add(ParamDragValue::for_param(&self.params.four, setter).without_label());
        });

        ui.horizontal(|ui| {
            ui.add(ParamDragValue::for_param(&self.params.five, setter).without_label());
            ui.add(ParamDragValue::for_param(&self.params.six, setter).without_label());
        });

        ui.horizontal(|ui| {
            ui.add(ParamDragValue::for_param(&self.params.seven, setter).without_label());
            ui.add(ParamDragValue::for_param(&self.params.eight, setter).without_label());
        });

        ui.horizontal(|ui| {
            ui.add(ParamDragValue::for_param(&self.params.nine, setter).without_label());
            ui.add(ParamDragValue::for_param(&self.params.ten, setter).without_label());
        });

        ui.horizontal(|ui| {
            ui.add(ParamDragValue::for_param(&self.params.eleven, setter).without_label());
            ui.add(ParamDragValue::for_param(&self.params.twelve, setter).without_label());
        });

        ui.horizontal(|ui| {
            ui.add(ParamDragValue::for_param(&self.params.thirteen, setter).without_label());
            ui.add(ParamDragValue::for_param(&self.params.fourteen, setter).without_label());
        });

        ui.horizontal(|ui| {
            ui.add(ParamDragValue::for_param(&self.params.fifteen, setter).without_label());
            ui.add(ParamDragValue::for_param(&self.params.sixteen, setter).without_label());
        });
    }

    fn update_preview(&mut self, ui: &mut Ui, _ctx: &Context, setter: &ParamSetter) -> Response {
        self.time += ui.input(|i| i.unstable_dt).min(1.0 / 30.0) as f64;
        
        let plot = Plot::new("plot")
            .legend(Legend::default())
            .show_axes(false)
            .show_grid(false)
            .allow_scroll(false)
            .allow_drag(false)
            .show_axes(false);
        
        return plot.show(ui, |plot_ui| {
            plot_ui.line(self.graph());
        })
        .response;
    }

    fn graph(&self) -> Line<'_> {
        Line::new(
            PlotPoints::from_explicit_callback(move |x| { 
                let mut signal = 1.0 * f32::sin(2.0 * x as f32 /*+ self.time as f32*/); // Commented out cool time scrolling

                signal = clip_lerp(signal, self.params.one.value() / 100.0);
                signal = waveshaper(signal, self.params.two.value() / 100.0);
                signal = fold(signal, self.params.three.value() / 100.0);
                signal = quant(signal, self.params.four.value() / 100.0);
                signal = chomper(signal, self.params.five.value() / 100.0);
                signal = sine(signal, self.params.six.value() / 100.0);
                signal = stepper(signal, self.params.seven.value() / 100.0);
                signal = humpback(signal, self.params.eight.value() / 100.0);

                return signal as f64;
            }, 
            .., 
            512),
        )
        .color(Color32::from_hex(PLUGIN_METADATA.window_theme.b_high).expect("Couldn't parse color"))
        .style(LineStyle::Solid)
    }
}

impl MlemInterface<SixteenDistParams> for TidbitInterface {
    fn params(&self) ->  Arc<SixteenDistParams> {
        return self.params.clone();
    }

    fn build(&mut self, _ctx: &Context) { }
    
    fn update_bar(&mut self, ui: &mut Ui, _ctx: &Context, setter: &ParamSetter) { }
    
    fn update_center(&mut self, ui: &mut Ui, _ctx: &Context, setter: &ParamSetter) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                self.update_params(ui, _ctx, setter);
            });
            self.update_preview(ui, _ctx, setter);
        });
    }
}