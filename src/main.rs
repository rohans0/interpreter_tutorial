use std::env;
use std::io::Write;
use std::process;
use std::io;
use std::fs;

mod scanner;
mod token;
mod parser;

use crate::scanner::Scanner;
use crate::token::Token;


fn run(source: &str) -> io::Result<()> {
    let mut scanner = Scanner::new(source);

    let tokens: Vec<Token> = scanner.scan_tokens();

    for t in tokens {
        println!("{}",t);
    }
    Ok(())
}

fn error(line: u32, message: &str) {
    report(line,"",message);
}

fn report(line: u32, place: &str, message: &str) {
    println!("[line {}] Error {}: {}", line, place, message);
}

fn run_file(path: &str) -> io::Result<()> {
    if let Err(_) = run(&fs::read_to_string(path)?) {
        process::exit(65);
    }
    Ok(())
}

fn run_prompt() {
    let reader = io::stdin();
    let mut line = String::new();

    loop {
        print!("> ");
        let _ = io::stdout().flush();
        line.clear();
        if reader.read_line(&mut line).unwrap_or(0) == 0 {break}
        let _ = run(line.trim());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Usage: ...");
        process::exit(64);
    }
    else if args.len() == 1 {
        if let Err(e) = run_file(&args[0]) {
            eprintln!("File error: {}", e);
            process::exit(66);
        }
    }
    else {
        run_prompt();
    }
}
