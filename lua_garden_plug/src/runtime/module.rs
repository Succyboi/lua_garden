use std::collections::BTreeMap;

use mlua::prelude::*;
use nih_plug::prelude::*;
use crate::runtime::module_content::ModuleContent;

use super::{library, parameter::Parameter, utils};

pub const LUA_BUFFERS_KEY: &str = "BUFFER_RAW";
pub const LUA_SAMPLE_RATE_KEY: &str = "SAMPLE_RATE";
pub const LUA_CHANNELS_KEY: &str = "CHANNELS";
pub const LUA_BUFFER_SIZE_KEY: &str = "BUFFER_SIZE";
pub const LUA_INPUT_NOISE_KEY: &str = "INPUT_NOISE";
pub const LUA_NAME_KEY: &str = "MODULE_NAME";
pub const LUA_AUTHORS_KEY: &str = "MODULE_AUTHORS";
pub const LUA_ABOUT_KEY: &str = "MODULE_ABOUT";
pub const LUA_LOGS_KEY: &str = "LOGS";
pub const LUA_PARAMETERS_KEY: &str = "PARAMETERS";
pub const LUA_PARAMETER_VALUE_UPDATES_KEY: &str = "PARAMETER_VALUE_UPDATES";
const UNKNOWN: &str = "???";

pub struct RuntimeModule {
    pub hash: String,
    
    lua: Lua,
    lua_buffers: LuaTable,
    channels: usize,

    content: ModuleContent
}

impl RuntimeModule {
    pub fn new(content: ModuleContent, sample_rate : f32) -> RuntimeModule {
        let lua = Lua::new();
        let lua_buffers = lua.create_table().expect("Couldn't create buffers.");

        let mut module = Self {
            hash: String::new(),

            lua: lua,
            lua_buffers: lua_buffers,
            channels: 0,

            content: content
        };

        module.lua.globals().set(LUA_BUFFERS_KEY, &module.lua_buffers).expect("Couldn't set global.");
        module.lua.globals().set(LUA_SAMPLE_RATE_KEY, sample_rate).expect("Couldn't set global.");

        module.hash = format!("{:x}", module.content.generate_hash());

        return module;
    }

    pub fn init(&mut self) -> LuaResult<(String, String, String)> {
        let init_contents = format!("{internal}\n{header}\n\n{content}\n\n{footer}", 
            internal = library::internal_includes(), 
            header = library::INIT_HEADER, 
            content = &self.content.init,
            footer = library::INIT_FOOTER);
        self.lua.load(init_contents).exec()?;

        // Read additional data
        let globals = self.lua.globals();
        let mut name = String::from(UNKNOWN);
        let mut authors = String::from(UNKNOWN);
        let mut about = String::from(UNKNOWN);

        if globals.contains_key(LUA_NAME_KEY)? {
            name = globals.get(LUA_NAME_KEY)?;
        }
        if globals.contains_key(LUA_AUTHORS_KEY)? {
            authors = globals.get(LUA_AUTHORS_KEY)?;
        }
        if globals.contains_key(LUA_ABOUT_KEY)? {
            about = globals.get(LUA_ABOUT_KEY)?;
        }
        
        Ok((name, authors, about))
    }

    pub fn reset(&mut self) -> LuaResult<()> {
        let reset_contents = format!("{header}\n\n{content}\n\n{footer}", 
            header = library::RESET_HEADER, 
            content = &self.content.reset,
            footer = library::RESET_FOOTER);
        self.lua.load(reset_contents).exec()?;
        
        Ok(())
    }

    pub fn trigger(&mut self) -> LuaResult<()> {
        let trigger_contents = format!("{header}\n\n{content}\n\n{footer}", 
            header = library::TRIGGER_HEADER, 
            content = &self.content.trigger,
            footer = library::TRIGGER_FOOTER);
        self.lua.load(trigger_contents).exec()?;
        
        Ok(())
    }

    pub fn run(&mut self, buffer : &mut Buffer, input_noise: bool, clip: bool) -> LuaResult<Vec<String>> {
        self.lua.globals().set(LUA_CHANNELS_KEY,buffer.channels())?;
        self.lua.globals().set(LUA_BUFFER_SIZE_KEY,buffer.samples())?;
        self.lua.globals().set(LUA_INPUT_NOISE_KEY, input_noise)?;
        
        for c in 0..buffer.channels() {
            if self.channels <= c {
                let buffer = self.lua.create_table()?;
                self.lua_buffers.set(c + 1, buffer)?; // Lua indexes start at 1
                self.channels += 1;
            }
        }

        // Write to lua buffers
        let mut channel_sample_index = 1; // Lua indexes start at 1
        for channel_samples in buffer.iter_samples() {            
            let mut channel = 0;
            for sample in channel_samples {
                let channel_buffer : LuaTable = self.lua_buffers.get(channel + 1)?; // Lua indexes start at 1
                channel_buffer.set(channel_sample_index, *sample)?;

                channel += 1;
            }

            channel_sample_index += 1;
        }
        
        // Execute lua run
        let run_contents = format!("{header}\n\n{content}\n\n{footer}", 
            header = library::RUN_HEADER, 
            content = &self.content.run,
            footer = library::RUN_FOOTER);
        self.lua.load(run_contents).exec()?;

        // Write from lua buffers to plugin buffer
        let mut channel_sample_index = 1; // Lua indexes start at 1
        for channel_samples in buffer.iter_samples() {                        
            let mut channel = 0;
            for sample in channel_samples {
                let channel_buffer: LuaTable = self.lua_buffers.get(channel + 1)?;
                *sample = if clip {
                    utils::clip(channel_buffer.get(channel_sample_index)?)
                } else {
                    channel_buffer.get(channel_sample_index)?
                };

                channel += 1;
            }

            channel_sample_index += 1;
        }
        
        return self.process_logs();
    }

    pub fn get_parameters(&mut self) -> LuaResult<LuaTable> {
        return Ok(self.lua.globals().get(LUA_PARAMETERS_KEY)?);
    }

    pub fn update_parameter_value_updates(&mut self, parameters: &mut BTreeMap<String, Parameter>) -> LuaResult<()> {
        let updates_table = self.lua.create_table()?;

        for parameter in parameters {
            if !parameter.1.changed { continue; }

            updates_table.set(parameter.0.clone(), parameter.1.value)?;
            parameter.1.set_changed(false);
        }

        return Ok(self.lua.globals().set(LUA_PARAMETER_VALUE_UPDATES_KEY, updates_table)?);
    }

    fn process_logs(&mut self) -> LuaResult<Vec<String>> {
        // Get logs
        let mut logs = Vec::new();
        let lua_logs: LuaTable = self.lua.globals().get(LUA_LOGS_KEY)?;

        for log_pair in lua_logs.pairs::<String, String>() {
            let (_key, value) = log_pair?;
            logs.push(value);
        }

        // Clear logs
        self.lua.globals().set(LUA_LOGS_KEY, self.lua.create_table()?)?;

        Ok(logs)
    }
}