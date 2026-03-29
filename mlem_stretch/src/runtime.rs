use std::sync::Arc;
use mlem_base::{base::mlem_runtime::MlemRuntime, console::ConsoleSender, runtime::{pitch::semitone_to_playback_speed, rng::{Rng, RngSplitMix64}, stretch::Stretch}};
use nih_plug::{buffer::Buffer, prelude::Transport};
use crate::params::StretchParams;

const MIN_SPEED: f32 = 0.001;

// TODO fix artefacts at speed 1
pub struct StretchRuntime { 
    params: Arc<StretchParams>,
    console: ConsoleSender,

    rng: RngSplitMix64,
    last_active: bool,
    stretch: Vec<Stretch>,
    variance: Vec<f32>
}

impl StretchRuntime {
    pub fn new(params: Arc<StretchParams>) -> Self {
        return Self { 
            params,
            console: ConsoleSender::from_singleton(),

            rng: RngSplitMix64::new(),
            last_active: false,
            stretch: Vec::new(),
            variance: Vec::new()
        };
    }
}

impl StretchRuntime {
    fn run_stretch_buffers(&mut self, buffer: &mut Buffer) {
        for mut block_channel in buffer.iter_blocks(buffer.samples()) {     
            for channel in 0..block_channel.1.channels() {
                match block_channel.1.get_mut(channel) {
                    Some(mut samples) => {
                        if channel >= self.stretch.len() {
                            self.stretch.push(Stretch::new()
                                .with_sample_rate(self.sample_rate() as usize)
                                .with_max_buffer_size(self.buffer_size() as f32));
                        }

                        if channel >= self.variance.len() {
                            self.variance.push(self.rng.range_f32(-1.0..1.0));
                        }

                        let speed = f32::clamp(self.params.speed.value() + self.variance[channel] * self.params.variance.value(), MIN_SPEED, 1.0);
                        let pitch = semitone_to_playback_speed(self.params.pitch.value() as f32);

                        self.stretch[channel].set_window_size(self.params.window.value());
                        self.stretch[channel].process(&mut samples, speed, pitch);
                    },
                    None => {
                        self.console.log(format!("Could not get samples from block."));
                    }
                };
            }
        }
    }
}

impl MlemRuntime<StretchParams> for StretchRuntime {
    fn params(&self) ->  Arc<StretchParams> {
        return self.params.clone();
    }

    fn init(&mut self) { }
    
    fn reset(&mut self) {
        for s in self.stretch.iter_mut() {
            s.reset();
        }

        self.variance.clear();
    }
    
    fn run(&mut self, buffer: &mut Buffer, _transport: &Transport) {
        let active = self.params.stretch.value();
        let reset = !active && self.last_active;
        self.last_active = active;

        if reset {
            self.reset();
        }

        if active {
            self.run_stretch_buffers(buffer);
        } else {
            for s in self.stretch.iter_mut() {
                s.set_max_buffer_size(self.params.buffer.value());
            }
        }
    }
}