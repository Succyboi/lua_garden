-- Additions to lua's math library.

function math.sign (value)
    if value > 0.0 then
        return 1.0;
    else
        return -1.0
    end
end

function math.pow (value, power)
    return value ^ power;
end

function math.round (value)
    return math.floor(value + 0.5 * math.sign(value));
end

function math.clamp(value, min, max)
    if value < min then
        return min;
    elseif value > max then
        return max;
    else
        return value;
    end
end

function math.clamp01 (value)
    return math.clamp(value, 0, 1);
end

function math.lerp (a, b, t) 
    return a + (b - a) * math.clamp01(t);
end

function math.inverse_lerp (a, b, value)
    if a ~= b then
        return math.clamp01((value - a) / (b - a));
    end
end

function math.tense_lerp (a, b, t, tension)
    local c = 1.0;
    local d = 1.0;
    local e = 1.0;

    -- In exponential.
    local t0 = (t == 0) and c or d * (2 ^ (10 * (t / e - 1))) + c;

    -- Out exponential.
    local t1 = (t == e) and c + d or d * (-(2 ^ (-10 * t / e)) + 1) + c;

    return math.lerp(a, b, math.lerp(t0, t1, tension));
end

function math.move_towards (current, target, maxDelta)
    if math.abs(target - current) <= maxDelta then
        return target;
    end

    return current + math.sign(target - current) * maxDelta;
end

function math.linear_to_db (linear)
    if linear <= 0.0 then
        return -114.0;
    end

    return 20.0 * math.log(linear, 10);
end

function math.db_to_linear (db)
    return 10.0 ^ (db / 20.0);
end