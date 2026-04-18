use std::{string, sync::Arc};
use mlem_base::{base::mlem_runtime::MlemRuntime, console::{self, ConsoleSender}, runtime::{buffers::RecBuffer, pitch::semitone_to_playback_speed, rng::{Rng, RngSplitMix64}, stretch::Stretch}};
use nih_plug::{buffer::{self, Buffer}, prelude::Transport};
use crate::{params::TidbitParams};

pub struct TidbitRuntime { 
    params: Arc<TidbitParams>,
    console: ConsoleSender,

    rng: RngSplitMix64,
    buffers: Vec<RecBuffer>,
    pool: Vec<RecBuffer>
}

impl TidbitRuntime {
    pub fn new(params: Arc<TidbitParams>) -> Self {
        return Self { 
            params,
            console: ConsoleSender::from_singleton(),

            rng: RngSplitMix64::new(),
            buffers: Vec::new(),
            pool: Vec::new()
        };
    }
}

impl TidbitRuntime {
    fn new_buffer(&mut self) -> RecBuffer {
        let mut buffer = RecBuffer::new().with_sample_rate(self.sample_rate() as usize);
        buffer.set_max_length(self.rng.range_f32(self.params.min_length.value()..self.params.max_length.value()));
        
        return buffer;
    }
}

impl MlemRuntime<TidbitParams> for TidbitRuntime {
    fn params(&self) ->  Arc<TidbitParams> {
        return self.params.clone();
    }

    fn init(&mut self) { }
    
    fn reset(&mut self) {
        self.rng = RngSplitMix64::new();
        self.buffers.clear();
        self.pool.clear();
    }
    
    fn run(&mut self, buffer: &mut Buffer, _transport: &Transport) {
        for mut block_channel in buffer.iter_blocks(buffer.samples()) {     
            for channel in 0..block_channel.1.channels() {
                if channel >= self.buffers.len() {
                    let buffer = self.new_buffer();
                    self.buffers.push(buffer);
                }

                match block_channel.1.get_mut(channel) {
                    Some(samples) => {
                        for i in 0..samples.len() {
                            self.buffers[channel].push(samples[i]);

                            if self.buffers[channel].full() {
                                if self.rng.next_f32() < self.params.store_chance.value() / 100.0 {
                                    self.pool.push(self.buffers[channel].clone());
                                }

                                self.buffers[channel] = self.new_buffer();
                            }
                        }
                    },
                    None => {
                        self.console.log(format!("Could not get samples from block."));
                    }
                };
            }
        }
    }
}