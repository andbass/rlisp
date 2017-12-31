extern crate rlisp;
extern crate readline;

use rlisp::Lisp;
use rlisp::eval::{FuncResult, FuncError};
use rlisp::parse::{ParseError};

use std::io::{self, Read, Write};

fn main() {
    let mut lisp = Lisp::new();

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
