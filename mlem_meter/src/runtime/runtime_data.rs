use std::collections::BTreeMap;

use crate::interface::interface_data::InterfaceData;

use super::{parameter::Parameter, Runtime};

#[derive(Clone)]
pub struct RuntimeData {
    pub sample_rate: f32,
    pub buffer_size: usize,
    pub channels: usize,
    pub run_ms: f32,

    pub  meter_id: usize,
    pub active_time_ms: f32,

    pub lufs_global_loudness: f64,
    pub lufs_momentary_loudness: f64,
    pub lufs_range_loudness: f64,
    pub lufs_shortterm_loudness: f64,

    pub rms_momentary_loudness: f32,
    pub rms_shortterm_loudness: f32,
}

impl RuntimeData {
    pub fn new() -> RuntimeData {
        Self {
            sample_rate: 0.0,
            buffer_size: 0,
            channels: 0,
            run_ms: 0.0,
            
            meter_id: 0,
            active_time_ms: 0.0,

            lufs_global_loudness: 0.0,
            lufs_momentary_loudness: 0.0,
            lufs_range_loudness: 0.0,
            lufs_shortterm_loudness: 0.0,

            rms_momentary_loudness: 0.0,
            rms_shortterm_loudness: 0.0
        }
    }

    pub fn update_from_interface(&mut self, interface_data: &InterfaceData) {
        self.meter_id = interface_data.meter_id;
    }

    pub fn update_from_runtime(&mut self, runtime: &mut Runtime, interface_data: &InterfaceData) {
        runtime.update_runtime_data(self);
    }
}