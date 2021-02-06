use crate::object::{
    Object,
    Int,
    Bool,
    Null,
    Return,
};
use crate::ast;
use crate::env::Env;

pub fn eval(node: ast::Node, env: &mut Env) -> Object {
    return match node {
        ast::Node::Program(p) => eval_program(&p.stmts, env),
        ast::Node::Stmt(s) => eval_stmt(&s, env),
        ast::Node::Expr(e) => eval_expr(&e, env),
    };
}

pub fn eval_program(stmts: &Vec<ast::Stmt>, env: &mut Env) -> Object {
    let mut result = Object::Null(Null {});

    for stmt in stmts.iter() {
        result = eval_stmt(stmt, env);
        match &result {
            Object::Return(r) => return *(r.clone().val),
            _ => (),
        };
    }

    result
}

pub fn eval_block(stmts: &Vec<ast::Stmt>, env: &mut Env) -> Object {
    let mut result = Object::Null(Null {});

    for stmt in stmts.iter() {
        result = eval_stmt(stmt, env);
        match &result {
            Object::Return(r) => return Object::Return(r.clone()),
            _ => (),
        };
    }

    result
}

pub fn eval_stmt(stmt: &ast::Stmt, env: &mut Env) -> Object {
    return match stmt {
        ast::Stmt::ExprStmt(es) => eval_expr(&es.expr, env),
        ast::Stmt::Block(b) => eval_block(&b.stmts, env),
        ast::Stmt::Return(r) => {
            let ret = eval_expr(&r.val, env);
            Object::Return(Return { val: Box::new(ret) })
        },
        ast::Stmt::Let(l) => {
            let val = eval_expr(&l.val, env);
            env.set(l.name.val.clone(), val.clone());
            val
        },
    };
}

pub fn eval_expr(expr: &ast::Expr, env: &mut Env) -> Object {
    return match expr {
        ast::Expr::Int(n) => Object::Int(Int { val: n.val }),
        ast::Expr::Bool(b) => Object::Bool(Bool { val: b.val }),
        ast::Expr::Prefix(p) => eval_prefix_expr(&p, env),
        ast::Expr::Infix(i) => eval_infix_expr(&i, env),
        ast::Expr::If(i) => eval_if_expr(&i, env),
        ast::Expr::Ident(i) => {
            let val = env.get(i.val.clone());
            match val {
                Some(v) => v.clone(),
                None => Object::Null(Null {}),
            }
        },
        _ => panic!("Unsupported expression"),
    }
}

pub fn eval_prefix_expr(p: &ast::Prefix, env: &mut Env) -> Object {
    let rhs = eval_expr(&*p.rhs, env);
    return match p.op.as_str() {
        "!" => eval_prefix_bang(&rhs, env),
        "-" => eval_prefix_minus(&rhs, env),
        _ => Object::Null(Null {}),
    };
}

pub fn eval_infix_expr(i: &ast::Infix, env: &mut Env) -> Object {
    let lhs = eval_expr(&i.lhs, env);
    let rhs = eval_expr(&i.rhs, env);

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

    return match i.op.as_str() {
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

pub fn eval_prefix_bang(rhs: &Object, _env: &mut Env) -> Object {
    return match rhs {
        Object::Bool(b) => Object::Bool(Bool { val: !b.val }),
        Object::Null(_) => Object::Bool(Bool { val: true }),
        _ => Object::Bool(Bool { val: false }),
    };
}

pub fn eval_prefix_minus(rhs: &Object, _env: &mut Env) -> Object {
    return match rhs {
        Object::Int(i) => Object::Int(Int { val: -i.val }),
        _ => Object::Null(Null {}),
    };
}

pub fn eval_if_expr(i: &ast::If, env: &mut Env) -> Object {
    let cond = eval_expr(&i.cond, env);
    if is_truthy(&cond) {
        eval_stmt(&ast::Stmt::Block(i.cons.clone()), env)
    } else {
        match &i.alt {
            Some(alt) => eval_stmt(&ast::Stmt::Block(alt.clone()), env),
            None => Object::Null(Null {}),
        }
    }
}

fn is_truthy(obj: &Object) -> bool {
    return match obj {
        Object::Null(_) => false,
        Object::Bool(b) => b.val,
        _ => true,
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer;
    use crate::parser;
    use crate::env;

    #[test]
    fn eval_return() {
        struct Test<'a> {
            input: &'a str,
            expected: isize,
        }

        let tests: Vec<Test> = vec! [
            Test { input: "return 10;", expected: 10 },
            Test { input: "return 10; 11;", expected: 10 },
            Test { input: "return 2 * 5; 11;", expected: 10 },
            Test { input: "9; return 2 * 5; 11;", expected: 10 },
            Test {
                input: "
                if (10 > 1) {
                    if (10 > 1) {
                        return 10;
                    }
                    return 1;
                }",
                expected: 10,
            },
        ];

        for test in tests.iter() {
            let evaled = test_eval(test.input);
            test_int(evaled, test.expected);
        }
    }

    #[test]
    fn eval_let() {
        struct Test<'a> {
            input: &'a str,
            expected: isize,
        }

        let tests: Vec<Test> = vec! [
            Test {
                input: "let a = 5; a;",
                expected: 5
            },
            Test {
                input: "let a = 5 * 5; a;",
                expected: 25
            },
            Test {
                input: "let a = 5; let b = a; b;",
                expected: 5
            },
            Test {
                input: "let a = 5; let b = a; let c = a + b + 5; c;",
                expected: 15
            },
        ];

        for test in tests.iter() {
            let evaled = test_eval(test.input);
            test_int(evaled, test.expected);
        }
    }

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

    #[test]
    fn eval_if() {
        struct Test<'a> {
            input: &'a str,
            expected: isize,
        }

        let tests: Vec<Test> = vec! [
            Test { input: "if (true) { 10 }", expected: 10 },
            Test { input: "if (1) { 10 }", expected: 10 },
            Test { input: "if (1 < 2) { 10 }", expected: 10 },
            Test { input: "if (1 < 2) { 10 } else { 20 }", expected: 10 },
            Test { input: "if (1 > 2) { 10 } else { 20 }", expected: 20 },
        ];

        for test in tests.iter() {
            let evaled = test_eval(test.input);
            test_int(evaled, test.expected);
        }

        let inputs = vec! [
            "if (false) { 10 }",
            "if (1 > 2) { 10 }",
        ];
        for input in inputs.iter() {
            let evaled = test_eval(input);
            test_null(evaled);
        }
    }

    fn test_eval(input: &str) -> Object {
        let mut l = lexer::new(&input);
        let mut p = parser::new(&mut l);
        let program = p.parse_program();
        let mut env = env::new();
        return eval(ast::Node::Program(program), &mut env);
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

    fn test_null(obj: Object) {
        match obj {
            Object::Null(_) => return,
            _ => panic!("We evaled other than null."),
        };
    }
}
