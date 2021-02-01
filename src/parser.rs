use crate::ast::{
    Program,
    Stmt,
    Expr,
    Let,
    Return,
    ExprStmt,
    Ident,
    Int,
    Prefix,
    Infix,
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
    Parser {
        l: l,
        cur_token: first_token,
        peek_token: second_token,
        errors: vec![],
    }
}

impl Parser<'_> {
    pub fn parse_program(&mut self) -> Program {
        let stmts: Vec<Stmt> = vec![];
        let mut p = Program {
            stmts: stmts,
        };

        while self.cur_token.t != token::Type::Eof {
            let stmt = self.parse_stmt();
            p.stmts.push(stmt);
            self.next_token();
        }
        p
    }

    fn parse_stmt(&mut self) -> Stmt {
        return match self.cur_token.t {
            token::Type::Let => Stmt::Let(self.parse_let_stmt()),
            token::Type::Return => Stmt::Return(self.parse_return_stmt()),
            _ => Stmt::ExprStmt(self.parse_expr_stmt()),
        };
    }

    fn parse_let_stmt(&mut self) -> Let {
        let t = self.cur_token.clone();

        let _ = self.expect_peek(token::Type::Ident);

        let ident = Ident {
            token: self.cur_token.clone(),
            val: self.cur_token.clone().literal,
        };

        let stmt = Let {
            token: t,
            name: ident,
        };

        let _ = self.expect_peek(token::Type::Assign);

        while !self.cur_token_is(token::Type::Semicolon) {
            self.next_token();
        }
        stmt
    }

    fn parse_return_stmt(&mut self) -> Return {
        let t = self.cur_token.clone();
        self.next_token();

        while !self.cur_token_is(token::Type::Semicolon) {
            self.next_token();
        }
        Return { token: t }
    }

    fn parse_expr_stmt(&mut self) -> ExprStmt {
        let t = self.cur_token.clone();
        let expr = self.parse_expr(Precedence::Lowest);

        if self.peek_token_is(token::Type::Semicolon) {
            self.next_token();
        }
        ExprStmt { token: t, expr: expr }
    }

    fn parse_expr(&mut self, prec: Precedence) -> Expr {
        let mut lhs = self.prefix_parse(self.cur_token.clone().t);

        while !self.peek_token_is(token::Type::Semicolon) &&
            prec < self.peek_precedence() {

            // TODO:: extract a function
            match self.peek_token.clone().t {
                token::Type::Plus | token::Type::Minus |
                token::Type::Slash | token::Type::Asterisk |
                token::Type::Equ | token::Type::Neq |
                token::Type::Lt | token::Type::Gt
                => {
                    self.next_token();
                    lhs = Expr::Infix(self.parse_infix(lhs));
                },
                _ => return lhs,
            }
        }
        lhs
    }

    fn parse_ident(&mut self) -> Ident {
        let t = self.cur_token.clone();
        Ident { token: t.clone(), val: t.literal }
    }

    fn parse_int(&mut self) -> Int {
        let t = self.cur_token.clone();
        let n: isize = t.clone().literal.parse().unwrap();
        Int { token: t, val: n }
    }

    fn parse_prefix(&mut self) -> Prefix {
        let t = self.cur_token.clone();
        let op = self.cur_token.clone().literal;
        self.next_token();
        let rhs = self.parse_expr(Precedence::Prefix);
        Prefix { token: t, op: op, rhs: Box::new(rhs) }
    }

    fn parse_infix(&mut self, lhs: Expr) -> Infix {
        let t = self.cur_token.clone();
        let op = self.cur_token.clone().literal;
        let prec = self.cur_precedence();
        self.next_token();
        let rhs = self.parse_expr(prec);
        Infix {
            token: t,
            lhs: Box::new(lhs),
            op: op,
            rhs: Box::new(rhs),
        }
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

    fn cur_precedence(&self) -> Precedence {
        return to_precedence(self.cur_token.clone().t);
    }

    fn peek_precedence(&self) -> Precedence {
        return to_precedence(self.peek_token.clone().t);
    }

    fn expect_peek(&mut self, t: token::Type) -> bool {
        if self.peek_token_is(t.clone()) {
            self.next_token();
            return true;
        }
        self.peek_error(t);
        false
    }

    fn peek_error(&mut self, t: token::Type) {
        let msg = format!(
            "expected peek token: {:?}, but got: {:?}", t, self.peek_token.t
        );
        self.errors.push(msg);
    }

    fn prefix_parse(&mut self, t: token::Type) -> Expr {
        match t {
            token::Type::Ident => {
                return Expr::Ident(self.parse_ident());
            },
            token::Type::Minus | token::Type::Bang => {
                return Expr::Prefix(self.parse_prefix());
            },
            _ => return Expr::Int(self.parse_int()),
        }
    }
}

fn to_precedence(t: token::Type) -> Precedence {
    return match t {
        token::Type::Equ | token::Type::Neq
            => Precedence::Equals,
        token::Type::Lt | token::Type::Gt
            => Precedence::Lt,
        token::Type::Plus | token::Type::Minus
            => Precedence::Add,
        token::Type::Slash | token::Type::Asterisk
            => Precedence::Mul,
        _ => Precedence::Lowest,
    };
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

#[test]
fn prefix_exprs() {
    let inputs = vec![ "!5;", "-15;" ];
    let expect_prefixes = vec![ "!", "-" ];
    let expect_ints = vec![ 5, 15 ];

    for (i, input) in inputs.iter().enumerate() {
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

        let pre = match &es.expr {
            Expr::Prefix(pre) => pre,
            _ => panic!("We parsed other than prefix expression."),
        };

        assert_eq!(pre.op, expect_prefixes[i]);

        match &*pre.rhs {
            Expr::Int(num) => assert_eq!(num.val, expect_ints[i]),
            _ => panic!("We parsed other than integer."),
        }
    }
}

#[test]
fn infix_exprs() {
    struct Test<'a> {
        input: &'a str,
        lhs: isize,
        op: &'a str,
        rhs: isize,
    }

    let tests: Vec<Test> = vec![
        Test { input: "5+5;", lhs: 5, op: "+", rhs: 5 },
        Test { input: "5-5;", lhs: 5, op: "-", rhs: 5 },
        Test { input: "5*5;", lhs: 5, op: "*", rhs: 5 },
        Test { input: "5/5;", lhs: 5, op: "/", rhs: 5 },
        Test { input: "5>5;", lhs: 5, op: ">", rhs: 5 },
        Test { input: "5<5;", lhs: 5, op: "<", rhs: 5 },
        Test { input: "5==5;", lhs: 5, op: "==", rhs: 5 },
        Test { input: "5!=5;", lhs: 5, op: "!=", rhs: 5 },
    ];

    for test in tests.iter() {
        let mut l = lexer::new(&test.input);
        let mut p = new(&mut l);
        let program = p.parse_program();

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.stmts.len(), 1);

        let stmt = &program.stmts[0];
        let es = match stmt {
            Stmt::ExprStmt(es) => es,
            _ => panic!("We parsed other than expression statement."),
        };

        let i = match &es.expr {
            Expr::Infix(i) => i,
            _ => panic!("We parsed other than infix expression."),
        };

        let lhs = match &*(i.lhs) {
            Expr::Int(int) => int.val,
            _ => panic!("We parsed other than integer."),
        };

        let rhs = match &*(i.rhs) {
            Expr::Int(int) => int.val,
            _ => panic!("We parsed other than integer."),
        };

        assert_eq!(lhs, test.lhs);
        assert_eq!(i.op, test.op);
        assert_eq!(rhs, test.rhs);
    }
}
