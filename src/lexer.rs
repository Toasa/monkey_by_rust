use crate::token;

struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> token::Token {
        let tok: token::Token = match self.ch {
            '=' => new_token(token::Type::Assign, "="),
            ';' => new_token(token::Type::Semicolon, ";"),
            '(' => new_token(token::Type::Lparen, "("),
            ')' => new_token(token::Type::Rparen, ")"),
            ',' => new_token(token::Type::Comma, ","),
            '+' => new_token(token::Type::Plus, "+"),
            '{' => new_token(token::Type::Lbrace, "{"),
            '}' => new_token(token::Type::Rbrace, "}"),
            '\0' => new_token(token::Type::Eof, ""),
            _ => new_token(token::Type::Eof, ""), // Need to occur a error?
        };

        self.read_char();
        return tok;
    }
}

fn new(input: &str) -> Lexer {
    let chars = input.chars().collect::<Vec<char>>();
    let first_char = chars[0];
    let mut l = Lexer {
        input: chars,
        position: 0,
        read_position: 0,
        ch: first_char,
    };
    l.read_char();
    return l;
}

fn new_token(t: token::Type, lit: &str) -> token::Token {
    return token::Token {
        t: t,
        literal: String::from(lit),
    };
}

#[test]
fn tokenize1() {
    let input = "=+(){},;";

    let expects = [
        new_token(token::Type::Assign, "="),
        new_token(token::Type::Plus, "+"),
        new_token(token::Type::Lparen, "("),
        new_token(token::Type::Rparen, ")"),
        new_token(token::Type::Lbrace, "{"),
        new_token(token::Type::Rbrace, "}"),
        new_token(token::Type::Comma, ","),
        new_token(token::Type::Semicolon, ";"),
    ];

    let mut l = new(input);
    for expect in expects.iter() {
        let tok = l.next_token();
        assert_eq!(tok.t, expect.t);
        assert_eq!(tok.literal, expect.literal);
    }
}
