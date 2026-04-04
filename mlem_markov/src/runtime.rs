use std::{string, sync::Arc};
use markov::Chain;
use mlem_base::{base::mlem_runtime::MlemRuntime, console::{self, ConsoleSender}, runtime::{pitch::semitone_to_playback_speed, rng::{Rng, RngSplitMix64}, stretch::Stretch}};
use nih_plug::{buffer::{self, Buffer}, prelude::Transport};
use crate::{consts::DEFAULT_DATA, params::MarkovParams};

const GENERATE_CHUNK_SIZE: usize = 16;
const MIN_WORDS: usize = 1;

pub struct MarkovRuntime { 
    params: Arc<MarkovParams>,
    console: ConsoleSender,
    
    rng: RngSplitMix64,
    words: Vec<String>,
    markov: Chain<String>,
    buffer: Vec<u8>,

    word_count: usize,
    curr_byte: u8,
    bit_pos: usize,
    timer: usize,
    period: f32
}

impl MarkovRuntime {
    pub fn new(params: Arc<MarkovParams>) -> Self {
        return Self { 
            params,
            console: ConsoleSender::from_singleton(),

            rng: RngSplitMix64::new(),
            words: Vec::new(),
            markov: Chain::new(),
            buffer: Vec::new(),

            word_count: 0,
            curr_byte: 0,
            bit_pos: 0,
            timer: 0,
            period: 0.0
        };
    }
}

impl MarkovRuntime {
    fn retrain(&mut self, string: String) {
        self.markov = Chain::new();
        let mut buffer_preview = self.params.buffer_preview.lock().unwrap();
        buffer_preview.clear();

        for segment in string.split_whitespace() {
            self.markov.feed_str(segment);
            buffer_preview.push(String::from(segment));
        }
    }

    fn generate(&mut self) {
        for i in 0..GENERATE_CHUNK_SIZE {
            for u in self.markov.generate_str().chars() {
                if u == ' ' { continue; }

                self.buffer.push(u as u8);
            }
        }
    }

    fn next(&mut self) -> u8 {
        if self.buffer.len() <= 0 {
            self.generate();
        }

        return self.buffer.pop().expect("No value in buffer.");
    }
}

impl MlemRuntime<MarkovParams> for MarkovRuntime {
    fn params(&self) ->  Arc<MarkovParams> {
        return self.params.clone();
    }

    fn init(&mut self) {
        for word in String::from(DEFAULT_DATA).split_whitespace() {
            self.words.push(String::from(word));
        }
    }
    
    fn reset(&mut self) {
        self.timer = 0;
        self.period = self.rng.range_f32(0.0..1.0);
        self.word_count = self.rng.range_usize(MIN_WORDS..(self.params.words.value() as usize));
        self.buffer.clear();
        let mut words = String::new();
        for i in 0..self.word_count {
            words.push_str(&self.words[self.rng.range_usize(0..self.words.len()) as usize]);
            words.push(' ');
        }
    
        self.retrain(words);
    }
    
    fn run(&mut self, buffer: &mut Buffer, _transport: &Transport) {
        for channel_samples in buffer.iter_samples() {
            self.timer += 1;
            if self.timer as f32 / self.sample_rate() as f32 > (self.period * self.params.period.value()) {
                self.reset();
            }

            for sample in channel_samples {
                if self.bit_pos >= 8 {
                    self.bit_pos = 0;
                    self.curr_byte = self.next();
                };

                let byte = self.curr_byte;
                let mask = 1 << self.bit_pos;
                let raw = (mask & byte) > 0;
                let value = if raw { 1.0 } else { -1.0 }; 
                self.bit_pos = self.bit_pos + 1;

                *sample = value * self.params.volume.value();
            }
        }
    }
}