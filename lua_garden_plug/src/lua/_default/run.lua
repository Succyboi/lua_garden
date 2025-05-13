-- run.lua
-- Write to buffers here to output.
-- 
-- Available globals:
-- SAMPLE_RATE - The sample rate the plugin is running at.
-- CHANNELS - The channels the plugin is running at.
-- BUFFER - Sample buffer, indexed by channel, then sample.
-- BUFFER_SIZE - The length of each sample buffer.

runtime.iterate(function(sample)
    for channel = 1, BUFFER.channels do 
        -- Code that makes noise goes here.
    end
end);