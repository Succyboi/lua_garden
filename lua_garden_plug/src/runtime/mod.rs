pub mod module;
pub mod workspace;
pub mod utils;
pub mod library;
pub mod module_content;
pub mod runtime_data;
pub mod parameter;

use crate::console::ConsoleSender;
use module::RuntimeModule;
use module_content::ModuleContent;
use utils::{ Timer, RMS };
use mlua::prelude::*;
use nih_plug::prelude::*;

pub struct Runtime {
    pub console: Option<ConsoleSender>,

    pub name: String,
    pub author: String,
    pub description: String,

    module: Option<RuntimeModule>,

    sample_rate : f32,
    buffer_size : usize,
    channels : usize,

    run_time_rms: RMS,
    input_noise: bool,
    clip: bool
}

impl Runtime {
    pub fn new(console: Option<ConsoleSender>) -> Runtime {
        let runtime = Self {
            console: console,

            name: String::new(),
            author: String::new(),
            description: String::new(),

            module: None,

            sample_rate: 0.0,
            buffer_size: 0,
            channels: 0,

            run_time_rms: RMS::new(),
            input_noise: false,
            clip: true
        };

        return runtime;
    }

    pub fn load_new_module(&mut self, content: ModuleContent) {
        let module = RuntimeModule::new(content, self.sample_rate);
        
        self.load_module(Some(module));
    }

    pub fn load_module(&mut self, module: Option<RuntimeModule>) {
        match module {
            Some(m) => {
                self.log(format!("Loading module... ({})\n", m.hash));
                self.module = Some(m);
            }
            None => {
                self.log(format!("Clearing module..."));
                self.module = None;
            }
        }
    }
    
    pub fn init(&mut self, sample_rate: Option<f32>) -> bool {        
        match sample_rate {
            Some(rate) => self.sample_rate = rate,
            None => (),
        }

        let execute_timer = Timer::new();
        let init_result = self.initialize_lua();
        let execute_time = execute_timer.elapsed_ms();

        match init_result {
            Ok(_r) => {
                self.run_time_rms.set(execute_time);
                self.log(format!("Initialization took {:.2}ms.", execute_time));
                return true;
            },
            Err(e) => {
                self.log(format!("Failed to initialize: {e}"));
                return false;
            }
        }
    }

    pub fn reset(&mut self) -> bool {
        let execute_timer = Timer::new();
        let reset_result = self.reset_lua();

        match reset_result {
            Ok(_r) => {
                self.log(format!("Reset in {:.2}ms.", execute_timer.elapsed_ms()));
                return true;
            },
            Err(e) => {
                self.log(format!("Failed to reset: {e}"));
                return  false;
            }
        }
    }

    pub fn trigger(&mut self) -> bool {
        let execute_timer = Timer::new();
        let trigger_result = self.trigger_lua();

        match trigger_result {
            Ok(_r) => {
                self.log(format!("Triggered in {:.2}ms.", execute_timer.elapsed_ms()));
                return true;
            },
            Err(e) => {
                self.log(format!("Failed to trigger: {e}"));
                return  false;
            }
        }
    }

    pub fn run(&mut self, buffer : &mut Buffer) -> bool {
        let execute_timer = Timer::new();
        let run_result = self.run_lua(buffer);

        match run_result {
            Ok(_r) => {
                self.run_time_rms.process( execute_timer.elapsed_ms(), self.sample_rate);
                return true;
            },
            Err(e) => {
                self.log(format!("Failed to run: {e}"));
                return  false;
            }
        }
    }

    pub fn get_sample_rate(&self) -> f32 {
        return self.sample_rate;
    }

    pub fn get_buffer_size(&self) -> usize {
        return self.buffer_size;
    }

    pub fn get_channels(&self) -> usize {
        return self.channels;
    }

    pub fn get_run_ms(&self) -> f32 {
        return self.run_time_rms.get();
    }

    pub fn set_clip(&mut self, clip: bool) {
        self.clip = clip;
    }

    pub fn set_input_noise(&mut self, input_noise: bool) {
        self.input_noise = input_noise;
    }

    fn initialize_lua(&mut self) -> LuaResult<()> {
        self.log(format!("Setting up Lua state..."));

        match &mut self.module {
            Some(module) => { 
                let init_result = module.init();

                match &init_result {
                    Ok(r) => {
                        self.log(format!("Initialized module:\n{name} by {authors}\n\"{about}\"", 
                            name = r.0, 
                            authors = r.1,
                            about = r.2));

                        self.name = r.0.clone();
                        self.author = r.1.clone();
                        self.description = r.2.clone();
                    },
                    Err(_e) => { 
                        init_result?; 
                    }
                }
            }
            None => self.log(format!("No module loaded."))
        }

        Ok(())
    }

    fn reset_lua(&mut self) -> LuaResult<()> {
        match &mut self.module {
            Some(module) => module.reset()?,
            None => self.log(format!("No module loaded."))
        }

        Ok(())
    }

    fn trigger_lua(&mut self) -> LuaResult<()> {
        match &mut self.module {
            Some(module) => module.trigger()?,
            None => self.log(format!("No module loaded."))
        }
        
        Ok(())
    }

    fn run_lua(&mut self, buffer : &mut Buffer) -> LuaResult<()> {
        self.channels = buffer.channels();
        self.buffer_size = buffer.samples();

        match &mut self.module {
            Some(module) => {
                let logs = module.run(buffer, self.input_noise, self.clip)?;
            
                for log in logs {
                    self.log(log);
                }
            }
            None => self.log(format!("No module loaded."))
        }

        Ok(())
    }

    fn log(&self, message : String) {
        match &self.console {
            Some(c) => {
                c.log(message);
            },
            None => {
                println!("No console exists for Runtime. Log not registered by receiver: {}", message)
            }
        }
    }
}