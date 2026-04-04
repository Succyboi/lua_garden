use std::sync::{Arc, Mutex, atomic::AtomicUsize};
use atomic_float::AtomicF32;
use mlem_base::base::mlem_params::MlemParams;
use nih_plug::{params::{BoolParam, FloatParam, IntParam, Params}, prelude::{FloatRange, IntRange}};
use nih_plug_egui::{EguiState};
use crate::consts::PLUGIN_METADATA;

#[derive(Params)]
pub struct MarkovParams {
    #[persist = "editor-state"] pub editor_state: Arc<EguiState>,
    #[id = "volume"]            pub volume: FloatParam,
    #[id = "words"]             pub words: IntParam,
    #[id = "period"]            pub period: FloatParam,
    
    sample_rate: AtomicF32,
    buffer_size: AtomicUsize,
    channels: AtomicUsize,
    run_ms: AtomicF32,

    pub buffer_preview: Mutex<Vec<String>>,
}

impl MlemParams for MarkovParams {
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

impl Default for MarkovParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(PLUGIN_METADATA.window_width, PLUGIN_METADATA.window_height),
            volume: FloatParam::new("Volume", 0.1, FloatRange::Linear { min: 0.0, max: 1.0 }),
            words: IntParam::new("Words", 4, IntRange::Linear { min: 1, max: 8 }),
            period: FloatParam::new("Period", 0.2, FloatRange::Linear { min: 0.01, max: 2.0 }),

            sample_rate: AtomicF32::new(0.0),
            buffer_size: AtomicUsize::new(0),
            channels: AtomicUsize::new(0),
            run_ms: AtomicF32::new(0.0),

            buffer_preview: Mutex::from(Vec::new())
        }
    }
}