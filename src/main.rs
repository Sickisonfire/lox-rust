use std::error::Error;
use std::io::Write;
use std::{env, fs, io};

mod scanner;
use scanner::Scanner;

mod token;

mod token_type;

mod error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.len() {
        0 => run_prompt().unwrap_or_else(|err| eprintln!("error: {}", err)),
        1 => run_file(&args[0]).unwrap_or_else(|err| eprintln!("error: {}", err)),
        _ => eprintln!("Usage: lox [script]"),
    };

    Ok(())
}

fn run_file(arg: &str) -> io::Result<()> {
    run(fs::read_to_string(arg)?.as_str());

    Ok(())
}
fn run_prompt() -> io::Result<()> {
    loop {
        let mut out = io::stdout().lock();
        out.write_all(b"> ")?;
        out.flush()?;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => match n {
                0 => break,
                _ => {
                    // dbg!("{}", input);
                    print!("{input}");
                    run(input.as_str());
                }
            },
            Err(err) => print!("{err}"),
        }
    }
    Ok(())
}

fn run(source: &str) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token.lexeme);
    }
}