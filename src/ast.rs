use crate::token;
use std::fmt;

#[allow(dead_code)]
pub enum Node {
    Program(Program),
    Stmt(Stmt),
    Expr(Expr),
}

#[allow(dead_code)]
pub enum Stmt {
    Let(Let),
    Return(Return),
    ExprStmt(ExprStmt),
    Block(Block),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Stmt::Let(l) => format!("{}", l),
            Stmt::Return(r) => format!("{}", r),
            Stmt::ExprStmt(es) => format!("{}", es),
            Stmt::Block(b) => format!("{}", b),
        };
        write!(f, "{}", s)
    }
}

pub enum Expr {
    Ident(Ident),
    Int(Int),
    Prefix(Prefix),
    Infix(Infix),
    Boolean(Boolean),
    If(If),
    Func(Func),
    Call(Call),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Expr::Ident(i) => format!("{}", i),
            Expr::Int(n) => format!("{}", n),
            Expr::Prefix(p) => format!("{}", p),
            Expr::Infix(i) => format!("{}", i),
            Expr::Boolean(b) => format!("{}", b),
            Expr::If(i) => format!("{}", i),
            Expr::Func(f) => format!("{}", f),
            Expr::Call(c) => format!("{}", c),
        };
        write!(f, "{}", s)
    }
}

pub struct Program {
    pub stmts: Vec<Stmt>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.stmts {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

pub struct Let {
    pub token: token::Token,
    pub name: Ident,
    pub val: Expr,
}

impl fmt::Display for Let {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "let {} = {};", self.name, self.val)
    }
}

pub struct Return {
    pub token: token::Token,
    pub val: Expr,
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return {};", self.val)
    }
}

pub struct ExprStmt {
    pub token: token::Token,
    pub expr: Expr,
}

impl fmt::Display for ExprStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}

pub struct Block {
    pub token: token::Token,
    pub stmts: Vec<Stmt>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.stmts {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

pub struct Ident {
    pub token: token::Token,
    pub val: String,
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

pub struct Int {
    pub token: token::Token,
    pub val: isize,
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

pub struct Prefix {
    pub token: token::Token,
    pub op: String,
    pub rhs: Box<Expr>,
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.op, self.rhs)
    }
}

pub struct Infix {
    pub token: token::Token,
    pub lhs: Box<Expr>,
    pub op: String,
    pub rhs: Box<Expr>,
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.lhs, self.op, self.rhs)
    }
}

pub struct Boolean {
    pub token: token::Token,
    pub val: bool,
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

pub struct If {
    pub token: token::Token,
    pub cond: Box<Expr>,
    pub cons: Block,
    pub alt: Option<Block>,
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if {} {}", self.cond, self.cons)?;
        if let Some(alt) = &self.alt {
            write!(f, "{}", alt)?;
        }
        Ok(())
    }
}

pub struct Func {
    pub token: token::Token,
    pub params: Vec<Ident>,
    pub body: Block,
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn")?;
        write!(f, "(")?;
        for param in &self.params {
            write!(f, "{}", param)?;
        }
        write!(f, ")")?;
        write!(f, "{}", self.body)
    }
}

pub struct Call {
    pub token: token::Token,
    pub func: Box<Expr>,
    pub args: Vec<Expr>,
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.func)?;
        write!(f, "(")?;

        let len = self.args.len();
        for (i, arg) in self.args.iter().enumerate() {
            write!(f, "{}", arg)?;
            if i != len - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}
