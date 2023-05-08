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
    let tokens = Vec::new();
    let mut scnr = scanner::scanner::Scanner { source, tokens };
    scnr.scan_tokens();
    for tok in scnr.tokens.iter() {
        println!("token: {:?}", tok);
    }
}

// fn error(line: i32, message: String) {
//     report(line, String::from(""), message)
// }
// fn report(line: i32, whre: String, message: String) {
//     println!("[line {}], Error: {}: {}", line, whre, message)
// }
