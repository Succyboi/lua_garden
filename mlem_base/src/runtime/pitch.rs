pub const NOTES_PER_OCTAVE: f32 = 12.0;

pub fn semitone_to_playback_speed(semitone: f32) -> f32 {
    if semitone == 0.0 { return 1.0; }

    return 1.0 * f32::powf(2.0, semitone / NOTES_PER_OCTAVE);
}