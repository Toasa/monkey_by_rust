use std::io::Write;
mod token;
mod lexer;

fn main() {
    let prompt = ">> ";
    loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();

        let mut l = lexer::new(input.trim());
        loop {
            let tok = l.next_token();
            if tok.t == token::Type::Eof {
                break;
            }
            println!("{} ", tok.literal);
        }
    }
}
