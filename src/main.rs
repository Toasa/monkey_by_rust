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

        // print tokens sequense.
        loop {
            let tok = l.next_token();
            if tok.t == token::Type::Eof {
                break;
            }
            println!("{} ", tok.literal);
        }

        let mut p = parser::new(&mut l);
        let _program = p.parse_program();
    }
}
