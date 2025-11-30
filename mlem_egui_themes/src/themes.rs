use crate::egui::Color32;

const COLOR_PARSING_ERROR: &str = "Couldn't parse hex color.";

/// The colors for a theme variant.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    pub font_name: &'static str,
    pub font_data: Option<&'static[u8]>,
    pub mono_font_name: &'static str,
    pub mono_font_data: Option<&'static[u8]>,
    pub icon_font_name: &'static str,
    pub icon_font_data: Option<&'static[u8]>,
    
    pub font_heading_size: f32,
    pub font_body_size: f32,
    pub font_monospace_size: f32,
    pub font_button_size: f32,
    pub font_small_size: f32,
    pub font_fallback_to_default: bool,

    pub darkmode: bool,
    pub selection_opacity: f32,
    pub shadow_opacity: f32,

    pub background: Color32,

    pub f_high: Color32,
    pub f_med: Color32,
    pub f_low: Color32,
    pub f_inv: Color32,

    pub b_high: Color32,
    pub b_med: Color32,
    pub b_low: Color32,
    pub b_inv: Color32
}

// ===
// GARDEN DEFAULT THEMES

const GARDEN_FONT_NAME: &str = "Inter Regular";
const GARDEN_FONT: &[u8] = include_bytes!("../static/fonts/inter_regular_v4-1.otf");
const GARDEN_MONO_FONT_NAME: &str = "Cozette";
const GARDEN_MONO_FONT: &[u8] = include_bytes!("../static/fonts/cozette_vector_v1-25-2.otf");
const GARDEN_ICON_FONT_NAME: &str = "Phosphor";
const GARDEN_ICON_FONT: &[u8] = include_bytes!("../static/fonts/phosphor_v2-1.ttf");

pub fn garden_day() -> Theme {
    Theme {
        font_name: &GARDEN_FONT_NAME,
        font_data: Some(&GARDEN_FONT),
        mono_font_name: &GARDEN_MONO_FONT_NAME,
        mono_font_data: Some(&GARDEN_MONO_FONT),
        icon_font_name: &GARDEN_ICON_FONT_NAME,
        icon_font_data: Some(&GARDEN_ICON_FONT),
        
        font_heading_size: 18.0,
        font_body_size: 12.0,
        font_monospace_size: 13.0,
        font_button_size: 12.0,
        font_small_size: 8.0,
        font_fallback_to_default: true,

        darkmode: true,
        selection_opacity: 1.0,
        shadow_opacity: 1.0,
    
        background: Color32::from_hex("#eeefee").expect(COLOR_PARSING_ERROR),

        f_high: Color32::from_hex("#222222").expect(COLOR_PARSING_ERROR),
        f_med: Color32::from_hex("#00b9be").expect(COLOR_PARSING_ERROR),
        f_low: Color32::from_hex("#bbbcbb").expect(COLOR_PARSING_ERROR),
        f_inv: Color32::from_hex("#545454").expect(COLOR_PARSING_ERROR),
    
        b_high: Color32::from_hex("#545454").expect(COLOR_PARSING_ERROR),
        b_med: Color32::from_hex("#ced0ce").expect(COLOR_PARSING_ERROR),
        b_low: Color32::from_hex("#f5f5f5").expect(COLOR_PARSING_ERROR),
        b_inv: Color32::from_hex("#ff3796").expect(COLOR_PARSING_ERROR)
    }
}

