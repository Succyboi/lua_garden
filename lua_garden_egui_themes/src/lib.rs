mod themes;

pub use themes::*;
use nih_plug_egui::egui::{self, epaint, style };
use crate::epaint::{ FontFamily::Proportional, FontId };
use crate::style::TextStyle::{ Button, Monospace, Heading, Body, Small };

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
            color: theme.b_med,
            ..old.bg_stroke
        },
        fg_stroke: egui::Stroke {
            color: theme.f_high,
            ..old.fg_stroke
        },
        ..old
    }
}

impl Theme {
    pub fn set_visuals(&self, ctx: &egui::Context) {
        let old = ctx.style().visuals.clone();

        let visuals = egui::Visuals {
            hyperlink_color: self.f_med,
            faint_bg_color: self.b_low,
            extreme_bg_color: self.b_low,
            code_bg_color: self.b_low,
            warn_fg_color: self.f_low,
            error_fg_color: self.f_inv,
            window_fill: self.background,
            panel_fill: self.background,
            window_stroke: egui::Stroke {
                color: self.b_low,
                ..old.window_stroke
            },
            widgets: style::Widgets {
                noninteractive: make_widget_visual(old.widgets.noninteractive, self, self.b_low),
                inactive: make_widget_visual(old.widgets.inactive, self, self.b_low),
                hovered: make_widget_visual(old.widgets.hovered, self, self.b_med),
                active: make_widget_visual(old.widgets.active, self, self.b_high),
                open: make_widget_visual(old.widgets.open, self, self.b_low),
            },
            selection: style::Selection {
                bg_fill: self.b_inv.linear_multiply(self.selection_opacity),
                stroke: egui::Stroke {
                    color: self.b_low,
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
                    egui::FontData::from_static(font_data));

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
                    egui::FontData::from_static(font_data));

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
                    egui::FontData::from_static(font_data));

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
        ctx.set_style(style);
    }
}
