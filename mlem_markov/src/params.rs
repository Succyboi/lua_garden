use std::sync::{Arc, atomic::AtomicUsize};
use atomic_float::AtomicF32;
use mlem_base::base::mlem_params::MlemParams;
use nih_plug::{params::{BoolParam, FloatParam, IntParam, Params}, prelude::FloatRange};
use nih_plug_egui::EguiState;
use crate::consts::PLUGIN_METADATA;

#[derive(Params)]
pub struct MarkovParams {
    #[persist = "editor-state"] pub editor_state: Arc<EguiState>,
    #[id = "volume"]            pub volume: FloatParam,
    #[id = "word_count"]        pub word_count: IntParam,
    
    sample_rate: AtomicF32,
    buffer_size: AtomicUsize,
    channels: AtomicUsize,
    run_ms: AtomicF32,
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
            word_count: IntParam::new("Word Count", 1, nih_plug::prelude::IntRange::Linear { min: 1, max: 8 }),

            sample_rate: AtomicF32::new(0.0),
            buffer_size: AtomicUsize::new(0),
            channels: AtomicUsize::new(0),
            run_ms: AtomicF32::new(0.0),
        }
    }
}