pub fn garden_night() -> Theme {
    Theme {
        font_name: &GARDEN_FONT_NAME,
        font_data: Some(&GARDEN_FONT),
        mono_font_name: &GARDEN_MONO_FONT_NAME,
        mono_font_data: Some(&GARDEN_MONO_FONT),
        icon_font_name: &GARDEN_ICON_FONT_NAME,
        icon_font_data: Some(&GARDEN_ICON_FONT),

        font_heading_size: 18.0,
        font_body_size: 12.0,
        font_monospace_size: 13.0,
        font_button_size: 12.0,
        font_small_size: 8.0,
        font_fallback_to_default: true,

        darkmode: true,
        selection_opacity: 1.0,
        shadow_opacity: 1.0,
    
        background: Color32::from_hex("#222222").expect(COLOR_PARSING_ERROR),

        f_high: Color32::from_hex("#ffffff").expect(COLOR_PARSING_ERROR),
        f_med: Color32::from_hex("#458FFF").expect(COLOR_PARSING_ERROR),
        f_low: Color32::from_hex("#888888").expect(COLOR_PARSING_ERROR),
        f_inv: Color32::from_hex("#000000").expect(COLOR_PARSING_ERROR),
    
        b_high: Color32::from_hex("#555555").expect(COLOR_PARSING_ERROR),
        b_med: Color32::from_hex("#333333").expect(COLOR_PARSING_ERROR),
        b_low: Color32::from_hex("#111111").expect(COLOR_PARSING_ERROR),
        b_inv: Color32::from_hex("#ffb545").expect(COLOR_PARSING_ERROR)
    }
}

pub fn garden_gameboy() -> Theme {
    Theme {
        font_name: &GARDEN_FONT_NAME,
        font_data: Some(&GARDEN_FONT),
        mono_font_name: &GARDEN_MONO_FONT_NAME,
        mono_font_data: Some(&GARDEN_MONO_FONT),
        icon_font_name: &GARDEN_ICON_FONT_NAME,
        icon_font_data: Some(&GARDEN_ICON_FONT),

        font_heading_size: 18.0,
        font_body_size: 12.0,
        font_monospace_size: 13.0,
        font_button_size: 12.0,
        font_small_size: 8.0,
        font_fallback_to_default: true,

        darkmode: true,
        selection_opacity: 1.0,
        shadow_opacity: 1.0,
    
        background: Color32::from_hex("#9BBC0F").expect(COLOR_PARSING_ERROR),

        f_high: Color32::from_hex("#0F380F").expect(COLOR_PARSING_ERROR),
        f_med: Color32::from_hex("#0F380F").expect(COLOR_PARSING_ERROR),
        f_low: Color32::from_hex("#306230").expect(COLOR_PARSING_ERROR),
        f_inv: Color32::from_hex("#306230").expect(COLOR_PARSING_ERROR),
    
        b_high: Color32::from_hex("#8BAC0F").expect(COLOR_PARSING_ERROR),
        b_med: Color32::from_hex("#8BAC0F").expect(COLOR_PARSING_ERROR),
        b_low: Color32::from_hex("#8BAC0F").expect(COLOR_PARSING_ERROR),
        b_inv: Color32::from_hex("#0F380F").expect(COLOR_PARSING_ERROR)
    }
}

pub fn garden_playdate() -> Theme {
    Theme {
        font_name: &GARDEN_FONT_NAME,
        font_data: Some(&GARDEN_FONT),
        mono_font_name: &GARDEN_MONO_FONT_NAME,
        mono_font_data: Some(&GARDEN_MONO_FONT),
        icon_font_name: &GARDEN_ICON_FONT_NAME,
        icon_font_data: Some(&GARDEN_ICON_FONT),

        font_heading_size: 18.0,
        font_body_size: 12.0,
        font_monospace_size: 13.0,
        font_button_size: 12.0,
        font_small_size: 8.0,
        font_fallback_to_default: true,

        darkmode: true,
        selection_opacity: 1.0,
        shadow_opacity: 1.0,
    
        background: Color32::from_hex("#3a3630").expect(COLOR_PARSING_ERROR),

        f_high: Color32::from_hex("#cac7ba").expect(COLOR_PARSING_ERROR),
        f_med: Color32::from_hex("#f16f3d").expect(COLOR_PARSING_ERROR),
        f_low: Color32::from_hex("#807666").expect(COLOR_PARSING_ERROR),
        f_inv: Color32::from_hex("#3a3630").expect(COLOR_PARSING_ERROR),
        
        b_high: Color32::from_hex("#3a3630").expect(COLOR_PARSING_ERROR),
        b_med: Color32::from_hex("#433f39").expect(COLOR_PARSING_ERROR),
        b_low: Color32::from_hex("#433f39").expect(COLOR_PARSING_ERROR),
        b_inv: Color32::from_hex("#a6e22e").expect(COLOR_PARSING_ERROR)
    }
}

// GARDEN DEFAULT THEMES
// ===