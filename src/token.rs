mod token {
    struct Token {
        t: Type,
    }

    enum Type {
        Illegal,
        Eof,
        Ident,
        Int,
        Assign,
        Plus,
        Comma,
        Semicolon,
        Lparen,
        Rparen,
        Lbrace,
        Rbrace,
        Function,
        Let,
    }

    use std::fmt;
    impl fmt::Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Type::Illegal => write!(f, "ILLEGAL"),
                Type::Eof => write!(f, "EOF"),
                Type::Ident => write!(f, "IDENT"),
                Type::Int => write!(f, "Int"),
                Type::Assign => write!(f, "="),
                Type::Plus => write!(f, "+"),
                Type::Comma => write!(f, ","),
                Type::Semicolon => write!(f, ";"),
                Type::Lparen => write!(f, "("),
                Type::Rparen => write!(f, ")"),
                Type::Lbrace => write!(f, "{{"),
                Type::Rbrace => write!(f, "}}"),
                Type::Function => write!(f, "FUNCTION"),
                Type::Let => write!(f, "LET"),
            }
        }
    }
}

