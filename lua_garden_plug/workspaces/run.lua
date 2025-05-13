runtime.iterate(function(sample)
    local hardness = Hardness:get_smoothed();
    for channel = 1, BUFFER.channels do 
        local input = BUFFER[channel][sample];

        BUFFER[channel][sample] = input * (math.abs(input) + hardness) / (input * input + (hardness - 1) * math.abs(input) + 1);
    end
end);