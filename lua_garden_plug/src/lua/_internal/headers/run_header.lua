-- Ensure globals have values and we have a proper buffer.
SAMPLE_RATE = SAMPLE_RATE or 0;
CHANNELS = CHANNELS or 0;
BUFFER_RAW = BUFFER_RAW or { };
BUFFER_SIZE = BUFFER_SIZE or { };
BUFFER = Buffer:copy(BUFFER_RAW, BUFFER_SIZE, CHANNELS);
INPUT_NOISE = INPUT_NOISE or false;

Parameter.update_values_from_global();

-- ↑↑↑↑ --
-- HEADER
-- ==== --