-- Represents the runtime.

LOGS = { };
LOG_COUNT = 0;

runtime = { };

function runtime.log (log)
    LOG_COUNT = LOG_COUNT + 1;
    LOGS[LOG_COUNT] = tostring(log);
end

function runtime.iterate (tick)
    local start_tick = TICK;
    for b = 1, BUFFER.size do
        TICK = start_tick + b

        if INPUT_NOISE then
            for c = 1, BUFFER.channels do 
                BUFFER[c][b] = gen.noise() * 0.1;
            end
        end

        tick(b);
    end
end