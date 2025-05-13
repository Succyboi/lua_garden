use std::{ fs::{self, File}, io::{self, Write} };
use super::{ library, module_content::ModuleContent };

#[derive(Clone, PartialEq)]
pub struct Workspace {
    pub path: String,
    pub content: ModuleContent
}

impl Workspace {
    pub fn create_at_path(path: String, content: Option<ModuleContent>) -> Result<Workspace, String> {
        let content = match content {
            Some(c) => c,
            None => library::MODULE_DEFAULT.to_module_content()
        };

        match Workspace::create_files(&path, &content) {
            Err(e) => return Err(format!("Failed to create workspace folder: {}", e)),
            Ok(_) => ()
        }

        let workspace = Self {
            path: path,
            content: content
        };

        return Ok(workspace);
    }

    pub fn load_from_path(path: String) -> Result<Workspace, String> {
        let mut workspace = Self {
            path: path,
            
            content: library::MODULE_DEFAULT.to_module_content()
        };

        match workspace.read_files() {
            Err(e) => return Err(format!("Failed to load workspace from folder: {}", e)),
            Ok(_) => ()
        }

        return Ok(workspace);
    }

    pub fn update(&mut self) -> Result<(), String> {
        match self.read_files() {
            Err(e) => return Err(format!("Couldn't read from workspace: {}", e)),
            Ok(_) => ()
        }

        Ok(())
    }

    fn create_files(path: &String, content :&ModuleContent) -> io::Result<()> {
        fs::create_dir_all(path)?;

        let mut init_file = File::create(format!("{path}/{file}", path = path, file = library::INIT_PATH))?;
        init_file.write_all(content.init.as_bytes())?;
        let mut reset_file = File::create(format!("{path}/{file}", path = path, file = library::RESET_PATH))?;
        reset_file.write_all(content.reset.as_bytes())?;
        let mut trigger_file = File::create(format!("{path}/{file}", path = path, file = library::TRIGGER_PATH))?;
        trigger_file.write_all(content.trigger.as_bytes())?;       
        let mut run_file = File::create(format!("{path}/{file}", path = path, file = library::RUN_PATH))?;
        run_file.write_all(content.run.as_bytes())?;
        let mut interface_file = File::create(format!("{path}/{file}", path = path, file = library::INTERFACE_PATH))?;
        interface_file.write_all(content.interface.as_bytes())?;

        Ok(())
    }

    fn read_files(&mut self) -> io::Result<()> {
        self.content.init = fs::read_to_string(format!("{path}/{file}", path = &self.path, file = library::INIT_PATH))?;
        self.content.reset = fs::read_to_string(format!("{path}/{file}", path = &self.path, file = library::RESET_PATH))?;
        self.content.trigger = fs::read_to_string(format!("{path}/{file}", path = &self.path, file = library::TRIGGER_PATH))?;
        self.content.run = fs::read_to_string(format!("{path}/{file}", path = &self.path, file = library::RUN_PATH))?;
        self.content.interface = fs::read_to_string(format!("{path}/{file}", path = &self.path, file = library::INTERFACE_PATH))?;

        Ok(())
    }
}