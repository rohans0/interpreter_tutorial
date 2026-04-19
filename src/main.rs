use std::env;
use std::process;
use std::io;
use std::fs;

#[derive(Debug)]
enum Token {
}

struct Scanner<'a> {
    source: &'a str
}

impl<'a> Scanner<'a> {
    fn scan_tokens(&mut self) -> Vec<Token> {
        Vec::new()
    }
}

fn run(source: &str) {
    let mut scanner = Scanner {
        source: source
    };
    let tokens: Vec<Token> = scanner.scan_tokens();
    
    for t in tokens {
        println!("{:?}",t);
    }
}

fn run_file(path: &str) -> io::Result<()> {
    run(&fs::read_to_string(path)?);
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let reader = io::stdin();
    let mut line = String::new();

    loop {
        line.clear();
        print!("> ");
        if reader.read_line(&mut line)? == 0 {break;}
        run(line.trim());
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Usage: ...");
        process::exit(64);
    }
    else if args.len() == 1 {
        if let Err(_) = run_file(&args[0]) {
            println!("error");
        }
    }
    else {
        if let Err(_) = run_prompt() {
            println!("error");
        }
    }
}
