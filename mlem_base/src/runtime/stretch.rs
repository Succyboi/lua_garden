use std::{collections::VecDeque, usize};
use crate::runtime::utils;

pub struct Stretch {
    sample_rate: usize,
    window_size: f32,

    buffer: VecDeque<f32>,
    rec_buffer: VecDeque<f32>,
    buffer_pos: usize,
    delta: isize
}

impl Stretch {
    pub fn new() -> Stretch {
        let mut stretch = Self {
            sample_rate: 0,
            window_size: 0.0,

            buffer: VecDeque::new(),
            rec_buffer: VecDeque::new(),
            buffer_pos: 0,
            delta: 0
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

    pub fn set_sample_rate(&mut self, sample_rate: usize) {
        self.sample_rate = sample_rate;
    }

    pub fn set_window_size(&mut self, window_size: f32) {
        self.window_size = window_size;
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.buffer_pos = 0;
        self.delta = 0;
    }

    pub fn process(&mut self, input: impl AsRef<[f32]>, mut output: impl AsMut<[f32]>) {
        let input = input.as_ref();
        let output = output.as_mut();

        if self.buffer.is_empty() {
            if !self.try_update_curr(input, output, 0) {
                self.buffer.push_back(0.0);
            }
        }

        let mut index = 0;
        
        while index < output.len() {
            if self.buffer_pos >= self.buffer.len() && self.delta < 0 {
                let from = (index as f32 / output.len() as f32 * input.len() as f32) as usize;
                self.try_update_curr(input, output, from);
            }

            output[index] = self.buffer[self.buffer_pos % self.buffer.len()];

            self.delta -= 1;
            self.buffer_pos += 1;
            index += 1;
        }

        
        /* OLD BUT USEFUL SNIPPET
        let input = input.as_ref();
        let output = output.as_mut();

        let target_window_len = usize::min(self.window_len(), input.len());
        let in_windows = f32::ceil(input.len() as f32 / target_window_len as f32);
        let window_len = input.len() / in_windows as usize;
        let out_windows = output.len() as f32 / window_len as f32;
        let in_window_reps = f32::floor(out_windows / in_windows) as usize;

        // TODO left off here. No clue if this is correct lmao. Test it.

        for i in 0..output.len() {
            let in_t = i % window_len;
            let win = i / window_len;
            let in_off = win / in_window_reps * window_len;
            let in_i = in_t + in_off;

            output[i] = input[in_i];
        }
        */
    }

    fn try_update_curr(&mut self, input: &[f32], output: &[f32], from: usize) -> bool {
        let target = usize::min(from + self.window_len(), input.len() - 1);
        let next_zero = utils::next_zero_crossing(input, target);
        
        if next_zero == target { return false; }

        self.buffer.clear();
        self.buffer_pos = 0;

        for i in from..next_zero {
            self.buffer.push_back(input[i]);
        }

        let stretch_factor = output.len() as f32 / input.len() as f32;
        self.delta += f32::floor(self.buffer.len() as f32 * stretch_factor) as isize;
        
        return false;
    }

    fn window_len(&self) -> usize {
        return f32::floor(self.window_size * self.sample_rate as f32) as usize;
    }
}