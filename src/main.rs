use std::io::Write;
use std::{env, fs, io};

mod scanner;
use scanner::Scanner;

mod token;

mod token_type;

mod error;
use error::*;

fn main() -> LoxResult<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut interpreter = Lox::new();

    match args.len() {
        0 => interpreter
            .run_prompt()
            .unwrap_or_else(|err| eprintln!("{}", err)),
        1 => interpreter
            .run_file(&args[0])
            .unwrap_or_else(|err| eprintln!("{}", err)),
        _ => eprintln!("Usage: lox [script]"),
    };

    Ok(())
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Lox {
        Lox { had_error: false }
    }
    fn run_file(&mut self, arg: &str) -> LoxResult<()> {
        self.run(fs::read_to_string(arg)?.as_str())?;
<<<<<<< HEAD
=======
        if self.had_error {
            std::process::exit(65);
        };
>>>>>>> db064f6 (feat: Lox struct and had_error)
        Ok(())
    }
    fn run_prompt(&mut self) -> LoxResult<()> {
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
                        self.run(input.as_str())?;
<<<<<<< HEAD
=======
                        self.had_error = false;
>>>>>>> db064f6 (feat: Lox struct and had_error)
                    }
                },
                Err(err) => print!("{err}"),
            }
        }
        Ok(())
    }

    fn run(&mut self, source: &str) -> LoxResult<()> {
        let mut scanner = Scanner::new(source);
        let (tokens, had_scan_error) = scanner.scan_tokens()?;
        if had_scan_error {
            self.had_error = true
        };

        for token in tokens {
            println!("{}", token.to_string());
        }
        Ok(())
    }
}
