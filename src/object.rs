use std::fmt;
use crate::ast;
use crate::env;

#[derive(Clone)]
pub enum Object {
    Int(Int),
    Bool(Bool),
    Null(Null),
    Return(Return),
    Func(Func),
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

#[derive(Clone)]
pub struct Func {
    pub params: Vec<ast::Ident>,
    pub body: ast::Block,
    pub env: env::Env,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Object::Int(i) => write!(f, "{}", i.val.to_string()),
            Object::Bool(b) => write!(f, "{}", b.val.to_string()),
            Object::Null(_) => write!(f, "null"),
            Object::Return(r) => write!(f, "{}", r.val),
            Object::Func(func) => {
                write!(f, "fn")?;
                write!(f, "(")?;
                let args = &func.params.len();
                let mut i = 0;
                for param in &func.params {
                    write!(f, "{}", param)?;
                    if i != args - 1 {
                        write!(f, ", ")?;
                    }
                    i += 1;
                }
                write!(f, ")")?;
                write!(f, "{}", func.body)
            },
        };
    }
}
