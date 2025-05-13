runtime.iterate(function(sample)
    local tilt = Tilt:get_smoothed();
    local resonance = Resonance:get_smoothed();
    local freq = math.lerp(BottomFreq, TopFreq, tilt);

    local low_mix = math.lerp(1.0, 0.0, math.inverse_lerp(0.5, 1.0, tilt));
    local high_mix = math.lerp(0.0, 1.0, math.inverse_lerp(0.0, 0.5, tilt));
    local mix = math.lerp(0.0, 1.0, math.abs(0.5 - tilt) * 2.0);

    for channel = 1, BUFFER.channels do
        Filters[channel] = Filters[channel] or SVF:new(freq, resonance); -- Ensure our filters exist.
        Filters[channel].cutoff = freq;
        Filters[channel].resonance = resonance;

        local input = BUFFER[channel][sample];
        local filter_run = Filters[channel]:run(input);
        local wet = low_mix * filter_run.low + high_mix * filter_run.high;

        BUFFER[channel][sample] = math.lerp(input, wet, mix);
    end
end);