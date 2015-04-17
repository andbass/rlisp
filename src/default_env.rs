
use std::collections::HashMap;
use std::ops;

use eval::{Value, hard_fn, FuncResult, FuncError};

macro_rules! math {
    ($name:ident, $op:path) => {
        fn $name(mut items: Vec<Value>) -> FuncResult {
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

fn print(vals: Vec<Value>) -> FuncResult {
    for val in vals {
        print!("{:?} ", val);
    }

    println!("");

    Ok(Value::Void)
}

fn str(vals: Vec<Value>) -> FuncResult {
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

fn eq(vals: Vec<Value>) -> FuncResult {
    for i in (0 .. vals.len() - 1) {
        if vals[i] != vals[i + 1] {
            return Ok(Value::Bool(false));
        }
    }

    Ok(Value::Bool(true))
}

math!(add, ops::Add::add);
math!(sub, ops::Sub::sub);
math!(mul, ops::Mul::mul);
math!(div, ops::Div::div);

pub fn env() -> HashMap<String, Value> {
    let mut map = HashMap::new();

    map.insert("true".to_string(), Value::Bool(true));
    map.insert("false".to_string(), Value::Bool(false));

    map.insert("+".to_string(), hard_fn(add));
    map.insert("-".to_string(), hard_fn(sub));
    map.insert("*".to_string(), hard_fn(mul));
    map.insert("/".to_string(), hard_fn(div));
    map.insert("=".to_string(), hard_fn(eq));

    map.insert("print".to_string(), hard_fn(print));

    map.insert("str".to_string(), hard_fn(str));

    map
}
