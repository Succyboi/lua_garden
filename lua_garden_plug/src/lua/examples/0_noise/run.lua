-- run.lua
-- Write to buffers here to output.
-- 
-- Available globals:
-- SAMPLE_RATE - The sample rate the plugin is running at.
-- CHANNELS - The channels the plugin is running at.
-- BUFFER - Sample buffer, indexed by channel, then sample.
-- BUFFER_SIZE - The length of each sample buffer.

runtime.iterate(function(sample)
    local amp = Volume:get_smoothed();
    for channel = 1, BUFFER.channels do 
        BUFFER[channel][sample] = gen.noise() * amp;
    end
end);
-- The code above outputs stereo noise, at a low volume.