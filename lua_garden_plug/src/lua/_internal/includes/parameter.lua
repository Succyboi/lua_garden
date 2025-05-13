-- Parameters for smoothed values.

PARAMETERS = { };
PARAMETER_VALUE_UPDATES = nil;

Parameter = {
    name = "",

    value = 0,
    min = 0,
    max = 1,
    step_size = 0,
    smoothing_ms = 10.0,

    old_value = 0,
    set_tick = 0
}

function Parameter:new (name, value, min, max, step_size, smoothing_ms)
    self.__index = self;
    local parameter = setmetatable({
        name = name,

        value = math.clamp(value, min, max),
        min = min,
        max = max,
        step_size = step_size or 0.0,
        smoothing_ms = smoothing_ms or 10.0,

        old_value = value,
        set_tick = TICK or 0
    }, self);

    parameter:register();

    return parameter
end

function Parameter:register ()
    if PARAMETERS[self.name] ~= nil then
        runtime.log(string.format("A parameter with the name \"%s\" is already registered. Parameter names must be unique. The new parameter will not be registered.", self.name));
        return;
    end
    
    runtime.log(string.format("Registered parameter \"%s\".", self.name));
    PARAMETERS[self.name] = self;
end

function Parameter:set_value (value)
    self.value = math.clamp(value, self.min, self.max);
    self.old_value = self:get_smoothed();
    self.set_tick = TICK;
end

function Parameter:get_smoothed ()
    local smoothing_samples = SAMPLE_RATE / 1000.0 * self.smoothing_ms;
    local t = math.clamp((TICK - self.set_tick) / smoothing_samples, 0.0, 1.0);
    local smooth = math.lerp(self.old_value, self.value, t);

    if self.step_size <= 0 then
        return smooth;
    else
        return math.floor(smooth / self.step_size) * self.step_size;
    end
end

function Parameter:get_raw ()
    if self.step_size > 0 then
        return self.value;
    else
        return math.floor(self.value / self.step_size) * self.step_size;
    end
end

function Parameter.update_values_from_global ()
    if PARAMETER_VALUE_UPDATES == nil then
        return;
    end

    for p_name, p_value in pairs(PARAMETER_VALUE_UPDATES) do
        PARAMETERS[p_name]:set_value(p_value);
    end

    PARAMETER_VALUE_UPDATES = nil;
end