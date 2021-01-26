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
}

