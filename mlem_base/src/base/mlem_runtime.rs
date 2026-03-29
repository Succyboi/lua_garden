use std::sync::{Arc, atomic::Ordering};

use nih_plug::{buffer::Buffer, prelude::Transport};

use crate::base::{mlem_params::MlemParams};

pub trait MlemRuntime<T: MlemParams> {
    fn params(&self) ->  Arc<T>;

    fn init(&mut self);
    fn reset(&mut self);
    fn run(&mut self, buffer: &mut Buffer, transport: &Transport);

    fn sample_rate(&self) -> f32 {
        return self.params().sample_rate().load(Ordering::Relaxed);
    }

    fn buffer_size(&self) -> usize {
        return self.params().buffer_size().load(Ordering::Relaxed);
    }
    
    fn channels(&self) -> usize {
        return self.params().channels().load(Ordering::Relaxed);
    }
}