#[derive(Clone)]
pub struct Token {
    pub t: Type,
    pub literal: String,
}

#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum Type {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Equ,
    Neq,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

