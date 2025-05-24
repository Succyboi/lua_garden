-- init.lua
-- Initialize your code. Do intense work and large allocations.
-- 
-- Available globals:
-- SAMPLE_RATE - The sample rate the plugin is running at.
-- MODULE_NAME - This module's name.
-- MODULE_AUTHORS - Who made this module.
-- MODULE_ABOUT - A desciption of the module.

MODULE_NAME = "Explorative experiments";
MODULE_AUTHORS = "Puk";
MODULE_ABOUT = "WIP";

MaxLineLength = 0.2;

-- TODO doesn't handle wrap around

Line = {
    resolution = 0,
}

function Line:new(resolution)
    self.__index = self;
    return setmetatable({
        resolution = resolution
    }, self);
end

function Line:write(position, input)
    position = position % 1;

    local position_abs = position * self.resolution;
    local index_floor = math.floor(position_abs);
    local index_ceil = math.ceil(position_abs);
    local index_t = position_abs - index_floor;

    self[index_floor] = math.lerp(input, 0, index_t);
    self[index_ceil] = math.lerp(0, input, index_t);
end

function Line:read(position)
    position = position % 1;

    local position_abs = position * self.resolution;
    local index_floor = math.floor(position_abs);
    local index_ceil = math.ceil(position_abs);
    local index_t = position_abs - index_floor;

    return math.lerp(self[index_floor] or 0, self[index_ceil] or 0, index_t);
end