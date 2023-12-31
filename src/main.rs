use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process;

mod scanner;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();

    let mut lox_interpreter = Lox::new();

    if args.len() > 1 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() == 1 {
        lox_interpreter.run_file(args.get(0).unwrap());
    } else {
        lox_interpreter.run_prompt();
    }
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Lox {
        Lox { had_error: false }
    }

    fn run_file(&self, path: &str) {
        match fs::read_to_string(Path::new(path)) {
            Ok(program) => {
                self.run(&program);
                if self.had_error {
                    process::exit(64);
                }
            }
            Err(e) => println!("{e}"),
        }
    }

    fn run_prompt(&mut self) {
        let mut input = String::new();
        loop {
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    break;
                }
                Ok(_n) => {
                    self.run(&input);
                    self.had_error = false;
                    input.clear();
                }
                Err(e) => {
                    println!("Error: {e}");
                    break;
                }
            }
        }
    }

    fn run(&self, program: &str) {}

    fn error(&mut self, line: u64, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: u64, place: &str, message: &str) {
        eprintln!("[line {line}] Error{place}: {message}");
        self.had_error = true;
    }
}
