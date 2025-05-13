use std::hash::{ DefaultHasher, Hasher };
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

const ENCODING_SEPERATOR: char = '\\';

#[derive(PartialEq, Clone)]
pub struct ModuleContent {
    pub init: String,
    pub reset: String,
    pub trigger: String,
    pub run: String,
    pub interface: String
}

pub struct ConstModuleContent<'a> {
    pub init: &'a str,
    pub reset: &'a str,
    pub trigger: &'a str,
    pub run: &'a str,
    pub interface: &'a str
}

impl ModuleContent {
    pub fn new(init: String, reset: String, trigger: String, run: String, interface: String) -> ModuleContent {
        let content = Self {
            init,
            reset,
            trigger,
            run,
            interface
        };

        return content;
    }

    pub fn generate_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write(self.init.as_bytes());
        hasher.write(self.reset.as_bytes());
        hasher.write(self.trigger.as_bytes());
        hasher.write(self.run.as_bytes());
        
        return hasher.finish();
    }

    pub fn to_base64(&self) -> String{
        let hash = format!("{:x}", self.generate_hash());
        let init_enc = URL_SAFE.encode(self.init.clone());
        let reset_enc = URL_SAFE.encode(self.reset.clone());
        let trigger_enc = URL_SAFE.encode(self.trigger.clone());
        let run_enc = URL_SAFE.encode(self.run.clone());
        let interface_enc = URL_SAFE.encode(self.interface.clone());

        let base64 = format!("{hash}{sp}{init_enc}{sp}{reset_enc}{sp}{trigger_enc}{sp}{run_enc}{sp}{interface_enc}", 
            hash = hash,
            sp = ENCODING_SEPERATOR,
            init_enc = init_enc,
            reset_enc = reset_enc,
            trigger_enc = trigger_enc,
            run_enc = run_enc,
            interface_enc = interface_enc);
            
        return base64;
    }
}

impl<'a> ConstModuleContent<'a> {
    pub const fn new(init: &'a str, reset: &'a str, trigger: &'a str, run: &'a str, interface: &'a str) -> ConstModuleContent<'a> {
        Self {
            init,
            reset,
            trigger,
            run,
            interface
        }
    }

    pub fn to_module_content(&self) -> ModuleContent {
        return ModuleContent::new(
            String::from(self.init), 
            String::from(self.reset), 
            String::from(self.trigger), 
            String::from(self.run),
            String::from(self.interface));
    }
}