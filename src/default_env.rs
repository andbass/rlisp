
use std::collections::HashMap;
use std::io::{self, Read};
use std::ops;

use value::{Value};
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

            Ok(Value::Number(total))
        }
    }
}

pub fn print(vals: Vec<Value>) -> FuncResult {
    for val in vals {
        print!("{:?} ", val);
    }

    println!("");

    Ok(Value::Void)
}

pub fn read(vals: Vec<Value>) -> FuncResult {
    if vals.len() != 0 {
        return Err(FuncError::InvalidArguments);
    }

    let mut stdin = io::stdin();

    let mut input = String::new();
    match stdin.read_line(&mut input) {
        Ok(_) => {
            match input.pop() {
                None | Some('\n') => (),
                Some(ch) => {
                    input.push(ch);
                },
            }
            
            Ok(Value::Str(input))
        }
        Err(err) => Err(FuncError::IoError(err)),
    }
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

    Ok(Value::Str(result))
}

pub fn eq(vals: Vec<Value>) -> FuncResult {
    for i in (0 .. vals.len() - 1) {
        if vals[i] != vals[i + 1] {
            return Ok(Value::Bool(false));
        }
    }

    Ok(Value::Bool(true))
}

pub fn and(vals: Vec<Value>) -> FuncResult {
    for val in vals {
        match val {
            Value::Bool(val) => {
                if !val {
                    return Ok(Value::Bool(false));
                }
            },
            _ => return Err(FuncError::InvalidArguments),
        }
    }

    Ok(Value::Bool(true))
}

pub fn or(vals: Vec<Value>) -> FuncResult {
    for val in vals {
        match val {
            Value::Bool(val) => {
                if val {
                    return Ok(Value::Bool(true));
                }
            },
            _ => return Err(FuncError::InvalidArguments),
        }
    }

    Ok(Value::Bool(false))
}

math!(add, ops::Add::add);
math!(sub, ops::Sub::sub);
math!(mul, ops::Mul::mul);
math!(div, ops::Div::div);
