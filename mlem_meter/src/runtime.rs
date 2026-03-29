use std::sync::{Arc, atomic::Ordering};
use ebur128::{EbuR128, Mode};
use mlem_base::{base::mlem_runtime::MlemRuntime, console::ConsoleSender, runtime::{pitch::semitone_to_playback_speed, rng::{Rng, RngSplitMix64}, stretch::Stretch, utils::{self, Timer}}};
use nih_plug::{buffer::Buffer, prelude::Transport};
use crate::params::MeterParams;

pub struct MeterRuntime { 
    params: Arc<MeterParams>,
    console: ConsoleSender,

    last_playing: bool,
    active_time: Timer,
    lufs_global_loudness: f64,
    lufs_momentary_loudness: f64,
    lufs_range_loudness: f64,
    lufs_shortterm_loudness: f64,
    ebur128: Option<EbuR128>,
}

impl MeterRuntime {
    pub fn new(params: Arc<MeterParams>) -> Self {
        return Self { 
            params,
            console: ConsoleSender::from_singleton(),

            last_playing: false,
            active_time: Timer::new(),
            lufs_global_loudness: 0.0,
            lufs_momentary_loudness: 0.0,
            lufs_range_loudness: 0.0,
            lufs_shortterm_loudness: 0.0,
            ebur128: None,
        };
    }
}

impl MeterRuntime {
    fn run_ebur128(&mut self, buffer: &mut Buffer) -> Result<(), ebur128::Error> {
        match &mut self.ebur128 {
            Some(_ebur128) => (),
            None => {
                self.reset_meter()?;
            }
        };

        for block_channel in buffer.iter_blocks(buffer.samples()) {     
            for channel in 0..block_channel.1.channels() {
                match block_channel.1.get(channel) {
                    Some(samples) => {
                        let ebur128 = self.ebur128.as_mut().expect("No EbuR128.");
                        ebur128.add_frames_f32(samples)?;
                    },
                    None => {
                        self.console.log(format!("Could not get samples from block."));
                    }
                };
            }
        }

        let ebur128 = self.ebur128.as_ref().expect("No EbuR128.");
        self.lufs_global_loudness = ebur128.loudness_global()?;
        self.lufs_momentary_loudness = ebur128.loudness_momentary()?;
        self.lufs_range_loudness = ebur128.loudness_range()?;
        self.lufs_shortterm_loudness = ebur128.loudness_shortterm()?;

        Ok(())
    }

    fn reset_meter(&mut self) -> Result<(), ebur128::Error>  {
        self.ebur128 = Some(EbuR128::new(self.channels() as u32, self.sample_rate() as u32, Mode::all())?);
        self.active_time.reset();

        Ok(())
    }
}

impl MlemRuntime<MeterParams> for MeterRuntime {
    fn params(&self) ->  Arc<MeterParams> {
        return self.params.clone();
    }

    fn init(&mut self) { }
    
    fn reset(&mut self) {
        match self.reset_meter() {
            Ok(()) => (),
            Err(e) => self.console.log(format!("Failed to reset meter: {}", e))
        }
    }
    
    fn run(&mut self, buffer: &mut Buffer, transport: &Transport) {
        if self.params.reset_on_play.value() && !self.last_playing && transport.playing {        
            match self.reset_meter() {
                Ok(()) => (),
                Err(e) => {
                    self.console.log(format!("Failed to reset meter: {}", e));
                }
            }
        }
        self.last_playing = transport.playing;

        if self.params.reset_meter.load(Ordering::Relaxed) {
            match self.reset_meter() {
                Ok(()) => (),
                Err(e) => self.console.log(format!("Couldn't refresh EbuR128: {}", e))
            }

            self.params.reset_meter.store(false, Ordering::Relaxed);
        }

        match self.run_ebur128(buffer) {
            Ok(()) => (),
            Err(e) => {
                self.console.log(format!("Failed to run EbuR128: {}", e));
            }
        }
    }
}