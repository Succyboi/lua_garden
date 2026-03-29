pub mod utils;
pub mod buffers;
pub mod stretch;
pub mod rng;
pub mod pitch;

use std::sync::Arc;
use std::sync::atomic::Ordering;
use crate::base::{mlem_params::MlemParams, mlem_runtime::MlemRuntime};
use crate::console::ConsoleSender;
use crate::consts::BUILD_IS_DEBUG;
use nih_plug::{ prelude::* };
use utils::{ RMS, Timer };

pub struct Runtime<T: MlemParams, U: MlemRuntime<T>> {
    pub console: Option<ConsoleSender>,
    params: Arc<T>,
    runtime: U,

    sample_rate: f32,
    buffer_size: usize,
    channels: usize,
    
    run_time: RMS,
    clip: bool
}

impl<T: MlemParams, U: MlemRuntime<T>> Runtime<T, U> {
    pub fn new(console: Option<ConsoleSender>, params: Arc<T>, runtime: U) -> Runtime<T, U> {
        let runtime = Self {
            console: console,
            params,
            runtime,

            sample_rate: 0.0,
            buffer_size: 0,
            channels: 0,
            
            run_time: RMS::new(1.0),
            clip: BUILD_IS_DEBUG
        };

        return runtime;
    }
    
    pub fn init(&mut self, sample_rate: f32) {        
        let execute_timer = Timer::new();

        self.sample_rate = sample_rate;
        self.runtime.init();
        
        let execute_time = execute_timer.elapsed_ms();
        self.log(format!("Init in {:.2}ms.", execute_time));
    }

    pub fn reset(&mut self) {
        let execute_timer = Timer::new();

        self.runtime.reset();

        self.log(format!("Reset in {:.2}ms.", execute_timer.elapsed_ms()));
    }

    pub fn run(&mut self, buffer: &mut Buffer, transport: &Transport) {
        self.buffer_size = buffer.samples();
        self.channels = buffer.channels();
        let execute_timer = Timer::new();

        self.runtime.run(buffer, transport);

        if self.clip {
            for channel_samples in buffer.iter_samples() {                        
                for sample in channel_samples {
                    *sample = utils::clip(*sample);
                }
            }
        }

        self.run_time.process( execute_timer.elapsed_ms(), self.sample_rate);
        self.update_params();
    }

    pub fn update_params(&mut self) {
        self.params.sample_rate().store(self.sample_rate, Ordering::Relaxed);
        self.params.buffer_size().store(self.buffer_size, Ordering::Relaxed);
        self.params.channels().store(self.channels, Ordering::Relaxed);
        self.params.run_ms().store(self.run_time.get(), Ordering::Relaxed);
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