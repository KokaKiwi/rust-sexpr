use std::collections::HashMap;
use super::value::Value;

pub type Context = HashMap<String, Value>;

pub fn new() -> Context {
    HashMap::new()
}
