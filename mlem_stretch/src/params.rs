use std::sync::{Arc, atomic::AtomicUsize};
use atomic_float::AtomicF32;
use mlem_base::base::mlem_params::MlemParams;
use nih_plug::{params::{BoolParam, FloatParam, Params}, prelude::FloatRange};
use nih_plug_egui::EguiState;
use crate::consts::PLUGIN_METADATA;

const MIN_BUFFER_SECONDS: f32 = 0.1;
const MAX_BUFFER_SECONDS: f32 = 60.0;

#[derive(Params)]
pub struct StretchParams {
    #[persist = "editor-state"] pub editor_state: Arc<EguiState>,
    #[id = "stretch"]           pub stretch: BoolParam,
    #[id = "speed"]             pub speed: FloatParam,
    #[id = "variance"]          pub variance: FloatParam,
    #[id = "window"]            pub window: FloatParam,
    #[id = "pitch"]             pub pitch: FloatParam,
    #[id = "buffer"]            pub buffer: FloatParam,
    
    sample_rate: AtomicF32,
    buffer_size: AtomicUsize,
    channels: AtomicUsize,
    run_ms: AtomicF32,
}

impl MlemParams for StretchParams {
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

impl Default for StretchParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(PLUGIN_METADATA.window_width, PLUGIN_METADATA.window_height),
            stretch: BoolParam::new("Stretch", false),
            speed: FloatParam::new("Speed", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),
            variance: FloatParam::new("Variance", 0.01, FloatRange::Linear { min: 0.0, max: 1.0 }),
            window: FloatParam::new("Window Size", 0.05, FloatRange::Linear { min: 0.01, max: 0.2 }),
            pitch: FloatParam::new("Pitch", 0.0, FloatRange::Linear { min: -12.0, max: 12.0 }).with_unit("st"),
            buffer: FloatParam::new("Buffer Size", 60.0, FloatRange::Linear { min: MIN_BUFFER_SECONDS, max: MAX_BUFFER_SECONDS }).with_unit("s"),

            sample_rate: AtomicF32::new(0.0),
            buffer_size: AtomicUsize::new(0),
            channels: AtomicUsize::new(0),
            run_ms: AtomicF32::new(0.0),
        }
    }
}