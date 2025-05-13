-- Filters.

SVF = {
    cutoff = 0.0,
    resonance = 0.0,

    low = 0.0,
    high = 0.0,
    band = 0.0,
    notch = 0.0,
    cutoff_freq = 0.0,
    damp = 0.0
}

function SVF:new (cutoff, resonance)
    self.__index = self;
    local svf = setmetatable({
        cutoff = cutoff,
        resonance = resonance
    }, self);

    return svf
end

function SVF:run (input)
    self:compute_coeff();
    local low = 0.0;
    local high = 0.0;
    local band = 0.0;
    local notch = 0.0;
    
    self:pass(input);
    low = low + self.low * 0.5;
    high = high + self.high * 0.5;
    band = band + self.band * 0.5;
    notch = notch + self.notch * 0.5;

    self:pass(input);
    low = low + self.low * 0.5;
    high = high + self.high * 0.5;
    band = band + self.band * 0.5;
    notch = notch + self.notch * 0.5;

    return { 
        low = low, 
        high = high, 
        band = band, 
        notch = notch 
    };
end

function SVF:compute_coeff ()
    self.cutoff_freq = 2.0 * math.sin(math.pi * math.min(0.25, self.cutoff / (SAMPLE_RATE * 2.0)));
    self.damp = math.min(2.0 * (1.0 - (self.resonance ^ 0.25)), math.min(2.0, 2.0 / self.cutoff_freq - self.cutoff_freq * 0.5));
end

function SVF:pass (input)
    self.notch = input - self.damp * self.band;
    self.low = self.low + self.cutoff_freq * self.band;
    self.high = self.notch - self.low;
    self.band = self.cutoff_freq * self.high + self.band;
end