-- run.lua
-- Write to buffers here to output.
-- 
-- Available globals:
-- SAMPLE_RATE - The sample rate the plugin is running at.
-- CHANNELS - The channels the plugin is running at.
-- BUFFER - Sample buffer, indexed by channel, then sample.
-- BUFFER_SIZE - The length of each sample buffer.

runtime.iterate(function(sample)
    local t_adv = 1 / MaxLineLength / SAMPLE_RATE;

    for channel = 1, BUFFER.channels do
        Lines[channel] = Lines[channel] or Line:new(MaxLineLength * SAMPLE_RATE);
        ReadPositions[channel] = ReadPositions[channel] or 0;
        WritePositions[channel] = WritePositions[channel] or 0;

        ReadPositions[channel] = ReadPositions[channel] + t_adv;        
        local read = Lines[channel]:read(ReadPositions[channel]);

        Lines[channel]:write(WritePositions[channel], BUFFER[channel][sample]);
        WritePositions[channel] = ReadPositions[channel];

        BUFFER[channel][sample] = math.lerp(BUFFER[channel][sample], read, 0.5);
    end
end);