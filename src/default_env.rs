
use std::process;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::ops;

use value::{Value, ToLisp};
use eval::{FuncError, FuncResult};

macro_rules! math {
    ($name:ident, $op:path) => {
        pub fn $name(mut items: Vec<Value>) -> FuncResult {
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

pub fn pow(vals: Vec<Value>) -> FuncResult {
    if vals.len() != 2 {
        return Err(FuncError::InvalidArguments);
    }

    match vals[0] {
        Value::Number(base) => {
            match vals[1] {
                Value::Number(exp) => {
                    Ok(base.powf(exp).to_lisp())
                },
                _ => return Err(FuncError::InvalidArguments),
            }
        },
        _ => return Err(FuncError::InvalidArguments),
    }
}

pub fn print(vals: Vec<Value>) -> FuncResult {
    for val in vals {
        match val {
            Value::Str(string) => print!("{}", string),
            _ => print!("{:?}", val),
        }
    }

    println!("");
    Ok(Value::Nil)
}

pub fn input(mut vals: Vec<Value>) -> FuncResult {
    if vals.len() > 1 {
        return Err(FuncError::InvalidArguments);
    }

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

pub fn exit(vals: Vec<Value>) -> FuncResult {
    if vals.len() > 1 {
        return Err(FuncError::InvalidArguments);
    }

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

pub fn str_fn(vals: Vec<Value>) -> FuncResult {
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

pub fn eq(vals: Vec<Value>) -> FuncResult {
    for i in (0 .. vals.len() - 1) {
        if vals[i] != vals[i + 1] {
            return Ok(false.to_lisp());
        }
    }

    Ok(true.to_lisp())
}

pub fn and(vals: Vec<Value>) -> FuncResult {
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

pub fn or(vals: Vec<Value>) -> FuncResult {
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

pub fn not(vals: Vec<Value>) -> FuncResult {
    if vals.len() != 1 {
        return Err(FuncError::InvalidArguments);
    }

    match vals[0] {
        Value::Bool(val) => Ok(Value::Bool(!val)),
        _ => Err(FuncError::InvalidArguments),
    }
}

math!(add, ops::Add::add);
math!(sub, ops::Sub::sub);
math!(mul, ops::Mul::mul);
math!(div, ops::Div::div);
