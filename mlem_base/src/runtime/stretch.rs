use std::{collections::VecDeque, usize};
use nih_plug_egui::egui::output;

use crate::runtime::{buffers::RecBuffer, utils::{self, lerp}};

const DEFAULT_WINDOW_SIZE: f32 = 0.1;
const DEFAULT_MAX_BUFFER_SIZE: f32 = 60.0;

pub struct Stretch {
    sample_rate: usize,
    window_size: f32,

    buffer: RecBuffer,
    window_pos: f32,
    window_start: f32,
    window_end: f32,
    window_offset: f32
}

impl Stretch {
    pub fn new() -> Stretch {
        let mut stretch = Self {
            sample_rate: 0,
            window_size: DEFAULT_WINDOW_SIZE,

            buffer: RecBuffer::new()
                .with_max_length(DEFAULT_MAX_BUFFER_SIZE),
            window_pos: 0.0,
            window_start: 0.0,
            window_end: 0.0,
            window_offset: 0.0
        };

        stretch.reset();
        return stretch;
    }

    pub fn with_sample_rate(mut self, sample_rate: usize) -> Stretch {
        self.set_sample_rate(sample_rate);
        return self;
    }

    pub fn with_window_size(mut self, window_size: f32) -> Stretch {
        self.set_window_size(window_size);
        return self;
    }

    pub fn with_max_buffer_size(mut self, max_buffer_size: f32) -> Stretch {
        self.set_max_buffer_size(max_buffer_size);
        return self;
    }

    pub fn set_sample_rate(&mut self, sample_rate: usize) {
        self.sample_rate = sample_rate;
        self.buffer.set_sample_rate(sample_rate);
    }

    pub fn set_window_size(&mut self, window_size: f32) {
        self.window_size = window_size;
    }

    pub fn set_max_buffer_size(&mut self, max_buffer_size: f32) {
        self.buffer.set_max_length(max_buffer_size);
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.window_pos = 0.0;
        self.window_start = 0.0;
        self.window_end = 0.0;
        self.window_offset = 0.0;
    }

    pub fn process(&mut self, mut output: impl AsMut<[f32]>, speed: f32, pitch: f32) {
        let output = output.as_mut();
        self.buffer.push_mult(&output);

        let mut index = 0;
        while index < output.len() {
            output[index] = self.next(speed, pitch);

            index += 1;
        }
    }

    fn next(&mut self, speed: f32, pitch: f32) -> f32 {
        let pos = self.window_start + self.window_offset;
        let pos_f = self.buffer[f32::floor(pos) as usize];
        let pos_c = self.buffer[f32::ceil(pos) as usize];
        let pos_t = pos - f32::floor(pos);
        
        self.window_pos += 1.0 * speed;
        self.window_offset += 1.0 * pitch;

        let window_end = self.window_start + (self.window_end - self.window_start) /* * pitch <- cool length correction that causes clicking*/;
        if self.window_start + self.window_offset > window_end {
            self.window_start = utils::previous_zero_crossing(&self.buffer, usize::MAX, f32::floor(self.window_pos - self.window_len() * f32::max(pitch - 1.0, 0.0)) as usize, self.default_window_len() as usize) as f32;
            self.window_end = utils::nearest_zero_crossing(&self.buffer, usize::MAX, f32::floor(self.window_start + self.window_len() * pitch) as usize, self.default_window_len() as usize) as f32;
            self.window_offset = 0.0;
        }

        return lerp(pos_f, pos_c, pos_t);
    }

    fn window_len(&self) -> f32 {
        return self.window_size * self.sample_rate as f32;
    }

    fn default_window_len(&self) -> f32 {
        return DEFAULT_WINDOW_SIZE * self.sample_rate as f32;
    }
}