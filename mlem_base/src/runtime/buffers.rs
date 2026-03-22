use std::{collections::VecDeque, ops::Index, process::Output};

use nih_plug::buffer;

pub struct RecBuffer {
    sample_rate: usize,
    max_length: f32,

    buffer: VecDeque<f32>
}

impl RecBuffer {
    pub fn new() -> RecBuffer {
        let line = Self {
            sample_rate: 0,
            max_length: 0.0,

            buffer: VecDeque::new()
        };

        return line;
    }

    pub fn with_sample_rate(mut self, sample_rate: usize) -> RecBuffer {
        self.set_sample_rate(sample_rate);
        return self;
    }

    pub fn with_max_length(mut self, length: f32) -> RecBuffer {
        self.set_max_length(length);
        return self;
    }

    pub fn set_sample_rate(&mut self, sample_rate: usize) {
        self.sample_rate = sample_rate;
        
        self.refit();
    }

    pub fn set_max_length(&mut self, length: f32) {
        self.max_length = length;

        self.refit();
    }

    pub fn buffer(&self) -> &VecDeque<f32> {
        return &self.buffer;
    }

    pub fn max_length(&self) -> f32 {
        return self.max_length;
    }

    pub fn length(&self) -> f32 {
        return self.buffer.len() as f32 / self.sample_rate as f32;
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn push(&mut self, input: f32) {
        if self.full() { return; }

        self.buffer.push_back(input);
    }

    pub fn push_mult(&mut self, input: &impl AsRef<[f32]>) {
        if self.full() { return; }

        let input = input.as_ref();

        for i in 0..input.len() {
            self.push(input[i]);
        }
    }

    pub fn pop(&mut self) -> f32 {
        match self.buffer.pop_front() {
            Some(s) => return s,
            None => 0.0,
        }
    }

    pub fn pop_mult(&mut self, mut output: impl AsMut<[f32]>) {
        let output = output.as_mut();

        for i in 0..output.len() {
            output[i] = self.pop();
        }
    }

    fn max_len(&self) -> usize {
        return f32::floor(self.sample_rate as f32 * self.max_length) as usize;
    }

    fn refit(&mut self) {
        self.buffer.truncate(self.max_len());
    }

    fn full(&self) -> bool {
        return self.buffer.len() >= self.max_len();
    }
}

impl Index<usize> for RecBuffer {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        if self.buffer.len() <= 0 { return &0.0; }

        return &self.buffer[index % self.buffer.len()];
    }
}