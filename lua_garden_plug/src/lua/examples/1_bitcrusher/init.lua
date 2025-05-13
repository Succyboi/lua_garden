MODULE_NAME = "Bitcrusher";
MODULE_AUTHORS = "Puk";
MODULE_ABOUT = [[Reduces sample rate and bit depth.
Sounds like an old computer.
TODO PRETTY SURE BIT DEPTH IS WRONG, FIX.]];

Frequency = Parameter:new("frequency", 8000, 1, SAMPLE_RATE, 1);
BitDepth = Parameter:new("bit_depth", 4, 1, 16, 1);