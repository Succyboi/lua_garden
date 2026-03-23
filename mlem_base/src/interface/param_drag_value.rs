use std::{any::{Any, TypeId}, usize};
use nih_plug_egui::egui::{
    self, DragValue, Key, Response, Sense, Stroke, TextEdit, TextStyle, Ui, Vec2, Widget, WidgetText, emath::{self, Float}, vec2
};
use nih_plug::{params::IntParam, prelude::{FloatParam, Param, ParamSetter}};
use crate::interface::{PARAM_WIDTH, utils::{self, param_info, param_label_short}};

const DRAG_SPEED_DEFAULT: f32 = 0.01;
const DRAG_SPEED_GRANULAR: f32 = 0.0001;
const DEFAULT_MAX_DECIMALS: usize = 2;

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct ParamDragValue<'a, P: Param> {
    param: &'a P,
    setter: &'a ParamSetter<'a>,

    decimals: usize,
    fixed: bool,
    draw_label: bool,
    draw_unit: bool,

    /// Will be set in the `ui()` function so we can request keyboard input focus on Alt+click.
    keyboard_focus_id: Option<egui::Id>,
}

impl<'a, P: Param> ParamDragValue<'a, P> {
    /// Create a new slider for a parameter. Use the other methods to modify the slider before
    /// passing it to [`Ui::add()`].
    pub fn for_param(param: &'a P, setter: &'a ParamSetter<'a>) -> Self {
        Self {
            param,
            setter,

            decimals: DEFAULT_MAX_DECIMALS,
            fixed: true,
            draw_label: true,
            draw_unit: true,

            keyboard_focus_id: None,
        }
    }

    pub fn with_decimals(mut self, decimals: usize) -> Self {
        self.decimals = decimals;
        self
    }

    /// Don't draw the text slider's current value after the slider.
    pub fn without_unit(mut self) -> Self {
        self.draw_unit = false;
        self
    }

    pub fn without_label(mut self) -> Self {
        self.draw_label = false;
        self
    }

    pub fn fill(mut self, fill: bool) -> Self {
        self.fixed = fill;
        self
    }

    fn plain_value(&self) -> P::Plain {
        self.param.modulated_plain_value()
    }

    fn normalized_value(&self) -> f32 {
        self.param.modulated_normalized_value()
    }

    fn begin_drag(&self) {
        self.setter.begin_set_parameter(self.param);
    }

    fn set_normalized_value(&self, normalized: f32) {
        // This snaps to the nearest plain value if the parameter is stepped in some way.
        // TODO: As an optimization, we could add a `const CONTINUOUS: bool` to the parameter to
        //       avoid this normalized->plain->normalized conversion for parameters that don't need
        //       it
        let value = self.param.preview_plain(normalized);
        if value != self.plain_value() {
            self.setter.set_parameter(self.param, value);
        }
    }

    /// Begin and end drag still need to be called when using this..
    fn reset_param(&self) {
        self.setter
            .set_parameter(self.param, self.param.default_plain_value());
    }

    fn end_drag(&self) {
        self.setter.end_set_parameter(self.param);
    }
}

//TODO figure out how to compress this implementation for floatparam, intparam.
impl Widget for ParamDragValue<'_, FloatParam> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            if self.fixed {
                ui.set_width(PARAM_WIDTH);
            }

            // Allocate an automatic ID for keeping track of keyboard focus state
            // FIXME: There doesn't seem to be a way to generate IDs in the public API, not sure how
            //        you're supposed to do this
            let (kb_edit_id, _) = ui.allocate_space(vec2(0.0, 0.0));
            self.keyboard_focus_id = Some(kb_edit_id);

            let min = self.param.range().unnormalize(0.0);
            let max = self.param.range().unnormalize(1.0);
            let range = max - min;
            let original_value = self.param.range().unnormalize(self.normalized_value());
            let mut value = original_value;
            let drag_speed = if ui.input(|i| i.modifiers.shift) { DRAG_SPEED_GRANULAR } else { DRAG_SPEED_DEFAULT };
            let unit = if self.draw_unit { self.param.unit() } else { "" };
            let response = ui.add(
                DragValue::new(&mut value)
                .speed(range as f32 * drag_speed)
                .range(min..=max)
                .max_decimals(self.decimals)
                .update_while_editing(false)
                .suffix(unit)
            );
            
            if self.draw_label && !response.has_focus() && !response.lost_focus() {
                param_label_short(ui, self.param);
            }

            if self.fixed {
                utils::fill_seperator_available(ui);
            }

            if value != original_value {
                self.begin_drag();
                self.set_normalized_value(self.param.range().normalize(value));
                self.end_drag();
            }

            if response.secondary_clicked() || response.middle_clicked() {
                self.begin_drag();
                self.reset_param();
                self.end_drag();
            }

            response.on_hover_ui(|ui| {
                param_info(ui, self.param);
            })
        })
        .inner
    }
}

impl Widget for ParamDragValue<'_, IntParam> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            if self.fixed {
                ui.set_width(PARAM_WIDTH);
            }

            // Allocate an automatic ID for keeping track of keyboard focus state
            // FIXME: There doesn't seem to be a way to generate IDs in the public API, not sure how
            //        you're supposed to do this
            let (kb_edit_id, _) = ui.allocate_space(vec2(0.0, 0.0));
            self.keyboard_focus_id = Some(kb_edit_id);

            let min = self.param.range().unnormalize(0.0);
            let max = self.param.range().unnormalize(1.0);
            let range = max - min;
            let original_value = self.param.range().unnormalize(self.normalized_value());
            let mut value = original_value;
            let drag_speed = if ui.input(|i| i.modifiers.shift) { DRAG_SPEED_GRANULAR } else { DRAG_SPEED_DEFAULT };
            let unit = if self.draw_unit { self.param.unit() } else { "" };
            let response = ui.add(
                DragValue::new(&mut value)
                .speed(range as f32 * drag_speed)
                .range(min..=max)
                .max_decimals(0)
                .update_while_editing(false)
                .suffix(unit)
            );

            if self.draw_label && !response.has_focus() && !response.lost_focus() {
                param_label_short(ui, self.param);
            }

            if self.fixed {
                utils::fill_seperator_available(ui);
            }

            if value != original_value {
                self.begin_drag();
                self.set_normalized_value(self.param.range().normalize(value));
                self.end_drag();
            }

            if response.secondary_clicked() || response.middle_clicked() {
                self.begin_drag();
                self.reset_param();
                self.end_drag();
            }

            response.on_hover_ui(|ui| {
                param_info(ui, self.param);
            })
        })
        .inner
    }
}