use nih_plug::util::window;
use nih_plug_egui::egui::output;

pub struct Stretch {
    sample_rate: usize,
    window_size: f32
}

impl Stretch {
    pub fn new() -> Stretch {
        let stretch = Self {
            sample_rate: 0,
            window_size: 0.0
        };

        return stretch;
    }

    pub fn with_sample_rate(mut self, sample_rate: usize) -> Stretch {
        self.set_sample_rate(sample_rate);
        return self;
    }

    pub fn with_max_length(mut self, window_size: f32) -> Stretch {
        self.set_max_length(window_size);
        return self;
    }

    pub fn set_sample_rate(&mut self, sample_rate: usize) {
        self.sample_rate = sample_rate;
    }

    pub fn set_max_length(&mut self, window_size: f32) {
        self.window_size = window_size;
    }

    pub fn process(&self, input: impl AsRef<[f32]>, mut output: impl AsMut<[f32]>) {
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
    }

    fn window_len(&self) -> usize {
        return f32::floor(self.window_size * self.sample_rate as f32) as usize;
    }
}