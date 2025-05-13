-- init.lua
-- Initialize your code. Do intense work and large allocations.
-- 
-- Available globals:
-- SAMPLE_RATE - The sample rate the plugin is running at.
-- MODULE_NAME - This module's name.
-- MODULE_AUTHORS - Who made this module.
-- MODULE_ABOUT - A desciption of the module.

MODULE_NAME = "Noise";
MODULE_AUTHORS = "Puk";
MODULE_ABOUT = "Outputs stereo noise, at a low volume.";

Volume = Parameter:new("volume", 0.1, 0, 1, 0);