use std::f32::consts::PI;

use mlem_base::runtime::utils::{clip, lerp};

pub fn clip_lerp(input: f32, amount: f32) -> f32 {
    if amount == 0.0 { return input; }

    let dry = input; 
    let wet = if input == 0.0 { 0.0 } else { f32::signum(input) };

    return lerp(dry, wet, amount);
}

pub fn waveshaper(input: f32, amount: f32) -> f32 {
    if amount == 0.0 { return input; }

    let amount = lerp(1.0, 100.0, amount);

    return input * (f32::abs(input) + amount) / (input * input + (amount - 1.0) * f32::abs(input) + 1.0);
}

pub fn fold(input: f32, amount: f32) -> f32 {
    if amount == 0.0 { return input; }

    let input = input * lerp(1.0, 10.0, amount);

    return input % 1.0;
}

pub fn quant(input: f32, amount: f32) -> f32 {
    if amount == 0.0 { return input; }

    let amount = lerp(32.0, 1.0, amount);

    return f32::round(input * amount);
}

pub fn chomper(input: f32, amount: f32) -> f32 {
    if amount == 0.0 { return input; }

    let dry = input; 
    let wet = clip(1.5 * input - 0.7 * input.powf(3.0));

    return lerp(dry, wet, amount);
}

pub fn sine(input: f32, amount: f32) -> f32 {
    if amount == 0.0 { return input; }

    let dry = input; 
    let wet = (PI / 2.0 * input).sin();

    return lerp(dry, wet, amount);
}

pub fn stepper(input: f32, amount: f32) -> f32 {
    if amount == 0.0 { return input; }

    let dry = input; 
    let wet = clip(0.5 * (input * (input * 2.0 * PI).cos() + input));

    return lerp(dry, wet, amount);
}

pub fn humpback(input: f32, amount: f32) -> f32 {
    if amount == 0.0 { return input; }

    let dry = input; 
    let wet = clip(0.14 * input.powf(5.0) - 1.15 * input.powf(3.0) + 1.9 * input);

    return lerp(dry, wet, amount);
}