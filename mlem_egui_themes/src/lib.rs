mod themes;

use egui::{Color32, TextWrapMode};
pub use themes::*;
use std::sync::Arc;
use nih_plug_egui::egui::{self, epaint, style };
use crate::epaint::{ FontFamily::Proportional, FontId };
use crate::style::TextStyle::{ Button, Monospace, Heading, Body, Small };

const COLOR_PARSING_ERROR: &str = "Couldn't parse hex color.";

pub fn set_theme(ctx: &egui::Context, theme: Theme) {
    theme.set_visuals(ctx);
    theme.set_font(ctx);
}

fn make_widget_visual(
    old: style::WidgetVisuals,
    theme: &Theme,
    bg_fill: egui::Color32,
) -> style::WidgetVisuals {
    style::WidgetVisuals {
        bg_fill,
        weak_bg_fill: bg_fill,
        bg_stroke: egui::Stroke {
            color: Color32::from_hex(theme.b_med).expect(COLOR_PARSING_ERROR),
            ..old.bg_stroke
        },
        fg_stroke: egui::Stroke {
            color: Color32::from_hex(theme.f_high).expect(COLOR_PARSING_ERROR),
            ..old.fg_stroke
        },
        ..old
    }
}

impl Theme {
    pub fn set_visuals(&self, ctx: &egui::Context) {
        let old = ctx.style().visuals.clone();

        let visuals = egui::Visuals {
            hyperlink_color: Color32::from_hex(self.f_med).expect(COLOR_PARSING_ERROR),
            faint_bg_color: Color32::from_hex(self.b_low).expect(COLOR_PARSING_ERROR),
            extreme_bg_color: Color32::from_hex(self.b_low).expect(COLOR_PARSING_ERROR),
            code_bg_color: Color32::from_hex(self.b_low).expect(COLOR_PARSING_ERROR),
            warn_fg_color: Color32::from_hex(self.f_low).expect(COLOR_PARSING_ERROR),
            error_fg_color: Color32::from_hex(self.f_inv).expect(COLOR_PARSING_ERROR),
            window_fill: Color32::from_hex(self.background).expect(COLOR_PARSING_ERROR),
            panel_fill: Color32::from_hex(self.background).expect(COLOR_PARSING_ERROR),
            window_stroke: egui::Stroke {
                color: Color32::from_hex(self.b_low).expect(COLOR_PARSING_ERROR),
                ..old.window_stroke
            },
            widgets: style::Widgets {
                noninteractive: make_widget_visual(old.widgets.noninteractive, self, Color32::from_hex(self.b_low).expect(COLOR_PARSING_ERROR)),
                inactive: make_widget_visual(old.widgets.inactive, self, Color32::from_hex(self.b_low).expect(COLOR_PARSING_ERROR)),
                hovered: make_widget_visual(old.widgets.hovered, self, Color32::from_hex(self.b_med).expect(COLOR_PARSING_ERROR)),
                active: make_widget_visual(old.widgets.active, self, Color32::from_hex(self.b_high).expect(COLOR_PARSING_ERROR)),
                open: make_widget_visual(old.widgets.open, self, Color32::from_hex(self.b_low).expect(COLOR_PARSING_ERROR)),
            },
            selection: style::Selection {
                bg_fill: Color32::from_hex(self.b_inv).expect(COLOR_PARSING_ERROR).linear_multiply(self.selection_opacity),
                stroke: egui::Stroke {
                    color: Color32::from_hex(self.b_low).expect(COLOR_PARSING_ERROR),
                    ..old.selection.stroke
                },
            },

            window_shadow: epaint::Shadow {
                color: old.window_shadow.color.linear_multiply(self.shadow_opacity),
                ..old.window_shadow
            },
            popup_shadow: epaint::Shadow {
                color: old.window_shadow.color.linear_multiply(self.shadow_opacity),
                ..old.popup_shadow
            },
            dark_mode: self.darkmode,
            ..old
        };

        ctx.set_visuals(visuals);
    }

    pub fn set_font(&self, ctx: &egui::Context) {
        let mut fonts = if self.font_fallback_to_default {
            egui::FontDefinitions::default()
        } else {
            egui::FontDefinitions::empty()
        };

        match self.font_data {
            None => (),
            Some(font_data) => {
                fonts.font_data.insert(
                    self.font_name.to_owned(),
                    Arc::new(egui::FontData::from_static(font_data)));

                fonts.families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, self.font_name.to_owned());

                fonts.families
                    .entry(egui::FontFamily::Monospace)
                    .or_default()
                    .push(self.font_name.to_owned());
            }
        }

        match self.mono_font_data {
            None => (),
            Some(font_data) => {
                fonts.font_data.insert(
                    self.mono_font_name.to_owned(),
                    Arc::new(egui::FontData::from_static(font_data)));

                fonts.families
                    .entry(egui::FontFamily::Monospace)
                    .or_default()
                    .insert(0, self.mono_font_name.to_owned());
            }
        }

        match self.icon_font_data {
            None => (),
            Some(font_data) => {
                fonts.font_data.insert(
                    self.icon_font_name.to_owned(),
                    Arc::new(egui::FontData::from_static(font_data)));

                fonts.families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(1, self.icon_font_name.to_owned());
            }
        }

        ctx.set_fonts(fonts);

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(self.font_heading_size, Proportional)),
            (Body, FontId::new(self.font_body_size, Proportional)),
            (Monospace, FontId::new(self.font_monospace_size, egui::FontFamily::Monospace)),
            (Button, FontId::new(self.font_button_size, Proportional)),
            (Small, FontId::new(self.font_small_size, Proportional)),
        ]
        .into();
        style.wrap_mode = Some(TextWrapMode::Wrap);

        ctx.set_style(style);
    }
}
