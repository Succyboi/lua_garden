use core::fmt;
use std::fmt::format;
use std::sync::mpsc::channel;
use std::{ fmt::Error, sync::atomic::Ordering };
use mlem_base::console::ConsoleSender;
use crate::consts::PLUGIN_METADATA;
use crate::{ StretchParams };
use nih_plug::{ prelude::* };
use utils::{ RMS, Timer };
use mlem_base::runtime::utils::{ self };
use signalsmith_stretch::Stretch;

const MIN_SPEED: f32 = 0.001;
const MAX_BUFFER_SECONDS: f32 = 60.0;

pub struct Runtime {
    pub console: Option<ConsoleSender>,

    sample_rate: f32,
    buffer_size: usize,
    channels: usize,

    last_active: bool,
    stretch: Vec<Stretch>,
    stretch_buffers: Vec<Vec<f32>>,

    run_time: RMS,
    clip: bool
}

impl Runtime {
    pub fn new(console: Option<ConsoleSender>) -> Runtime {
        let runtime = Self {
            console: console,

            sample_rate: 0.0,
            buffer_size: 0,
            channels: 0,
            
            last_active: false,
            stretch: Vec::new(),
            stretch_buffers: Vec::new(),

            run_time: RMS::new(1.0),
            clip: PLUGIN_METADATA.build_is_debug
        };

        return runtime;
    }
    
    pub fn init(&mut self, sample_rate: f32) {        
        let execute_timer = Timer::new();

        self.sample_rate = sample_rate;
        
        let execute_time = execute_timer.elapsed_ms();
        self.log(format!("Init in {:.2}ms.", execute_time));
    }

    pub fn reset(&mut self) {
        let execute_timer = Timer::new();

        for s in self.stretch.iter_mut() {
            s.reset();
        }

        for b in self.stretch_buffers.iter_mut() {
            b.clear();
        }

        self.log(format!("Reset in {:.2}ms.", execute_timer.elapsed_ms()));
    }

    pub fn run(&mut self, buffer: &mut Buffer, params: &StretchParams, transport: &Transport) {
        self.buffer_size = buffer.samples();
        self.channels = buffer.channels();
        let execute_timer = Timer::new();
        let active = params.speed.value() < 1.0; // TODO switch for param
        let reset = !active && self.last_active;

        if reset {
            self.reset();
        }
        if active {
            self.run_stretch_buffers(buffer, params);

            if self.clip {
                for channel_samples in buffer.iter_samples() {                        
                    for sample in channel_samples {
                        *sample = utils::clip(*sample);
                    }
                }
            }
        }

        self.run_time.process( execute_timer.elapsed_ms(), self.sample_rate);
        self.update_params(params);
    }

    fn run_stretch_buffers(&mut self, buffer: &mut Buffer, params: &StretchParams) {
        let max_buffer_len = (self.sample_rate * MAX_BUFFER_SECONDS) as usize;

        for mut block_channel in buffer.iter_blocks(buffer.samples()) {     
            for channel in 0..block_channel.1.channels() {
                match block_channel.1.get_mut(channel) {
                    Some(mut samples) => {
                        if channel >= self.stretch.len() {
                            self.stretch.push(Stretch::preset_default(1, self.sample_rate as u32));
                            self.stretch_buffers.push(Vec::new());
                        }

                        if self.stretch_buffers[channel].len() > max_buffer_len {
                            continue;
                        }

                        let output_len = f32::round(samples.len() as f32 / f32::max(params.speed.value(), MIN_SPEED)) as usize;
                        let mut output = vec![0.0f32; output_len];
                        self.stretch[channel].process(&mut samples, &mut output);
                        self.stretch_buffers[channel].append(&mut output);
                    },
                    None => {
                        self.log(format!("Could not get samples from block."));
                    }
                };
            }
        }
    }

    pub fn update_params(&mut self, params: &StretchParams) {
        params.sample_rate.store(self.sample_rate, Ordering::Relaxed);
        params.buffer_size.store(self.buffer_size, Ordering::Relaxed);
        params.channels.store(self.channels, Ordering::Relaxed);
        params.run_ms.store(self.run_time.get(), Ordering::Relaxed);
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