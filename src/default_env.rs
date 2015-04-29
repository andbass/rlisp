
use std::process;
use std::io::{self, Write};
use std::ops;

use value::{Value, ToLisp, FromLisp};
use eval::{Lisp, FuncError, FuncResult};

macro_rules! math {
    ($name:ident, $op:path) => {
        pub fn $name(mut items: Vec<Value>, _: &mut Lisp) -> FuncResult {
            let mut total = try!(f32::from_lisp(items.remove(0)));

            for item in items {
                total = $op(total, try!(f32::from_lisp(item)));
            }

            Ok(total.to_lisp())
        }
    }
}

// Core functions
pub fn define(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let sym = try!(vals.remove(0).as_sym());

    let val = vals.remove(0);

    lisp.cur_scope().set(&sym, val);
    
    Ok(Value::Nil)
}

pub fn lambda(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    let args = try!(vals.remove(0).as_list());

    let mut arg_strs = Vec::new();
    for arg in args {
        let sym = try!(arg.as_sym());
        arg_strs.push(sym);
    }

    Ok(Value::Lambda {
        args: arg_strs,
        body: vals,
    })
}

pub fn if_fn(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let cond: bool = try!(bool::from_lisp(vals.remove(0)));
    
    let token = try!(vals.remove(0).unquote());
    let else_token = try!(vals.remove(0).unquote());

    lisp.eval_token(if cond { 
        token 
    } else { 
        else_token 
    })
}

pub fn eval(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    lisp.eval_token_vec(vals)
}

pub fn seq(vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let mut tokens = Vec::new();

    for val in vals {
        tokens.push(try!(val.unquote()));
    }

    lisp.eval_token_vec(tokens)
}

pub fn print(vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    for val in vals {
        match val {
            Value::Str(string) => print!("{}", string),
            _ => print!("{:?}", val),
        }
    }

    println!("");
    Ok(Value::Nil)
}

pub fn input(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
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

pub fn exit(vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
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

pub fn str_fn(vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
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

pub fn eq(vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    for i in (0 .. vals.len() - 1) {
        if vals[i] != vals[i + 1] {
            return Ok(false.to_lisp());
        }
    }

    Ok(true.to_lisp())
}

pub fn and(vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    for val in vals {
        let bool_val = try!(bool::from_lisp(val));

        if !bool_val {
            return Ok(false.to_lisp());
        }
    }

    Ok(true.to_lisp())
}

pub fn or(vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    for val in vals {
        let bool_val = try!(bool::from_lisp(val));

        if bool_val {
            return Ok(true.to_lisp());
        }
    }

    Ok(false.to_lisp())
}

pub fn not(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    let bool_val = try!(bool::from_lisp(vals.remove(0)));
    Ok((!bool_val).to_lisp())
}

pub fn list(vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    Ok(Value::List(vals))
}

math!(add, ops::Add::add);
math!(sub, ops::Sub::sub);
math!(mul, ops::Mul::mul);
math!(div, ops::Div::div);
