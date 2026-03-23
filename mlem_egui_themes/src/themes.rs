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

    pub background: &'static str,

    pub f_high: &'static str,
    pub f_med: &'static str,
    pub f_low: &'static str,
    pub f_inv: &'static str,

    pub b_high: &'static str,
    pub b_med: &'static str,
    pub b_low: &'static str,
    pub b_inv: &'static str
}

const MLEM_FONT_NAME: &str = "Inter Regular";
const MLEM_FONT: &[u8] = include_bytes!("../static/fonts/inter_regular_v4-1.otf");
const MLEM_FONT_HEADING_SIZE: f32 = 18.0;
const MLEM_FONT_SMALL_SIZE: f32 = 8.0;
const MLEM_FONT_SIZE: f32 = 12.0;

const MLEM_MONO_FONT_NAME: &str = "Cozette";
const MLEM_MONO_FONT: &[u8] = include_bytes!("../static/fonts/cozette_vector_v1-25-2.otf");
const MLEM_MONO_FONT_SIZE: f32 = 13.0;

const MLEM_ICON_FONT_NAME: &str = "Phosphor";
const MLEM_ICON_FONT: &[u8] = include_bytes!("../static/fonts/phosphor_v2-1.ttf");

pub const MLEM_LIGHT : Theme = Theme {
    font_name: &MLEM_FONT_NAME,
    font_data: Some(&MLEM_FONT),
    mono_font_name: &MLEM_MONO_FONT_NAME,
    mono_font_data: Some(&MLEM_MONO_FONT),
    icon_font_name: &MLEM_ICON_FONT_NAME,
    icon_font_data: Some(&MLEM_ICON_FONT),
    
    font_heading_size: MLEM_FONT_HEADING_SIZE,
    font_body_size: MLEM_FONT_SIZE,
    font_monospace_size: MLEM_MONO_FONT_SIZE,
    font_button_size: MLEM_FONT_SIZE,
    font_small_size: MLEM_FONT_SMALL_SIZE,
    font_fallback_to_default: true,

    darkmode: true,
    selection_opacity: 1.0,
    shadow_opacity: 1.0,

    background: "#F8F7F7",

    f_high: "#2b2b26",
    f_med: "#8E8E93",
    f_low: "#cfcfcf",
    f_inv: "#5f5f5f",

    b_high: "#5f5f5f",
    b_med: "#cfcfcf",
    b_low: "#e4e4e4",
    b_inv: "#8E8E93"
};

pub const MLEM_DARK: Theme = Theme {
    font_name: &MLEM_FONT_NAME,
    font_data: Some(&MLEM_FONT),
    mono_font_name: &MLEM_MONO_FONT_NAME,
    mono_font_data: Some(&MLEM_MONO_FONT),
    icon_font_name: &MLEM_ICON_FONT_NAME,
    icon_font_data: Some(&MLEM_ICON_FONT),
    
    font_heading_size: MLEM_FONT_HEADING_SIZE,
    font_body_size: MLEM_FONT_SIZE,
    font_monospace_size: MLEM_MONO_FONT_SIZE,
    font_button_size: MLEM_FONT_SIZE,
    font_small_size: MLEM_FONT_SMALL_SIZE,
    font_fallback_to_default: true,

    darkmode: true,
    selection_opacity: 1.0,
    shadow_opacity: 1.0,

    background: "#080707",

    f_high: "#D9D9D3",
    f_med: "#6E6E6E",
    f_low: "#303030",
    f_inv: "#A1A1A1",

    b_high: "#A1A1A1",
    b_med: "#303030",
    b_low: "#1C1C1C",
    b_inv: "#6E6E6E"
};