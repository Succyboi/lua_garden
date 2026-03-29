use std::sync::{Arc, atomic::{AtomicBool, AtomicUsize}};
use atomic_float::{AtomicF32, AtomicF64};
use mlem_base::base::mlem_params::MlemParams;
use nih_plug::{params::{BoolParam, Params}};
use nih_plug_egui::EguiState;
use crate::consts::PLUGIN_METADATA;

#[derive(Params)]
pub struct MeterParams {
    #[persist = "editor-state"] pub editor_state: Arc<EguiState>,
    #[id = "reset_on_play"]     pub reset_on_play: BoolParam,
    
    sample_rate: AtomicF32,
    buffer_size: AtomicUsize,
    channels: AtomicUsize,
    run_ms: AtomicF32,
    
    pub reset_meter: AtomicBool,
    pub active_time_ms: AtomicF32,
    pub lufs_global_loudness: AtomicF64,
    pub lufs_momentary_loudness: AtomicF64,
    pub lufs_range_loudness: AtomicF64,
    pub lufs_shortterm_loudness: AtomicF64
}

impl MlemParams for MeterParams {
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

impl Default for MeterParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(PLUGIN_METADATA.window_width, PLUGIN_METADATA.window_height),
            reset_on_play: BoolParam::new("Reset On Play", true),

            reset_meter: AtomicBool::new(false),
            sample_rate: AtomicF32::new(0.0),
            buffer_size: AtomicUsize::new(0),
            channels: AtomicUsize::new(0),
            run_ms: AtomicF32::new(0.0),

            active_time_ms: AtomicF32::new(0.0),
            lufs_global_loudness: AtomicF64::new(0.0),
            lufs_momentary_loudness: AtomicF64::new(0.0),
            lufs_range_loudness: AtomicF64::new(0.0),
            lufs_shortterm_loudness: AtomicF64::new(0.0)
        }
    }
}