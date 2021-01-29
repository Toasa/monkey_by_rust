use crate::token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    next_pos: usize,
    ch: char,
}

impl Lexer {
    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.next_pos];
        }
        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_space();

        let tok: token::Token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    new_token(token::Type::Equ, "==")
                } else {
                    new_token(token::Type::Assign, "=")
                }
            },
            ';' => new_token(token::Type::Semicolon, ";"),
            '(' => new_token(token::Type::Lparen, "("),
            ')' => new_token(token::Type::Rparen, ")"),
            ',' => new_token(token::Type::Comma, ","),
            '+' => new_token(token::Type::Plus, "+"),
            '-' => new_token(token::Type::Minus, "-"),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    new_token(token::Type::Neq, "!=")
                } else {
                    new_token(token::Type::Bang, "!")
                }
            },
            '/' => new_token(token::Type::Slash, "/"),
            '*' => new_token(token::Type::Asterisk, "*"),
            '<' => new_token(token::Type::Lt, "<"),
            '>' => new_token(token::Type::Gt, ">"),
            '{' => new_token(token::Type::Lbrace, "{"),
            '}' => new_token(token::Type::Rbrace, "}"),
            '\0' => new_token(token::Type::Eof, ""),
            _ => {
                if is_letter(self.ch) {
                    let lit = &self.read_identifier();
                    let t = look_up_ident(lit);
                    return new_token(t, lit);
                } else if is_digit(self.ch) {
                    let lit = &self.read_number();
                    return new_token(token::Type::Int, lit);
                } else {
                    new_token(token::Type::Illegal, &self.ch.to_string())
                }
            },
        };

        self.read_char();
        return tok;
    }

    fn skip_space(&mut self) {
        while is_space(self.ch) {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let from = self.pos;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.extract_token(from, self.pos)
    }

    fn read_number(&mut self) -> String {
        let from = self.pos;
        while is_digit(self.ch) {
            self.read_char();
        }
        self.extract_token(from, self.pos)
    }

    // extract token by indexing [i..j) from self.input.
    fn extract_token(&mut self, i: usize, j: usize) -> String {
        let token: String = self.input[i..j].iter().collect();
        token
    }

    fn peek_char(&self) -> char {
        if self.next_pos >= self.input.len() {
            return '\0';
        } else {
            return self.input[self.next_pos];
        }
    }
}

pub fn new(input: &str) -> Lexer {
    let chars = input.chars().collect::<Vec<char>>();
    let first_char = chars[0];

    let mut l = Lexer {
        input: chars,
        pos: 0,
        next_pos: 0,
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

fn look_up_ident(ident: &String) -> token::Type {
    if ident == "fn" {
        return token::Type::Function;
    } else if ident == "let" {
        return token::Type::Let;
    } else if ident == "true" {
        return token::Type::True;
    } else if ident == "false" {
        return token::Type::False;
    } else if ident == "if" {
        return token::Type::If;
    } else if ident == "else" {
        return token::Type::Else;
    } else if ident == "return" {
        return token::Type::Return;
    } else {
        return token::Type::Ident;
    }
}

fn is_letter(c: char) -> bool {
    return 'a' <= c && c <= 'z' ||
           'A' <= c && c <= 'Z' ||
           c == '_';
}

fn is_digit(c: char) -> bool {
    return '0' <= c && c <= '9';
}

fn is_space(c: char) -> bool {
    return c == ' ' || c == '\t' ||
           c == '\n' || c == '\r'
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
        new_token(token::Type::Eof, ""),
    ];

    let mut l = new(input);
    for expect in expects.iter() {
        let tok = l.next_token();
        assert_eq!(tok.t, expect.t);
        assert_eq!(tok.literal, expect.literal);
    }
}

#[test]
fn tokenize2() {
    let input = "let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        ";

    let expects = [
        new_token(token::Type::Let, "let"),
        new_token(token::Type::Ident, "five"),
        new_token(token::Type::Assign, "="),
        new_token(token::Type::Int, "5"),
        new_token(token::Type::Semicolon, ";"),
        new_token(token::Type::Let, "let"),
        new_token(token::Type::Ident, "ten"),
        new_token(token::Type::Assign, "="),
        new_token(token::Type::Int, "10"),
        new_token(token::Type::Semicolon, ";"),
        new_token(token::Type::Let, "let"),
        new_token(token::Type::Ident, "add"),
        new_token(token::Type::Assign, "="),
        new_token(token::Type::Function, "fn"),
        new_token(token::Type::Lparen, "("),
        new_token(token::Type::Ident, "x"),
        new_token(token::Type::Comma, ","),
        new_token(token::Type::Ident, "y"),
        new_token(token::Type::Rparen, ")"),
        new_token(token::Type::Lbrace, "{"),
        new_token(token::Type::Ident, "x"),
        new_token(token::Type::Plus, "+"),
        new_token(token::Type::Ident, "y"),
        new_token(token::Type::Semicolon, ";"),
        new_token(token::Type::Rbrace, "}"),
        new_token(token::Type::Semicolon, ";"),
        new_token(token::Type::Let, "let"),
        new_token(token::Type::Ident, "result"),
        new_token(token::Type::Assign, "="),
        new_token(token::Type::Ident, "add"),
        new_token(token::Type::Lparen, "("),
        new_token(token::Type::Ident, "five"),
        new_token(token::Type::Comma, ","),
        new_token(token::Type::Ident, "ten"),
        new_token(token::Type::Rparen, ")"),
        new_token(token::Type::Semicolon, ";"),
        new_token(token::Type::Bang, "!"),
        new_token(token::Type::Minus, "-"),
        new_token(token::Type::Slash, "/"),
        new_token(token::Type::Asterisk, "*"),
        new_token(token::Type::Int, "5"),
        new_token(token::Type::Semicolon, ";"),
        new_token(token::Type::Int, "5"),
        new_token(token::Type::Lt, "<"),
        new_token(token::Type::Int, "10"),
        new_token(token::Type::Gt, ">"),
        new_token(token::Type::Int, "5"),
        new_token(token::Type::Semicolon, ";"),
        new_token(token::Type::If, "if"),
        new_token(token::Type::Lparen, "("),
        new_token(token::Type::Int, "5"),
        new_token(token::Type::Lt, "<"),
        new_token(token::Type::Int, "10"),
        new_token(token::Type::Rparen, ")"),
        new_token(token::Type::Lbrace, "{"),
        new_token(token::Type::Return, "return"),
        new_token(token::Type::True, "true"),
        new_token(token::Type::Semicolon, ";"),
        new_token(token::Type::Rbrace, "}"),
        new_token(token::Type::Else, "else"),
        new_token(token::Type::Lbrace, "{"),
        new_token(token::Type::Return, "return"),
        new_token(token::Type::False, "false"),
        new_token(token::Type::Semicolon, ";"),
        new_token(token::Type::Rbrace, "}"),
        new_token(token::Type::Int, "10"),
        new_token(token::Type::Equ, "=="),
        new_token(token::Type::Int, "10"),
        new_token(token::Type::Semicolon, ";"),
        new_token(token::Type::Int, "10"),
        new_token(token::Type::Neq, "!="),
        new_token(token::Type::Int, "9"),
        new_token(token::Type::Semicolon, ";"),
        new_token(token::Type::Eof, ""),
    ];

    let mut l = new(input);
    for expect in expects.iter() {
        let tok = l.next_token();
        assert_eq!(tok.t, expect.t);
        assert_eq!(tok.literal, expect.literal);
    }
}
