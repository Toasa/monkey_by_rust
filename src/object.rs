use std::fmt;

pub enum Object {
    Int(Int),
    Bool(Bool),
    Null(Null),
}

pub struct Int {
    pub val: isize,
}

pub struct Bool {
    pub val: bool,
}

pub struct Null {}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Object::Int(i) => write!(f, "{}", i.val.to_string()),
            Object::Bool(b) => write!(f, "{}", b.val.to_string()),
            Object::Null(_) => write!(f, "null"),
        };
    }
}
