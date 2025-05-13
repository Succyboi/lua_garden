-- Chromatic pitch calculations.
-- TODO test. Lua's indexes are fucking with me. No clue if this is right.

pitch = {
    tuning_frequency = 440.0,
    base_octave = 4,
    notes_per_octave = 12,
    note_names = { "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#" }
};

pitch.min_audible_frequency = 20.0;     -- Purves D, Augustine GJ, Fitzpatrick D, et al., editors. Sunderland(MA) : Sinauer Associates; 2001.
pitch.max_audible_frequency = 20000.0;  -- Neuroscience. 2nd edition.

function pitch.base_note ()
    return pitch.base_octave * pitch.notes_per_octave;
end

function pitch.note_hz (note)
    return pitch.tuning_frequency * 2.0 ^ ((note - pitch.base_note()) / pitch.notes_per_octave);
end


function pitch.note_to_playback (note)
    if note == 0.0 then
       return 1.0;
    end

    return 1.0 * 2.0 ^ (note / pitch.notes_per_octave);
end

function pitch.closest_note (hz)
    return math.floor(pitch.notes_per_octave * math.log(hz / pitch.tuning_frequency, 2.0) + pitch.base_note());
end

function pitch.closest_frequency (hz) 
    return pitch.note_hz(pitch.closest_note(hz));
end

function pitch.note_to_octave (note)
    return math.floor(note / pitch.notes_per_octave);
end

function pitch.note_name (note)
    local note_in_octave = math.floor(note) % pitch.notes_per_octave;
    return pitch.note_names[note_in_octave];
end