use crate::ast::{
    Program,
    Stmt,
    Expr,
    Let,
    Ident,
};
use crate::lexer;
use crate::token;

struct Parser<'a> {
    l: &'a mut lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
}

fn new(l: &mut lexer::Lexer) -> Parser {
    let first_token = l.next_token();
    let second_token = l.next_token();
    let p = Parser{
        l: l,
        cur_token: first_token,
        peek_token: second_token,
    };
    p
}

impl Parser<'_> {
    fn parse_program(&mut self) -> Program {
        let stmts: Vec<Stmt> = vec![];
        let mut p = Program {
            stmts: stmts,
        };

        while self.cur_token.t != token::Type::Eof {
            let stmt = self.parse_stmt();
            if let Some(s) = stmt {
                p.stmts.push(s);
            }
            self.next_token();
        }
        return p;
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.cur_token.t {
            token::Type::Let => {
                if let Some(ls) = self.parse_let_stmt() {
                    return Some(Stmt::Let(ls));
                }
                return None;
            },
            _ => return None,
        };
    }

    fn parse_let_stmt(&mut self) -> Option<Let> {
        let t = self.cur_token.clone();

        if !self.expect_peek(token::Type::Ident) {
            return None;
        }

        let ident = Ident {
            token: self.cur_token.clone(),
            val: self.cur_token.clone().literal,
        };

        let stmt = Let {
            token: t,
            name: ident,
        };

        if !self.expect_peek(token::Type::Assign) {
            return None;
        }

        while !self.cur_token_is(token::Type::Semicolon) {
            self.next_token();
        }

        return Some(stmt);
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn cur_token_is(&self, t: token::Type) -> bool {
        return self.cur_token.t == t;
    }

    fn peek_token_is(&self, t: token::Type) -> bool {
        return self.peek_token.t == t;
    }

    fn expect_peek(&mut self, t: token::Type) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        }
        return false;
    }
}

#[test]
fn let_stmts() {
    let input = "let x = 5;
        let y = 10;
        let foobar = 838383;";

    let mut l = lexer::new(input);
    let mut p = new(&mut l);

    let program = p.parse_program();

    assert_eq!(program.stmts.len(), 3);

    let idents = [
        "x", "y", "foobar"
    ];

    for (i, stmt) in program.stmts.iter().enumerate() {
        match stmt {
            Stmt::Let(ls) => {
                assert_eq!(ls.token.literal, "let");
                assert_eq!(ls.name.val, idents[i]);
                assert_eq!(ls.name.token.literal, idents[i]);
            },
            _ => panic!("We parsed other than let statement."),
        }
    }
}
