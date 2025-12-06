pub mod utils;
pub mod runtime_data;

use std::{ fmt::Error };

use crate::{PluginImplementation, PluginImplementationParams, console::ConsoleSender, runtime::runtime_data::RuntimeData};
use nih_plug::{params, prelude::*};
use utils::{ RMS, Timer };
use ebur128::{EbuR128, Mode};

pub struct Runtime {
    pub console: Option<ConsoleSender>,

    sample_rate: f32,
    buffer_size: usize,
    channels: usize,
    meter_id: usize,
    last_playing: bool,

    active_time: Timer,
    lufs_global_loudness: f64,
    lufs_momentary_loudness: f64,
    lufs_range_loudness: f64,
    lufs_shortterm_loudness: f64,
    
    rms_momentary_loudness: RMS,
    rms_shortterm_loudness: RMS,

    run_time: RMS,
    ebur128: Option<EbuR128>,
    clip: bool
}

impl Runtime {
    pub fn new(console: Option<ConsoleSender>) -> Runtime {
        let runtime = Self {
            console: console,

            sample_rate: 0.0,
            buffer_size: 0,
            channels: 0,
            meter_id: 0,
            last_playing: false,

            active_time: Timer::new(),
            lufs_global_loudness: 0.0,
            lufs_momentary_loudness: 0.0,
            lufs_range_loudness: 0.0,
            lufs_shortterm_loudness: 0.0,

            rms_momentary_loudness: RMS::new(0.4),
            rms_shortterm_loudness: RMS::new(3.0),
            
            run_time: RMS::new(1.0),
            ebur128: None,
            clip: crate::consts::BUILD_IS_DEBUG
        };

        return runtime;
    }
    
    pub fn init(&mut self, sample_rate: f32) {        
        self.sample_rate = sample_rate;

        let execute_timer = Timer::new();
        let execute_time = execute_timer.elapsed_ms();
        
        self.log(format!("Initialization took {:.2}ms.", execute_time));
    }

    pub fn reset(&mut self) {
        let execute_timer = Timer::new();

        /*TODO implement properly, panics on a bunch of expects
        match self.reset_meter() {
            Ok(()) => (),
            Err(e) => self.log(format!("Failed to reset meter: {}", e))
        }*/

        self.log(format!("Reset in {:.2}ms.", execute_timer.elapsed_ms()));
    }

    pub fn run(&mut self, buffer: &mut Buffer, params: &PluginImplementationParams, transport: &Transport) {
        self.buffer_size = buffer.samples();
        self.channels = buffer.channels();
        let execute_timer = Timer::new();

        if params.reset_on_play.value() && self.last_playing != transport.playing {
            self.last_playing = transport.playing;
        
            match self.reset_meter() {
                Ok(()) => (),
                Err(e) => {
                    self.log(format!("Failed to reset meter: {}", e));
                }
            }
        }

        match self.run_ebur128(buffer) {
            Ok(()) => (),
            Err(e) => {
                self.log(format!("Failed to run EbuR128: {}", e));
            }
        }

        for channel_samples in buffer.iter_samples() {                        
            for sample in channel_samples {
                self.rms_momentary_loudness.process(*sample, self.sample_rate);
                self.rms_shortterm_loudness.process(*sample, self.sample_rate);
            }
        }

        if self.clip {
            for channel_samples in buffer.iter_samples() {                        
                for sample in channel_samples {
                    *sample = utils::clip(*sample);
                }
            }
        }

        self.run_time.process( execute_timer.elapsed_ms(), self.sample_rate);
    }

    pub fn update_runtime_data(&mut self, runtime_data: &mut RuntimeData) {
        runtime_data.sample_rate = self.sample_rate;
        runtime_data.buffer_size = self.buffer_size;
        runtime_data.channels = self.channels;
        runtime_data.run_ms = self.run_time.get();

        runtime_data.active_time_ms = self.active_time.elapsed_ms();

        runtime_data.lufs_global_loudness = self.lufs_global_loudness;
        runtime_data.lufs_momentary_loudness = self.lufs_momentary_loudness;
        runtime_data.lufs_range_loudness = self.lufs_range_loudness;
        runtime_data.lufs_shortterm_loudness = self.lufs_shortterm_loudness;

        runtime_data.rms_momentary_loudness = self.rms_momentary_loudness.get();
        runtime_data.rms_shortterm_loudness = self.rms_shortterm_loudness.get();

        if runtime_data.meter_id != self.meter_id {
            self.meter_id = runtime_data.meter_id;
            self.active_time.reset();

            match self.reset_meter() {
                Ok(()) => (),
                Err(e) => self.log(format!("Couldn't refresh EbuR128."))
            }
        }
    }

    fn run_ebur128(&mut self, buffer: &mut Buffer) -> Result<(), Error> {
        match &mut self.ebur128 {
            Some(_ebur128) => (),
            None => {
                self.reset_meter().expect("Couldn't refresh EbuR128.");
            }
        };

        let ebur128 = self.ebur128.as_mut().expect("No EbuR128."); 
        for block_channel in buffer.iter_blocks(buffer.samples()) {     
            for channel in 0..block_channel.1.channels() {
                let block_channel_samples = block_channel.1.get(channel).expect("Could not get samples from block.");

                ebur128.add_frames_f32(block_channel_samples).expect("Couldn't add frames.");
            }
        }

        self.lufs_global_loudness = ebur128.loudness_global().expect("Couldn't get global loudness.");
        self.lufs_momentary_loudness = ebur128.loudness_momentary().expect("Couldn't get momentary loudness.");
        self.lufs_range_loudness = ebur128.loudness_range().expect("Couldn't get range loudness.");
        self.lufs_shortterm_loudness = ebur128.loudness_shortterm().expect("Couldn't get short term loudness.");

        Ok(())
    }

    fn reset_meter(&mut self) -> Result<(), Error>  {
        self.ebur128 = Some(EbuR128::new(self.channels as u32, self.sample_rate as u32, Mode::all()).expect("Couldn't create EbuR128"));

        self.rms_momentary_loudness = RMS::new(0.4);
        self.rms_shortterm_loudness = RMS::new(3.0);

        Ok(())
    }

    fn log(&self, message: String) {
        match &self.console {
            Some(c) => {
                c.log(message);
            },
            None => {
                println!("No console exists for Runtime. Log not registered by receiver: {}", message)
            }
        }
    }
}