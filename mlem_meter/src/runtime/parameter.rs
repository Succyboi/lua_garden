#[derive(Clone)]
pub struct Parameter {
    pub name: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step_size: f32,
    
    pub changed: bool
}

impl Parameter {
    pub fn new(name: String, value: f32, min: f32, max: f32, step_size: f32) -> Parameter {
        Self {
            name: name,
            value: value,
            min: min,
            max: max,
            step_size: step_size,
            
            changed: false
        }
    }

    pub fn update_from_parameter(&mut self, parameter: &Parameter) {
        self.value = parameter.value;
    }

    pub fn set_changed(&mut self, changed: bool) {
        self.changed = changed;
    }
}