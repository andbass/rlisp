
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
            let mut total = try!(f32::from_lisp(items.remove(0)));

            for item in items {
                total = $op(total, try!(f32::from_lisp(item.clone())));
            }

            Ok(total.to_lisp())
        }
    }
}

// Core functions
pub fn define(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let sym = try!(Token::from_lisp(vals.remove(0)));
    let sym = try!(sym.as_sym());

    let val = vals.remove(0);

    lisp.set_global(&sym, val);
    
    Ok(Value::Nil)
}

pub fn lambda(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let args: Token = try!(Token::from_lisp(vals.remove(0)));

    let mut body = Vec::new();
    for val in vals {
        let token = try!(Token::from_lisp(val.clone()));
        body.push(token); 
    }

    let args = try!(args.as_list());

    let mut arg_strs = Vec::new();
    for arg in args {
        arg_strs.push(try!(arg.as_sym()));
    }

    Ok(Value::Lambda {
        args: arg_strs,
        body: body,
    })
}

pub fn if_fn(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let cond: bool = try!(bool::from_lisp(vals.remove(0)));
    
    let token = try!(Token::from_lisp(vals.remove(0)));
    let else_token = try!(Token::from_lisp(vals.remove(0)));

    lisp.eval_token(if cond { 
        token 
    } else { 
        else_token 
    })
}

pub fn eval(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let token = try!(Token::from_lisp(vals.remove(0)));
    lisp.eval_token(token)
}

pub fn seq(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let ret_val = vals.remove(0);

    let ret_token = try!(Token::from_lisp(ret_val));

    for val in vals {
        let token = try!(Token::from_lisp(val.clone()));
        try!(lisp.eval_token(token));
    }

    lisp.eval_token(ret_token)
}

// Prints all variables current being tracked in all scopes, with exception to the global scope
// (that holds the stdlib)
pub fn scope_trace(_: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let mut index = 1;
    for env in &lisp.scopes[1..] {
        println!("{}: {:?}", index, env.map);
        index += 1;
    }

    Ok(Value::Nil)
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
            _ => return Err(FuncError::InvalidType),
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
            _ => return Err(FuncError::InvalidType),
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
        let bool_val = try!(bool::from_lisp(val.clone()));

        if !bool_val {
            return Ok(false.to_lisp());
        }
    }

    Ok(true.to_lisp())
}

pub fn or(vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    for val in vals {
        let bool_val = try!(bool::from_lisp(val.clone()));

        if bool_val {
            return Ok(true.to_lisp());
        }
    }

    Ok(false.to_lisp())
}

pub fn not(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let bool_val = try!(bool::from_lisp(vals.remove(0)));
    Ok((!bool_val).to_lisp())
}

pub fn cons(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let head: Token = try!(Token::from_lisp(vals.remove(0)));
    let tail: Token = try!(Token::from_lisp(vals.remove(0)));

    let mut tail_tokens = try!(tail.as_list());

    tail_tokens.insert(0, head);

    Ok(Value::Quote(Token::List(tail_tokens)))
}

math!(add, ops::Add::add);
math!(sub, ops::Sub::sub);
math!(mul, ops::Mul::mul);
math!(div, ops::Div::div);
