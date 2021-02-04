use crate::object::{
    Object,
    Int,
    Bool,
    Null,
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
        ast::Expr::Prefix(p) => {
            let rhs = eval_expr(&*p.rhs);
            eval_prefix_expr(&p.op, &rhs)
        },
        ast::Expr::Infix(i) => {
            let lhs = eval_expr(&*i.lhs);
            let rhs = eval_expr(&*i.rhs);
            eval_infix_expr(&i.op, &lhs, &rhs)
        },
        _ => panic!("Unsupported expression"),
    }
}

pub fn eval_prefix_expr(op: &str, rhs: &Object) -> Object {
    return match op {
        "!" => eval_prefix_bang(rhs),
        "-" => eval_prefix_minus(rhs),
        _ => Object::Null(Null {}),
    };
}

pub fn eval_infix_expr(op: &str, lhs: &Object, rhs: &Object) -> Object {
    let lval = match lhs {
        Object::Int(n) => n.val,
        Object::Bool(b) => b.val as isize,
        _ => return Object::Null(Null {}),
    };
    let rval = match rhs {
        Object::Int(n) => n.val,
        Object::Bool(b) => b.val as isize,
        _ => return Object::Null(Null {}),
    };

    return match op {
        "+" => Object::Int(Int { val: lval + rval }),
        "-" => Object::Int(Int { val: lval - rval }),
        "*" => Object::Int(Int { val: lval * rval }),
        "/" => Object::Int(Int { val: lval / rval }),
        "<" => Object::Bool(Bool { val: lval < rval }),
        ">" => Object::Bool(Bool { val: lval > rval }),
        "==" => Object::Bool(Bool { val: lval == rval }),
        "!=" => Object::Bool(Bool { val: lval != rval }),
        _ => Object::Null(Null {}),
    }
}

pub fn eval_prefix_bang(rhs: &Object) -> Object {
    return match rhs {
        Object::Bool(b) => Object::Bool(Bool { val: !b.val }),
        Object::Null(_) => Object::Bool(Bool { val: true }),
        _ => Object::Bool(Bool { val: false }),
    };
}

pub fn eval_prefix_minus(rhs: &Object) -> Object {
    return match rhs {
        Object::Int(i) => Object::Int(Int { val: -i.val }),
        _ => Object::Null(Null {}),
    };
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
            Test { input: "-5", expected: -5 },
            Test { input: "10", expected: 10 },
            Test { input: "5 + 5 + 5 - 5", expected: 10 },
            Test { input: "2 * 2 * 2 * 2", expected: 16 },
            Test { input: "2 + 3 * 4", expected: 14 },
            Test { input: "2 * 3 + 4", expected: 10 },
            Test { input: "-10 + 100 + -10", expected: 80 },
            Test { input: "50 / 2 * 2 + 10", expected: 60 },
            Test { input: "2 * (5 + 10)", expected: 30 },
            Test { input: "3 * 3 * 3 + 10", expected: 37 },
            Test { input: "3 * (3 * 3) + 10", expected: 37 },
            Test { input: "(5 + 10 * 2 + 15 / 3) * 2 + -10", expected: 50 },
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
            Test { input: "1 < 2", expected: true },
            Test { input: "1 > 2", expected: false },
            Test { input: "1 < 1", expected: false },
            Test { input: "1 < 1", expected: false },
            Test { input: "1 == 1", expected: true },
            Test { input: "1 != 1", expected: false },
            Test { input: "1 == 2", expected: false },
            Test { input: "1 != 2", expected: true },
            Test { input: "true == true", expected: true },
            Test { input: "false == false", expected: true },
            Test { input: "true == false", expected: false },
            Test { input: "true != false", expected: true },
            Test { input: "false != true", expected: true },
            Test { input: "(1 < 2) == true", expected: true },
            Test { input: "(1 < 2) == false", expected: false },
            Test { input: "(1 > 2) == true", expected: false },
            Test { input: "(1 > 2) == false", expected: true },
        ];

        for test in tests.iter() {
            let evaled = test_eval(test.input);
            test_bool(evaled, test.expected);
        }
    }

    #[test]
    fn eval_bang() {
        struct Test<'a> {
            input: &'a str,
            expected: bool,
        }

        let tests: Vec<Test> = vec! [
            Test { input: "!true", expected: false },
            Test { input: "!false", expected: true },
            Test { input: "!!true", expected: true },
            Test { input: "!!false", expected: false },
            Test { input: "!5", expected: false },
            Test { input: "!!5", expected: true },
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
