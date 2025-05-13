-- ==== --
-- FOOTER
-- ↓↓↓↓ --

for c = 1, BUFFER.channels do 
    for b = 1, BUFFER.size do
        BUFFER_RAW[c][b] = BUFFER[c][b];
    end
end