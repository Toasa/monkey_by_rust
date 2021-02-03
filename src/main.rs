use std::io::Write;
mod token;
mod lexer;
mod parser;
mod ast;

fn main() {
    let prompt = ">> ";
    loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();

        let mut l = lexer::new(input.trim());
        let mut p = parser::new(&mut l);

        let program = p.parse_program();

        println!("{}", program);
    }
}
