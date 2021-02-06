use std::collections::HashMap;
use crate::object::Object;

pub struct Env {
    pub idents: HashMap<String, Object>,
}

pub fn new() -> Env {
    return Env {
        idents: HashMap::new(),
    };
}

impl Env {
    pub fn get(&self, name: String) -> Option<&Object> {
        return self.idents.get(&name);
    }

    pub fn set(&mut self, name: String, obj: Object) {
        self.idents.insert(name, obj);
    }
}
