use nih_plug::{buffer::Buffer, prelude::Transport};

use crate::base::mlem_params::MlemParams;

pub trait MlemRuntime {
    fn init(&mut self, sample_rate: f32);
    fn reset(&mut self);
    fn run(&mut self, buffer: &mut Buffer, params: &dyn MlemParams, transport: &Transport);
}