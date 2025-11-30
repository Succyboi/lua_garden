pub mod utils;
pub mod runtime_data;
pub mod parameter;

use crate::console::ConsoleSender;
use nih_plug::prelude::*;
use utils::{ RMS, Timer };

pub struct Runtime {
    pub console: Option<ConsoleSender>,

    sample_rate : f32,
    buffer_size : usize,
    channels : usize,

    run_time_rms: RMS,
    clip: bool
}

impl Runtime {
    pub fn new(console: Option<ConsoleSender>) -> Runtime {
        let runtime = Self {
            console: console,

            sample_rate: 0.0,
            buffer_size: 0,
            channels: 0,

            run_time_rms: RMS::new(),
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

        self.log(format!("Reset in {:.2}ms.", execute_timer.elapsed_ms()));
    }

    pub fn run(&mut self, buffer : &mut Buffer) {
        self.buffer_size = buffer.samples();
        self.channels = buffer.channels();
        let execute_timer = Timer::new();

        if self.clip {
            for channel_samples in buffer.iter_samples() {                        
                for sample in channel_samples {
                    *sample = utils::clip(*sample);
                }
            }
        }

        self.run_time_rms.process( execute_timer.elapsed_ms(), self.sample_rate);
    }

    pub fn get_sample_rate(&self) -> f32 {
        return self.sample_rate;
    }

    pub fn get_buffer_size(&self) -> usize {
        return self.buffer_size;
    }

    pub fn get_channels(&self) -> usize {
        return self.channels;
    }

    pub fn get_run_ms(&self) -> f32 {
        return self.run_time_rms.get();
    }

    fn log(&self, message : String) {
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