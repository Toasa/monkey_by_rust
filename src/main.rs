use std::io::Write;
mod token;
mod lexer;
mod parser;
mod ast;
mod eval;
mod object;
mod env;

fn main() {
    let mut env = env::new();

    let prompt = ">> ";
    loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();

        let mut l = lexer::new(input.trim());
        let mut p = parser::new(&mut l);

        let program = p.parse_program();
        let root_node = ast::Node::Program(program);

        let evaled = eval::eval(root_node, &mut env);
        println!("{}", evaled);
    }
}
