use crate::token;
use std::fmt;

enum Node {
    Program(Program),
    Stmt(Stmt),
    Expr(Expr),
}

pub enum Stmt {
    Let(Let),
    Return(Return),
    ExprStmt(ExprStmt),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Stmt::Let(l) => format!("{}", l),
            Stmt::Return(r) => format!("{}", r),
            Stmt::ExprStmt(es) => format!("{}", es),
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
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Expr::Ident(i) => format!("{}", i),
            Expr::Int(n) => format!("{}", n),
            Expr::Prefix(p) => format!("{}", p),
            Expr::Infix(i) => format!("{}", i),
            Expr::Boolean(b) => format!("{}", b),
        };
        write!(f, "{}", s)
    }
}

pub struct Program {
    pub stmts: Vec<Stmt>,
}

pub struct Let {
    pub token: token::Token,
    pub name: Ident,
    // val: Expr,
}

impl fmt::Display for Let {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "let {} = {};", self.name, self.val)
        write!(f, "let {};", self.name)
    }
}

pub struct Return {
    pub token: token::Token,
    // pub val: Expr,
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "return {};", self.val)
        write!(f, "return ;")
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
