use std::sync::{Arc, Mutex, atomic::{AtomicBool, AtomicUsize}};
use atomic_float::{AtomicF32, AtomicF64};
use mlem_base::base::mlem_params::MlemParams;
use nih_plug::{params::{BoolParam, EnumParam, FloatParam, Params}, prelude::FloatRange};
use nih_plug_egui::{EguiState};
use crate::{consts::PLUGIN_METADATA, read_mode::DataReadMode};

pub const MAX_MONOSPACE_WIDTH: usize = 61;
pub const DATA_PREVIEW_SIZE_FULL: usize = MAX_MONOSPACE_WIDTH * 12;
pub const DATA_PREVIEW_SIZE_SMALL: usize = DATA_PREVIEW_SIZE_FULL - MAX_MONOSPACE_WIDTH * 5;

#[derive(Params)]
pub struct DataParams {
    #[persist = "editor-state"] pub editor_state: Arc<EguiState>,
    #[id = "mute"]              pub mute: BoolParam,
    #[id = "mono"]              pub mono: BoolParam,
    #[id = "read-mode"]         pub read_mode: EnumParam<DataReadMode>,
    
    sample_rate: AtomicF32,
    buffer_size: AtomicUsize,
    channels: AtomicUsize,
    run_ms: AtomicF32,
    
    pub path_refresh: AtomicBool,
    pub paths: Mutex<Vec<String>>,
    pub path_current: AtomicUsize,
    pub data_preview: Mutex<[u8; DATA_PREVIEW_SIZE_FULL]>,
    pub data_progress: AtomicF32
}

impl MlemParams for DataParams {
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

impl Default for DataParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(PLUGIN_METADATA.window_width, PLUGIN_METADATA.window_height),

            mute: BoolParam::new("Mute", true),
            mono: BoolParam::new("Mono", false),
            read_mode: EnumParam::new("Read Mode", DataReadMode::Bit8),

            sample_rate: AtomicF32::new(0.0),
            buffer_size: AtomicUsize::new(0),
            channels: AtomicUsize::new(0),
            run_ms: AtomicF32::new(0.0),

            path_refresh: AtomicBool::new(false),
            paths: Mutex::from(Vec::new()),
            path_current: AtomicUsize::new(0),
            data_preview: Mutex::from([0; DATA_PREVIEW_SIZE_FULL]),
            data_progress: AtomicF32::new(0.0)
        }
    }
}