use std::fmt;

#[derive(Clone)]
pub enum Object {
    Int(Int),
    Bool(Bool),
    Null(Null),
    Return(Return),
}

#[derive(Clone)]
pub struct Int {
    pub val: isize,
}

#[derive(Clone)]
pub struct Bool {
    pub val: bool,
}

#[derive(Clone)]
pub struct Null {}

#[derive(Clone)]
pub struct Return {
    pub val: Box<Object>,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Object::Int(i) => write!(f, "{}", i.val.to_string()),
            Object::Bool(b) => write!(f, "{}", b.val.to_string()),
            Object::Null(_) => write!(f, "null"),
            Object::Return(r) => write!(f, "{}", r.val),
        };
    }
}
