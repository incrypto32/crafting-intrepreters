use std::fs;
use std::io::{self, Write};
use std::process::exit;

use scanner::Scanner;

use crate::ast_printer::AstPrinter;
use crate::expr_eval::Interpreter;
use crate::parser::Parser;
mod ast_printer;
mod expr_eval;
mod parser;
mod scanner;
mod token;

fn main() {
    let mut lox = Lox::new();
    lox.run();
    if lox.had_error {
        exit(65); // EX_DATAERR for data format error
    }
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Lox {
        Lox { had_error: false }
    }

    fn run(&mut self) {
        let args: Vec<String> = std::env::args().collect();

        match args.len() {
            1 => self.run_prompt(),
            2 => {
                self.run_file(args[1].clone());
            }
            _ => {
                eprintln!("Usage: rlox [script]"); // Errors should go to stderr
                exit(64); // EX_USAGE for command line usage error
            }
        }
    }

    fn run_file(&mut self, path: String) {
        match fs::read_to_string(&path) {
            Ok(contents) => {
                let mut scanner = Scanner::new(contents);
                let tokens = scanner.scan_tokens();

                if scanner.has_error() {
                    self.had_error = true;
                } else {
                    // For now, we'll print tokens if no error, similar to REPL behavior.
                    // This can be removed or made conditional later.
                    let mut parser = Parser::new(tokens);
                    let expr = parser.parse().unwrap();
                    let mut printer = AstPrinter::new();
                    println!("{}", expr.accept(&mut printer));
                }
            }
            Err(e) => {
                self.report(0, Some(&format!("while reading '{}'", path)), &e.to_string());
                // self.had_error is set by self.report
            }
        }
    }

    fn run_prompt(&mut self) {
        let stdin = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush().unwrap_or_else(|e| {
                self.report(0, Some("while flushing stdout"), &e.to_string());
                // If we can't flush stdout, the REPL is likely unusable.
                // Set had_error and break.
                self.had_error = true;
            });

            if self.had_error {
                // Check if flush failed
                break;
            }

            let mut input = String::new();
            match stdin.read_line(&mut input) {
                Ok(0) => break, // EOF (Ctrl-D)
                Ok(_) => {
                    let mut scanner = Scanner::new(input);
                    let tokens = scanner.scan_tokens();

                    // Scanner prints its own errors to stderr.
                    // We do NOT set self.had_error for REPL line errors,
                    // allowing the user to continue.
                    if !scanner.has_error() {
                        let mut parser = Parser::new(tokens);
                        let expr = parser.parse().unwrap_or_else(|e| {
                            self.report(e.token.line, None, &e.to_string());
                            std::process::exit(65);
                        });
                        let mut printer = AstPrinter::new();
                        println!("{}", expr.accept(&mut printer));

                        let mut eval = Interpreter::new();
                        match eval.interpret(&expr) {
                            Ok(value) => println!("{}", value),
                            Err(e) => {
                                self.report(e.token.line, None, &e.message);
                            }
                        }
                    }
                }
                Err(e) => {
                    self.report(0, Some("while reading from prompt"), &e.to_string());
                    // self.had_error is set by self.report. Break REPL on stdin error.
                    break;
                }
            }
        }
    }

    #[allow(dead_code)] // Keep error method for potential future direct use
    fn error(&mut self, line: usize, message: &str) {
        self.report(line, None, message);
    }

    fn report(&mut self, line: usize, location: Option<&str>, message: &str) {
        eprintln!(
            "[line {}] Error{}: {}",
            line,
            location
                .map(|s| format!(" {}", s))
                .unwrap_or("".to_string()),
            message
        );
        self.had_error = true;
    }
}
