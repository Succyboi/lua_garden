-- Audio generators. Things that produce sound.

gen = { };

function gen.sine (phase)
    return math.sin(phase * math.pi)
end

function gen.tri (phase)
    local f = phase % 2
    if f < 0.5 then
        return 2 * f
    elseif f < 1.5 then
        return -2 * f + 2
    else
        return 2 * f - 4
    end
end

function gen.square (phase)
    if phase % 2 < 1 then
        return 1
    else
        return -1
    end
end

function gen.sawUp (phase)
    local f = (phase + 1) % 2
    return f - 1
end

function gen.sawDown (phase)
    return -gen.sawUp(phase)
end

function gen.noise ()
    return math.random(-1, 1);
end