use std::collections::HashMap;
use eval::value::Value;

#[deriving(Clone, Show)]
pub struct Context<'a> {
    values: HashMap<String, Value<'a>>,
}

#[allow(dead_code)]
impl<'a> Context<'a> {
    pub fn new() -> Context<'a> {
        Context {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value<'a>> {
        self.values.get(&name.into_string())
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Value<'a>> {
        self.values.get_mut(&name.into_string())
    }

    pub fn set(&mut self, name: &str, value: Value<'a>) {
        self.values.insert(name.into_string(), value);
    }
}
