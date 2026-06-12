use std::{fmt::format, fs, io, string, sync::Arc};
use mlem_base::{base::mlem_runtime::MlemRuntime, console::{self, ConsoleSender}, runtime::{buffers::RecBuffer, pitch::semitone_to_playback_speed, rng::{Rng, RngSplitMix64}, stretch::Stretch}};
use nih_plug::{buffer::{self, Buffer}, prelude::Transport};
use mp3lame_encoder::{Builder, DualPcm, FlushNoGap, MonoPcm};
use crate::params::MP3Params;

const CACHE_FILE_NAME: &str = "cache.mp3";
const FILE_PADDING: usize = 1024;

pub struct MP3Runtime { 
    params: Arc<MP3Params>,
    console: ConsoleSender
}

impl MP3Runtime {
    pub fn new(params: Arc<MP3Params>) -> Self {
        return Self { 
            params,
            console: ConsoleSender::from_singleton(),
        };
    }
}

impl MP3Runtime {
    fn process(&mut self, samples: &mut [f32]) -> Result<(), String> {
        let encoded_data = self.encode(samples)?;
        let decoded_samples = self.decode(encoded_data)?;
        let decoded_samples_length = decoded_samples.len() - usize::min(decoded_samples.len(), FILE_PADDING);

        for i in 0..usize::min(samples.len(), decoded_samples_length) {
            samples[i] = decoded_samples[i];
        }

        if samples.len() != decoded_samples_length {
            self.console.log(format!("Original ({original}) vs processed ({processed}) sample buffer length produced a difference of {difference} samples.", original = samples.len(), processed = decoded_samples.len(), difference = usize::abs_diff(samples.len(), decoded_samples.len())));
        }

        return Ok(());
    }
    
    fn encode(&mut self, samples: &mut [f32]) -> Result<Vec<u8>, String> {
        let mut encoder = Builder::new().expect("No builder was created.")
            .with_num_channels(1).map_err(|e| e.to_string())?
            .with_sample_rate(44_100).map_err(|e| e.to_string())?
            .with_brate(mp3lame_encoder::Bitrate::Kbps192).map_err(|e| e.to_string())?
            .with_quality(mp3lame_encoder::Quality::Best).map_err(|e| e.to_string())?
            .build().map_err(|e| e.to_string())?;

        let mut in_samples = Vec::new();
        in_samples.extend_from_slice(samples);
        let padding = [0.0f32; FILE_PADDING];
        in_samples.extend_from_slice(&padding);
        let in_buffer = MonoPcm(&in_samples);
        let mut out_buffer = Vec::new();

        out_buffer.reserve(mp3lame_encoder::max_required_buffer_size(in_samples.len()));
        encoder.encode_to_vec(in_buffer, &mut out_buffer).map_err(|e| e.to_string())?;
        encoder.flush_to_vec::<FlushNoGap>(&mut out_buffer).map_err(|e| e.to_string())?;

        let mut lame_tag = Vec::new();
        lame_tag.reserve(encoder.lame_tag_size());

        if !std::fs::metadata(CACHE_FILE_NAME).is_ok() {
            let mut output_file = fs::File::create(CACHE_FILE_NAME).map_err(|e| e.to_string())?;
            io::Write::write_all(&mut output_file, &lame_tag).map_err(|e| e.to_string())?;
            io::Write::write_all(&mut output_file, &out_buffer[..]).map_err(|e| e.to_string())?;
            io::Write::flush(&mut output_file).map_err(|e| e.to_string())?;
        }

        return Ok(());
    }

    fn decode(&mut self) -> Result<Vec<f32>, String> {
        return Ok(Vec::new());
    }
}

impl MlemRuntime<MP3Params> for MP3Runtime {
    fn params(&self) ->  Arc<MP3Params> {
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
                        self.process(samples);
                    },
                    None => {
                        self.console.log(format!("Could not get samples from block."));
                    }
                };
            }
        }
    }
}