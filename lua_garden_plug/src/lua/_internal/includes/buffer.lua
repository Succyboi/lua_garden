-- A container for passing around audio data.

Buffer = {
    size = 0,
    channels = 0,
}

function Buffer:new(size, channels)
    self.__index = self;
    return setmetatable({
        size = size,
        channels = channels,
    }, self);
end

function Buffer:copy(from_buffer, from_buffer_size, from_buffer_channels)
    local buffer = Buffer:new();
    buffer.size = from_buffer_size;
    buffer.channels = from_buffer_channels;

    for c = 1, buffer.channels do 
        buffer[c] = { };
        for b = 1, buffer.size do
            buffer[c][b] = from_buffer[c][b];
        end
    end

    return buffer;
end