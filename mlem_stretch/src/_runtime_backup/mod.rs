use std::sync::atomic::Ordering;

use mlem_base::console::ConsoleSender;
use mlem_base::runtime::pitch::{self, semitone_to_playback_speed};
use mlem_base::runtime::rng::{self, Rng, RngSplitMix64};
use crate::consts::PLUGIN_METADATA;
use crate::{ StretchParams };
use nih_plug::{ prelude::* };
use utils::{ RMS, Timer };
use mlem_base::runtime::utils::{ self };
use mlem_base::runtime::stretch::Stretch;

const MIN_SPEED: f32 = 0.001;

pub struct Runtime {
    pub console: Option<ConsoleSender>,

    sample_rate: f32,
    buffer_size: usize,
    channels: usize,

    rng: RngSplitMix64,
    last_active: bool,
    stretch: Vec<Stretch>,
    variance: Vec<f32>, //TODO Implement variance

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
            
            rng: RngSplitMix64::new(),
            last_active: false,
            stretch: Vec::new(),
            variance: Vec::new(),

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

        self.variance.clear();

        self.log(format!("Reset in {:.2}ms.", execute_timer.elapsed_ms()));
    }

    pub fn run(&mut self, buffer: &mut Buffer, params: &StretchParams, transport: &Transport) {
        self.buffer_size = buffer.samples();
        self.channels = buffer.channels();
        let execute_timer = Timer::new();
        let active = params.stretch.value();
        let reset = !active && self.last_active;
        self.last_active = active;

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
        } else {
            for s in self.stretch.iter_mut() {
                s.set_max_buffer_size(params.buffer.value());
            }
        }

        self.run_time.process( execute_timer.elapsed_ms(), self.sample_rate);
        self.update_params(params);
    }

    fn run_stretch_buffers(&mut self, buffer: &mut Buffer, params: &StretchParams) {
        for mut block_channel in buffer.iter_blocks(buffer.samples()) {     
            for channel in 0..block_channel.1.channels() {
                match block_channel.1.get_mut(channel) {
                    Some(mut samples) => {
                        if channel >= self.stretch.len() {
                            self.stretch.push(Stretch::new()
                                .with_sample_rate(self.sample_rate as usize)
                                .with_max_buffer_size(params.buffer.value()));
                        }

                        if channel >= self.variance.len() {
                            self.variance.push(self.rng.range_f32(-1.0..1.0));
                        }

                        let speed = f32::clamp(params.speed.value() + self.variance[channel] * params.variance.value(), MIN_SPEED, 1.0);
                        let pitch = semitone_to_playback_speed(params.pitch.value() as f32);

                        self.stretch[channel].set_window_size(params.window.value());
                        self.stretch[channel].process(&mut samples, speed, pitch);
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