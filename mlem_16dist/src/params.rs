use std::sync::{Arc, Mutex, atomic::AtomicUsize};
use atomic_float::AtomicF32;
use mlem_base::base::mlem_params::MlemParams;
use nih_plug::{params::{BoolParam, FloatParam, IntParam, Params}, prelude::{FloatRange, IntRange}};
use nih_plug_egui::{EguiState};
use crate::consts::PLUGIN_METADATA;

#[derive(Params)]
pub struct SixteenDistParams {
    #[persist = "editor-state"] pub editor_state: Arc<EguiState>,
    #[id = "1"]                 pub one: FloatParam,
    #[id = "2"]                 pub two: FloatParam,
    #[id = "3"]                 pub three: FloatParam,
    #[id = "4"]                 pub four: FloatParam,
    #[id = "5"]                 pub five: FloatParam,
    #[id = "6"]                 pub six: FloatParam,
    #[id = "7"]                 pub seven: FloatParam,
    #[id = "8"]                 pub eight: FloatParam,
    #[id = "9"]                 pub nine: FloatParam,
    #[id = "10"]                pub ten: FloatParam,
    #[id = "11"]                pub eleven: FloatParam,
    #[id = "12"]                pub twelve: FloatParam,
    #[id = "13"]                pub thirteen: FloatParam,
    #[id = "14"]                pub fourteen: FloatParam,
    #[id = "15"]                pub fifteen: FloatParam,
    #[id = "16"]                pub sixteen: FloatParam,

    
    sample_rate: AtomicF32,
    buffer_size: AtomicUsize,
    channels: AtomicUsize,
    run_ms: AtomicF32,
}

impl MlemParams for SixteenDistParams {
    fn sample_rate(&self) -> &AtomicF32 {
        return &self.sample_rate;        
    }

    fn buffer_size(&self) -> &AtomicUsize {
        return &self.buffer_size;
    }

    fn channels(&self) -> &AtomicUsize {
        return &self.channels;
    }

    fn run_ms(&self) -> &AtomicF32 {
        return &self.run_ms;
    }
}

impl Default for SixteenDistParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(PLUGIN_METADATA.window_width, PLUGIN_METADATA.window_height),
            one:        FloatParam::new("1", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),
            two:        FloatParam::new("2", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),
            three:      FloatParam::new("3", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),
            four:       FloatParam::new("4", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),
            five:       FloatParam::new("5", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),
            six:        FloatParam::new("6", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),
            seven:      FloatParam::new("7", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),
            eight:      FloatParam::new("8", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),
            nine:       FloatParam::new("9", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),
            ten:        FloatParam::new("10", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),
            eleven:     FloatParam::new("11", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),
            twelve:     FloatParam::new("12", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),
            thirteen:   FloatParam::new("13", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),
            fourteen:   FloatParam::new("14", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),
            fifteen:    FloatParam::new("15", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),
            sixteen:    FloatParam::new("16", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit("%"),

            sample_rate: AtomicF32::new(0.0),
            buffer_size: AtomicUsize::new(0),
            channels: AtomicUsize::new(0),
            run_ms: AtomicF32::new(0.0),
        }
    }
}