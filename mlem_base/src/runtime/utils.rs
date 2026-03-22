use std::{ops::Index, time::Instant, usize};

use nih_plug_egui::egui::output;

pub fn clip(input: f32) -> f32 {
    return f32::clamp(input, -1.0, 1.0)
}

pub fn nearest_zero_crossing(input: &impl Index<usize, Output = f32>, len: usize, from: usize, max_dist: usize) -> usize {
    let previous = previous_zero_crossing(input, len, from, max_dist);
    let next = next_zero_crossing(input, len, from, max_dist);

    if previous == from {
        return next;
    }

    if next == from {
        return previous;
    }

    if usize::abs_diff(previous, from) < usize::abs_diff(next, from) {
        return previous;
    }

    return next;
}

pub fn next_zero_crossing(input: &impl Index<usize, Output = f32>, len: usize, from: usize, max_dist: usize) -> usize {
    let from_pos = f32::is_sign_positive(input[from]);
    let mut i = from + 1;
    let len = usize::min(len, from + max_dist);

    while i < len {
        if from_pos != f32::is_sign_positive(input[i]) {
            return i;
        }

        i += 1;
    }

    return from;
}

pub fn previous_zero_crossing(input: &impl Index<usize, Output = f32>, len: usize, from: usize, max_dist: usize) -> usize {
    let from_pos = f32::is_sign_positive(input[from]);
    let mut i = from + 1;
    let len = usize::min(len, from + max_dist);

    while i < len {
        if from_pos != f32::is_sign_positive(input[len - i]) {
            return i;
        }

        i += 1;
    }

    return from;
}

pub struct Timer {
    pub instant: Instant
}

impl Timer {
    pub fn new() -> Timer {
        let timer = Self {
            instant: Instant::now()
        };

        return timer;
    }
    
    pub fn reset(&mut self) {
        self.instant = Instant::now();
    }

    pub fn elapsed_ms(&self) -> f32 {
        return self.instant.elapsed().as_nanos() as f32 / 1000000.0;
    }
}

const RMS_SILENT_THRESHOLD: f32 = 0.0000000001;

pub struct RMS {
    pub window_size_ms: f32,
    
    state: f32,
}

impl RMS {
    pub fn new(window_size_ms: f32) -> RMS {
        let rms = Self {
            window_size_ms: window_size_ms,

            state: 0.0
        };

        return rms;
    }

    pub fn reset(&mut self) {
        self.set(0.0);
    }

    pub fn set(&mut self, input: f32) {
        self.state = f32::abs(input * input);
    }

    pub fn get(&self) -> f32 {
        return f32::sqrt(self.state);
    }

    pub fn is_silent(&self) -> bool {
        return self.get() <= RMS_SILENT_THRESHOLD;
    }

    pub fn process(&mut self, input: f32, sample_rate: f32) {
        let power = f32::abs(input * input);
        self.state = power + self.get_coefficient(sample_rate) * (self.state - power);
    }

    fn get_coefficient(&self, sample_rate: f32) -> f32 {
        return f32::exp(-1000.0 / (self.window_size_ms * sample_rate));
    }
}