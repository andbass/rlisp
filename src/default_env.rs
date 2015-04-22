
use std::process;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::ops;

use value::{Value, ToLisp, FromLisp};
use parse::Token;
use eval::{Lisp, FuncError, FuncResult};

macro_rules! math {
    ($name:ident, $op:path) => {
        pub fn $name(mut items: Vec<Value>, _: &mut Lisp) -> FuncResult {
            let mut total = match items.remove(0) {
                Value::Number(n) => n,
                _ => return Err(FuncError::InvalidArguments),
            };

            for item in items {
                match item {
                    Value::Number(next) => {
                        total = $op(total, next);
                    },
                    _ => return Err(FuncError::InvalidArguments),
                }
            }

            Ok(total.to_lisp())
        }
    }
}

pub fn pow(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let base = try!(f32::from_lisp(vals.remove(0)));
    let exp = try!(f32::from_lisp(vals.remove(0)));

    Ok(base.powf(exp).to_lisp())
}

pub fn print(vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    for val in vals {
        match val {
            Value::Str(string) => print!("{}", string),
            _ => print!("{:?}", val),
        }
    }

    println!("");
    Ok(Value::Nil)
}

pub fn input(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let prompt = if vals.len() == 1 {
        match vals.remove(0) {
            Value::Str(prompt) => prompt,
            _ => return Err(FuncError::InvalidArguments),
        }
    } else {
        "".to_string()
    };

    print!("{}", prompt);
    match io::stdout().flush() {
        Ok(_) => (),
        Err(err) => return Err(FuncError::IoError(err)),
    }

    let mut stdin = io::stdin();

    let mut input = String::new();
    match stdin.read_line(&mut input) {
        Ok(_) => {
            input.pop(); // remove newline
            Ok(input.to_lisp())
        }
        Err(err) => Err(FuncError::IoError(err)),
    }
}

pub fn exit(vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let exit_code = if vals.len() == 1 {
        match vals[0] {
            Value::Number(code) => code as i32,
            _ => return Err(FuncError::InvalidArguments),
        }
    } else {
        0
    };

    process::exit(exit_code);
}

pub fn str_fn(vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let mut result = String::new();

    for val in vals {
        let string = match val {
            Value::Str(str) => str,
            other => format!("{:?}", other),
        };

        result.push_str(&string);
    }

    Ok(result.to_lisp())
}

pub fn eq(vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    for i in (0 .. vals.len() - 1) {
        if vals[i] != vals[i + 1] {
            return Ok(false.to_lisp());
        }
    }

    Ok(true.to_lisp())
}

pub fn and(vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    for val in vals {
        match val {
            Value::Bool(val) => {
                if !val {
                    return Ok(false.to_lisp());
                }
            },
            _ => return Err(FuncError::InvalidArguments),
        }
    }

    Ok(true.to_lisp())
}

pub fn or(vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    for val in vals {
        match val {
            Value::Bool(val) => {
                if val {
                    return Ok(true.to_lisp());
                }
            },
            _ => return Err(FuncError::InvalidArguments),
        }
    }

    Ok(false.to_lisp())
}

pub fn not(vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    match vals[0] {
        Value::Bool(val) => Ok(Value::Bool(!val)),
        _ => Err(FuncError::InvalidArguments),
    }
}

pub fn cons(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let head: Token = try!(Token::from_lisp(vals.remove(0)));
    let tail: Token = try!(Token::from_lisp(vals.remove(0)));

    let mut tail_tokens = match tail {
        Token::List(toks) => toks,
        _ => return Err(FuncError::InvalidArguments),
    };

    tail_tokens.insert(0, head);

    Ok(Value::Quote(Token::List(tail_tokens)))
}

math!(add, ops::Add::add);
math!(sub, ops::Sub::sub);
math!(mul, ops::Mul::mul);
math!(div, ops::Div::div);
