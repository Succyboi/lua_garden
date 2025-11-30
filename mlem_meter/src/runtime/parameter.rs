use mlua::{ prelude::LuaResult, Table };

const LUA_NAME_KEY: &str = "name";
const LUA_VALUE_KEY: &str = "value";
const LUA_MIN_KEY: &str = "min";
const LUA_MAX_KEY: &str = "max";
const LUA_STEP_SIZE_KEY: &str = "step_size";

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

    pub fn new_from_lua(lua_parameter: &Table) -> LuaResult<Parameter> {
        let name: String = lua_parameter.get(LUA_NAME_KEY)?;
        let value: f32 = lua_parameter.get(LUA_VALUE_KEY)?;
        let min: f32 = lua_parameter.get(LUA_MIN_KEY)?;
        let max: f32 = lua_parameter.get(LUA_MAX_KEY)?;
        let step_size: f32 = lua_parameter.get(LUA_STEP_SIZE_KEY)?;

        return Ok(Parameter::new(name, value, min, max, step_size));
    }

    pub fn update_from_parameter(&mut self, parameter: &Parameter) {
        self.value = parameter.value;
    }

    pub fn set_changed(&mut self, changed: bool) {
        self.changed = changed;
    }
}