use std::{
    env, fs,
    io::{self, Write},
    process::ExitCode,
};

use crate::{ast_printer::AstPrinter, intrepreter::Interpreter, parser::Parser, scanner::Scanner};
mod ast_printer;
mod intrepreter;
mod parser;
mod scanner;
mod token;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(code) => code,
    }
}

fn run() -> Result<(), ExitCode> {
    match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [] => repl(),
        [path] => run_file(path),
        _ => {
            eprintln!("Usage: rlox [script]");
            Err(ExitCode::from(64))
        }
    }
}

fn run_file(path: &str) -> Result<(), ExitCode> {
    let src = fs::read_to_string(path).map_err(|e| {
        eprintln!("Error reading {path}: {e}");
        ExitCode::from(65)
    })?;
    run_source(&src).map_err(|_| ExitCode::from(65))
}

fn repl() -> Result<(), ExitCode> {
    let mut line = String::new();
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        line.clear();
        if stdin.read_line(&mut line).unwrap() == 0 {
            break; // EOF
        }
        let _ = run_source(&line); // Ignore per-line errors, keep REPL alive
    }
    Ok(())
}

/// Scan → parse → interpret one chunk of Lox source.
fn run_source(src: &str) -> Result<(), ()> {
    let mut scanner = Scanner::new(src.to_owned());
    let tokens = scanner.scan_tokens();
    if scanner.has_error() {
        return Err(());
    }

    let mut parser = Parser::new(tokens);
    let stmts = parser.parse().map_err(|e| {
        eprintln!("{}", e);
        ()
    })?;

    let mut ast_printer = AstPrinter::new();
    for stmt in &stmts {
        println!("{}", stmt.accept(&mut ast_printer));
    }

    let mut interp = Interpreter::new();
    interp.interpret(&stmts).map_err(|e| {
        eprintln!("[line {}] Error: {}", e.token.line, e.message);
    })
}
