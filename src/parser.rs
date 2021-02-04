use crate::ast::{
    Program,
    Stmt,
    Expr,
    Let,
    Return,
    ExprStmt,
    Block,
    Ident,
    Int,
    Prefix,
    Infix,
    Boolean,
    If,
    Func,
    Call,
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

        let _ = self.expect_peek(token::Type::Assign);
        self.next_token();
        let val = self.parse_expr(Precedence::Lowest);

        if self.peek_token_is(token::Type::Semicolon) {
            self.next_token();
        }

        Let { token: t, name: ident, val: val }
    }

    fn parse_return_stmt(&mut self) -> Return {
        let t = self.cur_token.clone();
        self.next_token();

        let val = self.parse_expr(Precedence::Lowest);

        if self.peek_token_is(token::Type::Semicolon) {
            self.next_token();
        }
        Return { token: t, val: val }
    }

    fn parse_expr_stmt(&mut self) -> ExprStmt {
        let t = self.cur_token.clone();
        let expr = self.parse_expr(Precedence::Lowest);

        if self.peek_token_is(token::Type::Semicolon) {
            self.next_token();
        }
        ExprStmt { token: t, expr: expr }
    }

    fn parse_block(&mut self) -> Block {
        let mut stmts: Vec<Stmt> = vec![];

        let t = self.cur_token.clone();
        self.next_token();

        while !self.cur_token_is(token::Type::Rbrace) &&
              !self.cur_token_is(token::Type::Eof) {
            let stmt = self.parse_stmt();
            stmts.push(stmt);
            self.next_token();
        }
        Block { token: t, stmts: stmts }
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
                token::Type::Lparen => {
                    self.next_token();
                    lhs = Expr::Call(self.parse_call(lhs));
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

    fn parse_if(&mut self) -> If {
        let t = self.cur_token.clone();
        self.expect_peek(token::Type::Lparen);
        self.next_token();
        let cond = self.parse_expr(Precedence::Lowest);
        let _ = self.expect_peek(token::Type::Rparen);
        let _ = self.expect_peek(token::Type::Lbrace);
        let cons = self.parse_block();

        let has_alt = self.peek_token_is(token::Type::Else);

        let alt: Option<Block> = if has_alt {
            self.next_token();
            let _ = self.expect_peek(token::Type::Lbrace);
            Some(self.parse_block())
        } else { None };

        If { token: t, cond: Box::new(cond), cons: cons, alt: alt }
    }

    fn parse_func(&mut self) -> Func {
        let t = self.cur_token.clone();
        let _ = self.expect_peek(token::Type::Lparen);
        let params = self.parse_func_params();
        let _ = self.expect_peek(token::Type::Lbrace);
        let body = self.parse_block();
        Func { token: t, params: params, body: body }
    }

    fn parse_func_params(&mut self) -> Vec<Ident> {
        let mut params: Vec<Ident> = vec![];
        self.next_token();
        while !self.cur_token_is(token::Type::Rparen) {
            let param = self.parse_ident();
            params.push(param);

            // skip identifier
            self.next_token();

            if self.cur_token_is(token::Type::Comma) {
                self.next_token();
            }
        }
        params
    }

    fn parse_boolean(&mut self) -> Boolean {
        let t = self.cur_token.clone();
        let b: bool = if self.cur_token.clone().literal == "true"
            { true } else { false };
        Boolean { token:t , val: b}
    }

    fn parse_grouped_expr(&mut self) -> Expr {
        self.next_token();
        let e = self.parse_expr(Precedence::Lowest);
        let _ = self.expect_peek(token::Type::Rparen);
        e
    }

    fn parse_call(&mut self, func: Expr) -> Call {
        let t = self.cur_token.clone();
        let args = self.parse_call_args();
        Call { token: t, func: Box::new(func), args: args }
    }

    fn parse_call_args(&mut self) -> Vec<Expr> {
        let mut args: Vec<Expr> = vec![];
        self.next_token();
        while !self.cur_token_is(token::Type::Rparen) {
            let arg = self.parse_expr(Precedence::Lowest);
            args.push(arg);

            // skip argument
            self.next_token();

            if self.cur_token_is(token::Type::Comma) {
                self.next_token();
            }
        }
        args
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
        return match t {
            token::Type::Ident => {
                Expr::Ident(self.parse_ident())
            },
            token::Type::Lparen => {
                self.parse_grouped_expr()
            },
            token::Type::If => {
                Expr::If(self.parse_if())
            },
            token::Type::Function => {
                Expr::Func(self.parse_func())
            },
            token::Type::Minus | token::Type::Bang => {
                Expr::Prefix(self.parse_prefix())
            },
            token::Type::True | token::Type::False => {
                Expr::Boolean(self.parse_boolean())
            },
            _ => Expr::Int(self.parse_int()),
        };
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
        token::Type::Lparen 
            => Precedence::Call,
        _ => Precedence::Lowest,
    };
}

#[cfg(test)]
mod test {
    use super::*;

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

        let exprs = [ 5, 10, 838383 ];

        for (i, stmt) in program.stmts.iter().enumerate() {
            match stmt {
                Stmt::Let(ls) => {
                    assert_eq!(ls.token.literal, "let");
                    assert_eq!(ls.name.val, idents[i]);
                    assert_eq!(ls.name.token.literal, idents[i]);
                    test_int(&ls.val, exprs[i]);
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
        let expects = [ 5, 10, 993322 ];
        let program = test_parse_program(input);

        assert_eq!(program.stmts.len(), 3);

        for (i, stmt) in program.stmts.iter().enumerate() {
            match stmt {
                Stmt::Return(rs) => {
                    assert_eq!(rs.token.literal, "return");
                    test_int(&rs.val, expects[i]);
                },
                _ => panic!("We parsed other than return statement."),
            }
        }
    }

    #[test]
    fn ident_expr() {
        let input = "foobar;";
        let program = test_parse_program(input);

        assert_eq!(program.stmts.len(), 1);

        let stmt = &program.stmts[0];
        let es = test_expr_stmt(stmt);

        test_ident(&es.expr, "foobar");
    }

    #[test]
    fn int_expr() {
        let input = "5;";
        let program = test_parse_program(input);

        assert_eq!(program.stmts.len(), 1);

        let stmt = &program.stmts[0];
        let es = test_expr_stmt(stmt);

        test_int(&es.expr, 5);
    }

    #[test]
    fn boolean_expr() {
        let inputs = vec![ "true;", "false;"];
        let expects = vec![ true, false ];

        for (i, input) in inputs.iter().enumerate() {
            let program = test_parse_program(input);

            assert_eq!(program.stmts.len(), 1);

            let stmt = &program.stmts[0];
            let es = test_expr_stmt(stmt);

            match &es.expr {
                Expr::Boolean(b) => assert_eq!(b.val, expects[i]),
                _ => panic!("We parsed other than boolean expression."),
            }
        }
    }

    #[test]
    fn if_expr() {
        let inputs= vec![
            "if (x < y) { x };",
            "if (x < y) { x } else { y };"
        ];

        let has_alt = vec![ false, true ];

        for (i, input) in inputs.iter().enumerate() {
            let program = test_parse_program(input);

            assert_eq!(program.stmts.len(), 1);

            let stmt = &program.stmts[0];
            let es = test_expr_stmt(stmt);

            let ifstmt = match &es.expr {
                Expr::If(i) => i,
                _ => panic!("We parsed other than integer."),
            };

            if has_alt[i] {
                assert!(ifstmt.alt.is_some());
            } else {
                assert!(ifstmt.alt.is_none());
            }
        }
    }

    #[test]
    fn fn_expr() {
        let input = "fn(x, y) { x + y; }";
        let program = test_parse_program(input);

        assert_eq!(program.stmts.len(), 1);

        let stmt = &program.stmts[0];
        let es = test_expr_stmt(stmt);

        let f = match &es.expr {
            Expr::Func(f) => f,
            _ => panic!("We parsed other than function expression."),
        };

        assert_eq!(f.params[0].val, "x");
        assert_eq!(f.params[1].val, "y");
    }

    #[test]
    fn call_expr() {
        let input = "add(1, 2 * 3, 4 + 5);";
        let program = test_parse_program(input);

        assert_eq!(program.stmts.len(), 1);

        let stmt = &program.stmts[0];
        let es = test_expr_stmt(stmt);

        let c = match &es.expr {
            Expr::Call(c) => c,
            _ => panic!("We parsed other than function call."),
        };

        test_ident(&*(c.func), "add");
        test_int(&c.args[0], 1);
    }

    #[test]
    fn prefix_exprs() {
        let inputs = vec![ "!5;", "-15;" ];
        let expect_prefixes = vec![ "!", "-" ];
        let expect_ints = vec![ 5, 15 ];

        for (i, input) in inputs.iter().enumerate() {
            let program = test_parse_program(input);

            assert_eq!(program.stmts.len(), 1);

            let stmt = &program.stmts[0];
            let es = test_expr_stmt(stmt);

            let pre = match &es.expr {
                Expr::Prefix(pre) => pre,
                _ => panic!("We parsed other than prefix expression."),
            };

            assert_eq!(pre.op, expect_prefixes[i]);
            test_int(&*pre.rhs, expect_ints[i]);
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
            let program = test_parse_program(&test.input);
            assert_eq!(program.stmts.len(), 1);

            let stmt = &program.stmts[0];
            let es = test_expr_stmt(stmt);

            let i = match &es.expr {
                Expr::Infix(i) => i,
                _ => panic!("We parsed other than infix expression."),
            };

            test_int(&*(i.lhs), test.lhs);
            test_int(&*(i.rhs), test.rhs);
            assert_eq!(i.op, test.op);
        }
    }

    #[test]
    fn operator_precedence() {
        struct Test<'a> {
            input: &'a str,
            expected: &'a str,
        }

        let tests: Vec<Test> = vec![
            Test {
                input: "true",
                expected: "true"
            },
            Test {
                input: "false",
                expected: "false"
            },
            Test {
                input: "3 > 5 == false",
                expected: "((3 > 5) == false)"
            },
            Test {
                input: "3 < 5 == true",
                expected: "((3 < 5) == true)"
            },
            Test {
                input: "1 + (2 + 3) + 4",
                expected: "((1 + (2 + 3)) + 4)"
            },
            Test {
                input: "(2 + 5) * 4",
                expected: "((2 + 5) * 4)"
            },
            Test {
                input: "2 / (5 + 4)",
                expected: "(2 / (5 + 4))"
            },
            Test {
                input: "-(5 + 4)",
                expected: "(-(5 + 4))"
            },
            Test {
                input: "!(true + true)",
                expected: "(!(true + true))"
            },
            Test {
                input: "a + add(b * c) + d",
                expected: "((a + add((b * c))) + d)"
            },
            Test {
                input: "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                expected: "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"
            },
            Test {
                input: "add(a + b + c * d / f + g)",
                expected: "add((((a + b) + ((c * d) / f)) + g))"
            },
        ];

        for test in tests.iter() {
            let program = test_parse_program(&test.input);
            let stmt = &program.stmts[0];
            let stmt_str = format!("{}", stmt);
            assert_eq!(stmt_str, test.expected);
        }
    }

    fn test_parse_program(input: &str) -> Program {
        let mut l = lexer::new(input);
        let mut p = new(&mut l);
        let program = p.parse_program();
        assert_eq!(p.errors.len(), 0);
        program
    }

    fn test_expr_stmt(stmt: &Stmt) -> &ExprStmt {
        return match stmt {
            Stmt::ExprStmt(es) => es,
            _ => panic!("We parsed other than expression statement."),
        };
    }

    fn test_int(n: &Expr, expected: isize) {
        match n {
            Expr::Int(int) => assert_eq!(int.val, expected),
            _ => panic!("We parsed other than integer."),
        };
    }

    fn test_ident(ident: &Expr, expected: &str) {
        match ident {
            Expr::Ident(id) => assert_eq!(id.val, expected),
            _ => panic!("We parsed other than identifer."),
        };
    }
}
