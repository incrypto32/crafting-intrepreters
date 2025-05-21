use std::io::Write;
use std::process::exit;

use scanner::Scanner;
mod scanner;

fn main() {
    let lox = Lox { had_error: false };

    lox.run();
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn run(&self) {
        let args: Vec<String> = std::env::args().collect();

        match args.len() {
            1 => Self::runPrompt(),
            2 => {
                println!("Yeay");
                Self::runFile(args[1].clone());
            }
            _ => {
                println!("Usage: rlox [script]");
                std::process::exit(64);
            }
        }
    }

    fn runFile(a: String) {
        println!("{}", a);
    }

    fn runPrompt() {
        let stdin = std::io::stdin();
        loop {
            print!("Enter something > ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let mut scanner = Scanner::new(input.clone());
            let tokens = scanner.scan_tokens();
            println!("You entered: {}", input.trim());
            println!("{:?}", tokens);
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, location, message);
        self.had_error = true;
    }
}
