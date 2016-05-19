extern crate rlisp;
extern crate toml;
extern crate readline;

use rlisp::Lisp;
use rlisp::eval::{FuncResult, FuncError};
use rlisp::parse::{ParseError};

use std::io::{self, Read, Write};

fn main() {
    let mut lisp = Lisp::new();
    println!("rlisp {}", get_version());

    loop {
        let input = read(">>> ");
        let result = eval(input, &mut lisp);

        match result {
            Ok(val) => println!("=> {:?}", val),
            Err(err) => println!("Error: {:?}", err),
        }

        println!("");
    }
}

fn read(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
}

fn eval(mut input: String, lisp: &mut Lisp) -> FuncResult {
    let val = lisp.eval_raw(&input);

    match val {
        Ok(val) => Ok(val),
        Err(err) => match err {
            FuncError::ParsingErr(ParseError::UnclosedList) => {
                input.push_str(&read("... "));
                eval(input, lisp)
            },
            _ => Err(err),
        }
    }
}

fn get_version() -> String {
    use std::fs::File;

    let mut cargo = File::open("Cargo.toml").unwrap();

    let mut cargo_contents = String::new();
    cargo.read_to_string(&mut cargo_contents).unwrap();

    let toml: toml::Value = (&cargo_contents[..]).parse().unwrap();

    toml.lookup("package.version").unwrap()
        .as_str().unwrap()
        .to_string()
}
