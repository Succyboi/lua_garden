runtime.iterate(function(sample)
    local crush_interval = SAMPLE_RATE /  Frequency:get_smoothed(); -- The amount of original samples per crushed sample.
    local bit_depth = BitDepth:get_smoothed();

    for channel = 1, BUFFER.channels do 
        Store[channel] = Store[channel] or 0; -- Ensure we're not dealing with nil.
        Ticks[channel] = Ticks[channel] or math.maxinteger; -- Initialize ticks as maxinteger to fill with actual value immediately.

        if Ticks[channel] > crush_interval then
            Store[channel] = BUFFER[channel][sample]; -- Store incoming value.
            Ticks[channel] = Ticks[channel] - math.floor(Ticks[channel]); -- Reset tick.
        end

        BUFFER[channel][sample] = math.floor(Store[channel] * bit_depth) / bit_depth; -- Bit reduce stored value.
        Ticks[channel] = Ticks[channel] + 1; -- Increment tick.
    end
end);