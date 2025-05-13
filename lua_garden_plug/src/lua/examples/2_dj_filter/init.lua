MODULE_NAME = "DJ Filter";
MODULE_AUTHORS = "Puk";
MODULE_ABOUT = [[A single knob filter that blends from lowpass to highpass.]];

Tilt = Parameter:new("tilt", 0.5, 0.0, 1.0, 0);
Resonance = Parameter:new("resonance", 0.75, 0.0, 0.9, 0);

BottomFreq = pitch.min_audible_frequency;
TopFreq = pitch.max_audible_frequency;

-- TODO exponential pitch interpolation on tilt.