use crate::scanner::lexer::scan_tokens;
use std::env;
use std::fs;
use std::io;
use std::io::Write;

mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(args[1].to_string()),
        _ => {
            println!("Usage: jlox [script]");
            std::process::exit(64)
        }
    }
}

fn run_file(path: String) {
    println!("File Mode. The path is {}", path);

    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error reading file: {}", e);
            std::process::exit(1)
        }
    };

    run(contents)
}

fn run_prompt() {
    println!("Prompt Mode.");

    let mut line;
    loop {
        line = String::from("");

        print!("> ");
        let res = io::stdout().flush();
        if let Err(e) = res {
            println!("Error flushing stdout: {}", e);
            std::process::exit(1);
        }

        if let Err(e) = io::stdin().read_line(&mut line) {
            println!("Failed to read line: {}", e);
            std::process::exit(1);
        }

        if line.is_empty() {
            println!("Empty line detected. Exiting...");
            std::process::exit(0)
        }

        run(line)
    }
}

fn run(source: String) {
    println!("{}", source);
    let tokens = match scan_tokens(source) {
        Ok(tokens) => tokens,
        Err(err) => {
            println!("Error scanning: {}", err);
            return;
        }
    };
    for tok in tokens.iter() {
        println!("token: {:?}", tok);
    }
}
