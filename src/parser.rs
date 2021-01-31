use crate::ast::{
    Program,
    Stmt,
    Expr,
    Let,
    Return,
    ExprStmt,
    Ident,
    Int,
};
use crate::lexer;
use crate::token;

pub struct Parser<'a> {
    l: &'a mut lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
    errors: Vec<String>,
}

#[derive(PartialOrd, PartialEq)]
enum Precedence {
    Lowest,
    Equals, // ==
    Lt,     // <, >, <=, >=
    Add,    // + or -
    Mul,    // * or /
    Prefix, // -x or !x
    Call,   // func(x)
}

pub fn new(l: &mut lexer::Lexer) -> Parser {
    let first_token = l.next_token();
    let second_token = l.next_token();
    let p = Parser{
        l: l,
        cur_token: first_token,
        peek_token: second_token,
        errors: vec![],
    };
    p
}

impl Parser<'_> {
    pub fn parse_program(&mut self) -> Program {
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
            token::Type::Return => {
                return Some(Stmt::Return(self.parse_return_stmt()));
            },
            _ => {
                return Some(Stmt::ExprStmt(self.parse_expr_stmt()));
            },
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

    fn parse_return_stmt(&mut self) -> Return {
        let t = self.cur_token.clone();
        self.next_token();

        while !self.cur_token_is(token::Type::Semicolon) {
            self.next_token();
        }
        let ret = Return {
            token: t,
        };
        return ret;
    }

    fn parse_expr_stmt(&mut self) -> ExprStmt {
        let t = self.cur_token.clone();
        let expr = self.parse_expr(Precedence::Lowest);

        if self.peek_token_is(token::Type::Semicolon) {
            self.next_token();
        }
        return ExprStmt{ token: t, expr: expr };
    }

    fn parse_expr(&mut self, prec: Precedence) -> Expr {
        return self.prefix_parse(self.cur_token.clone().t);
    }

    fn parse_ident(&mut self) -> Ident {
        let t = self.cur_token.clone();
        return Ident{ token: t.clone(), val: t.literal };
    }

    fn parse_int(&mut self) -> Int {
        let t = self.cur_token.clone();
        let n: isize = t.clone().literal.parse().unwrap();
        return Int{ token: t, val: n };
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
        if self.peek_token_is(t.clone()) {
            self.next_token();
            return true;
        }
        self.peek_error(t);
        return false;
    }

    fn peek_error(&mut self, t: token::Type) {
        let msg = format!(
            "expected peek token: {:?}, but got: {:?}", t, self.peek_token.t
        );
        self.errors.push(msg);
    }

    fn prefix_parse(&mut self, t: token::Type) -> Expr {
        if t == token::Type::Ident {
            return Expr::Ident(self.parse_ident());
        } else {
            return Expr::Int(self.parse_int());
        }
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

    assert_eq!(p.errors.len(), 0);
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

#[test]
fn return_stmts() {
    let input = "return 5;
        return 10;
        return 993322;";

    let mut l = lexer::new(input);
    let mut p = new(&mut l);
    let program = p.parse_program();

    assert_eq!(p.errors.len(), 0);
    assert_eq!(program.stmts.len(), 3);

    let vals = [
        "5", "10", "993322"
    ];

    for (i, stmt) in program.stmts.iter().enumerate() {
        match stmt {
            Stmt::Return(rs) => {
                assert_eq!(rs.token.literal, "return");
            },
            _ => panic!("We parsed other than return statement."),
        }
    }
}

#[test]
fn ident_expr() {
    let input = "foobar;";

    let mut l = lexer::new(input);
    let mut p = new(&mut l);
    let program = p.parse_program();

    assert_eq!(p.errors.len(), 0);
    assert_eq!(program.stmts.len(), 1);

    let stmt = &program.stmts[0];
    let es = match stmt {
        Stmt::ExprStmt(es) => es,
        _ => panic!("We parsed other than expression statement."),
    };

    assert_eq!(es.token.literal, "foobar");
    match &es.expr {
        Expr::Ident(id) => {
            assert_eq!(id.val, "foobar");
        }
        _ => panic!("We parsed other than identifer."),
    }
}

#[test]
fn int_expr() {
    let input = "5;";

    let mut l = lexer::new(input);
    let mut p = new(&mut l);
    let program = p.parse_program();

    assert_eq!(p.errors.len(), 0);
    assert_eq!(program.stmts.len(), 1);

    let stmt = &program.stmts[0];
    let es = match stmt {
        Stmt::ExprStmt(es) => es,
        _ => panic!("We parsed other than expression statement."),
    };

    assert_eq!(es.token.literal, "5");
    match &es.expr {
        Expr::Int(i) => {
            assert_eq!(i.val, 5);
        }
        _ => panic!("We parsed other than integer."),
    }
}
