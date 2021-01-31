use crate::token;

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

pub enum Expr {
    Ident(Ident),
    Int(Int),
    Prefix(Prefix),
}

pub struct Program {
    pub stmts: Vec<Stmt>,
}

pub struct Let {
    pub token: token::Token,
    pub name: Ident,
    // val: Expr,
}

pub struct Return {
    pub token: token::Token,
    // pub val: Expr,
}

pub struct ExprStmt {
    pub token: token::Token,
    pub expr: Expr,
}

pub struct Ident {
    pub token: token::Token,
    pub val: String,
}

pub struct Int {
    pub token: token::Token,
    pub val: isize,
}

pub struct Prefix {
    pub token: token::Token,
    pub op: String,
    pub rhs: Box<Expr>,
}
