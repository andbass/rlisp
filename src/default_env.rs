
use std::process;
use std::io::{self, Write};
use std::ops;

use value::{Value, Args, ToLisp, FromLisp};
use valtype::Type;
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

fn make_lambda(args: Vec<Value>, body: Vec<Value>) -> FuncResult {
    let mut arg_strs = Vec::new();
    for arg in args {
        let sym = try!(arg.as_sym());
        arg_strs.push(sym);
    }

    Ok(Value::Lambda {
        args: arg_strs,
        body: body,
    })
}

// Core functions
pub fn define(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let sym = vals.remove(0);
    match sym {
        Value::Symbol(sym) => {
            let val = vals.remove(0);
            lisp.parent_scope().set(&sym, val);
        },
        Value::List(mut args) => {
            let name = try!(args.remove(0).as_sym());
            let func = try!(make_lambda(args, vals));

            lisp.parent_scope().set(&name, func);
        },
        _ => return Err(FuncError::InvalidType {
            expected: vec![Type::Symbol, Type::List],
            got: sym,
        }),
    }

    Ok(Value::Nil)
}

pub fn let_fn(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let defs = try!(vals.remove(0).as_list());

    for def in defs {
        let mut def = try!(def.as_list());
        if def.len() != 2 {
            return Err(FuncError::InvalidArguments {
                expected: Args::Fixed(2),
                got: def.len(),
            });
        }
        
        let name = try!(def.remove(0).as_sym());
        let value = try!(lisp.eval_token(def.remove(0)));

        lisp.cur_scope().set(&name, value);
    }

    lisp.eval_token_vec(vals)
}

pub fn type_of(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    let val = vals.remove(0);
    Ok(Value::Type(val.typ()))
}

// When defining a lambda, the first arg is the list of lambda args
// The rest of the arguments are the 'body' of the lambda
pub fn lambda(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    let args = try!(vals.remove(0).as_list());
    make_lambda(args, vals)
}

pub fn if_fn(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let cond: bool = try!(bool::from_lisp(vals.remove(0)));
    
    let token = vals.remove(0);
    let else_token = vals.remove(0);

    lisp.eval_token(if cond { 
        token 
    } else { 
        else_token 
    })
}

pub fn eval(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    lisp.eval_token_vec(vals)
}

pub fn id(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    Ok(vals.remove(0))
}

pub fn seq(vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let mut tokens = Vec::new();

    for val in vals {
        tokens.push(val);
    }

    lisp.eval_token_vec(tokens)
}

pub fn print(vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    for val in vals {
        match val {
            Value::String(string) => print!("{}", string),
            _ => print!("{:?}", val),
        }
    }

    println!("");
    Ok(Value::Nil)
}

pub fn input(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    let prompt = if vals.len() == 1 {
        try!(String::from_lisp(vals.remove(0)))
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

pub fn exit(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    let exit_code = if vals.len() == 1 {
        try!(f32::from_lisp(vals.remove(0))) as i32
    } else {
        0
    };

    process::exit(exit_code);
}

pub fn str_fn(vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    let mut result = String::new();

    for val in vals {
        let string = match val {
            Value::String(str) => str,
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

pub fn greater_than(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    let a = try!(f32::from_lisp(vals.remove(0)));
    let b = try!(f32::from_lisp(vals.remove(0)));

    Ok((a > b).to_lisp())
}

pub fn less_than(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    let a = try!(f32::from_lisp(vals.remove(0)));
    let b = try!(f32::from_lisp(vals.remove(0)));

    Ok((a < b).to_lisp())
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

// List ops
pub fn list(vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    Ok(Value::List(vals))
}

pub fn range(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    let step = if vals.len() == 3 {
        try!(f32::from_lisp(vals.remove(0))) as i32
    } else {
        1 
    };

    let initial = try!(f32::from_lisp(vals.remove(0))) as i32;
    let end = try!(f32::from_lisp(vals.remove(0))) as i32;
    
    let list = (initial..end)
        .step_by(step)
        .map(|n| (n as f32).to_lisp())
        .collect();

    Ok(Value::List(list))
}

pub fn len(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    let list = try!(vals.remove(0).as_list());
    Ok(Value::Number(list.len() as f32))
}

pub fn head(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    let mut list = try!(vals.remove(0).as_list());

    if list.len() == 0 {
        return Err(FuncError::GivenEmptyList);
    }

    Ok(list.remove(0))
}

pub fn tail(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    if vals.len() == 0 {
        return Err(FuncError::GivenEmptyList);
    }

    let mut list = try!(vals.remove(0).as_list());

    if list.len() == 0 {
        return Err(FuncError::GivenEmptyList);
    }

    list.remove(0);

    Ok(Value::List(list))
}

pub fn is_empty(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    let list = try!(vals.remove(0).as_list());
    Ok((list.len() == 0).to_lisp())
}

pub fn cons(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    let new_head = vals.remove(0);
    let mut list = try!(vals.remove(0).as_list());

    list.insert(0, new_head);
    Ok(Value::List(list))
}

pub fn join(mut vals: Vec<Value>, _: &mut Lisp) -> FuncResult {
    let mut list = try!(vals.remove(0).as_list());
    let new_last = vals.remove(0);

    list.push(new_last);
    Ok(Value::List(list))
}

pub fn map(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let func = vals.remove(0);
    let list = try!(vals.remove(0).as_list());

    let mut new_list = Vec::new();
    for val in list {
        let s_expr = Value::List(vec!(func.clone(), val));
        new_list.push(try!(lisp.eval_token(s_expr))); 
    }

    Ok(Value::List(new_list))
}


pub fn fold(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let func = vals.remove(0);
    let mut acc = vals.remove(0);
    let list = try!(vals.remove(0).as_list());

    for val in list {
        let s_expr = Value::List(vec![func.clone(), acc.clone(), val.clone()]);
        acc = try!(lisp.eval_token(s_expr));
    }

    Ok(acc)
}

pub fn filter(mut vals: Vec<Value>, lisp: &mut Lisp) -> FuncResult {
    let func = vals.remove(0);
    let list = try!(vals.remove(0).as_list());

    let mut new_list = Vec::new();
    for val in list {
        let s_expr = Value::List(vec![func.clone(), val.clone()]);
        let result = try!(lisp.eval_token(s_expr));

        let bool_result = try!(bool::from_lisp(result));

        if bool_result {
            new_list.push(val);
        }
    }

    Ok(Value::List(new_list))
}

math!(add, ops::Add::add);
math!(sub, ops::Sub::sub);
math!(mul, ops::Mul::mul);
math!(div, ops::Div::div);
