use std::{string, sync::Arc};
use mlem_base::{base::mlem_runtime::MlemRuntime, console::{self, ConsoleSender}, runtime::{buffers::RecBuffer, pitch::semitone_to_playback_speed, rng::{Rng, RngSplitMix64}, stretch::Stretch}};
use nih_plug::{buffer::{self, Buffer}, prelude::Transport};
use crate::{distortion::{chomper, clip_lerp, fold, humpback, quant, sine, stepper, waveshaper}, params::SixteenDistParams};

pub struct SixteenDistRuntime { 
    params: Arc<SixteenDistParams>,
    console: ConsoleSender,
}

impl SixteenDistRuntime {
    pub fn new(params: Arc<SixteenDistParams>) -> Self {
        return Self { 
            params,
            console: ConsoleSender::from_singleton(),
        };
    }
}

impl SixteenDistRuntime {
}

impl MlemRuntime<SixteenDistParams> for SixteenDistRuntime {
    fn params(&self) ->  Arc<SixteenDistParams> {
        return self.params.clone();
    }

    fn init(&mut self) { }
    
    fn reset(&mut self) {
    }
    
    fn run(&mut self, buffer: &mut Buffer, _transport: &Transport) {
        for mut block_channel in buffer.iter_blocks(buffer.samples()) {     
            for channel in 0..block_channel.1.channels() {
                match block_channel.1.get_mut(channel) {
                    Some(samples) => {
                        for i in 0..samples.len() {
                            samples[i] = clip_lerp(samples[i], self.params.one.smoothed.next()  / 100.0);
                            samples[i] = waveshaper(samples[i], self.params.two.smoothed.next()  / 100.0);
                            samples[i] = fold(samples[i], self.params.three.smoothed.next()  / 100.0);
                            samples[i] = quant(samples[i], self.params.four.smoothed.next()  / 100.0);
                            samples[i] = chomper(samples[i], self.params.five.smoothed.next()  / 100.0);
                            samples[i] = sine(samples[i], self.params.six.smoothed.next()  / 100.0);
                            samples[i] = stepper(samples[i], self.params.seven.smoothed.next()  / 100.0);
                            samples[i] = humpback(samples[i], self.params.eight.smoothed.next()  / 100.0);
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