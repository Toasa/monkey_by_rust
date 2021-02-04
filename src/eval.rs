use crate::object::{
    Object,
    Int,
    Bool,
};
use crate::ast;

pub fn eval(node: ast::Node) -> Object {
    return match node {
        ast::Node::Program(p) => eval_stmts(p.stmts),
        ast::Node::Stmt(s) => eval_stmt(&s),
        ast::Node::Expr(e) => eval_expr(&e),
    };
}

pub fn eval_stmts(stmts: Vec<ast::Stmt>) -> Object {
    let mut result = eval_stmt(&stmts[0]);
    for stmt in stmts.iter() {
        result = eval_stmt(stmt);
    }
    result
}

pub fn eval_stmt(stmt: &ast::Stmt) -> Object {
    return match stmt {
        ast::Stmt::ExprStmt(es) => eval_expr(&es.expr),
        _ => panic!("Unsupported statement"),
    };
}

pub fn eval_expr(expr: &ast::Expr) -> Object {
    return match expr {
        ast::Expr::Int(n) => Object::Int(Int { val: n.val }),
        ast::Expr::Bool(b) => Object::Bool(Bool { val: b.val }),
        _ => panic!("Unsupported expression"),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer;
    use crate::parser;

    #[test]
    fn eval_int() {
        struct Test<'a> {
            input: &'a str,
            expected: isize,
        }

        let tests: Vec<Test> = vec! [
            Test { input: "5", expected: 5 },
            Test { input: "10", expected: 10 },
        ];

        for test in tests.iter() {
            let evaled = test_eval(test.input);
            test_int(evaled, test.expected);
        }
    }

    #[test]
    fn eval_bool() {
        struct Test<'a> {
            input: &'a str,
            expected: bool,
        }

        let tests: Vec<Test> = vec! [
            Test { input: "true", expected: true },
            Test { input: "false", expected: false },
        ];

        for test in tests.iter() {
            let evaled = test_eval(test.input);
            test_bool(evaled, test.expected);
        }
    }

    fn test_eval(input: &str) -> Object {
        let mut l = lexer::new(&input);
        let mut p = parser::new(&mut l);
        let program = p.parse_program();
        return eval(ast::Node::Program(program));
    }

    fn test_int(obj: Object, expected: isize) {
        match obj {
            Object::Int(i) => assert_eq!(i.val, expected),
            _ => panic!("We evaled other than integer."),
        };
    }

    fn test_bool(obj: Object, expected: bool) {
        match obj {
            Object::Bool(b) => assert_eq!(b.val, expected),
            _ => panic!("We evaled other than boolean."),
        };
    }
}
