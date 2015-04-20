
use std::io;
use std::fmt;
use std::collections::HashMap;

use parse::{self, Token, ParseError};
use value::{Value, FromLisp, ToLisp, FnWrapper};
use env::Env;

pub type FuncResult = Result<Value, FuncError>;

#[derive(Debug)]
pub enum FuncError {
    InvalidArguments,
    InvalidType,
    UndeclaredSymbol(String),
    AttemptToCallNonFunction,
    IoError(io::Error),
    
    ParsingErr(ParseError),
}

pub struct Lisp {
    scopes: Vec<Env>,
}

impl Lisp {
    pub fn new() -> Lisp {
        Lisp {
            scopes: vec![Env::std_lib()], 
        }
    }

    pub fn eval<T: FromLisp>(&mut self, code: &str) -> Result<T, FuncError> {
        let result = try!(self.eval_raw(code));

        return T::from_lisp(result);
    }

    pub fn eval_raw(&mut self, code: &str) -> FuncResult {
        let mut token = match parse::tokenize_str(code) {
            Ok(tok) => tok,
            Err(err) => return Err(FuncError::ParsingErr(err)),
        };

        self.eval_token(token)
    }

    pub fn eval_token(&mut self, token: Token) -> FuncResult {
        match token {
            Token::Number(n) => Ok(Value::Number(n)),
            Token::StrLit(lit) => Ok(Value::Str(lit)),
            Token::Sym(sym) => {
                for env in self.scopes.iter().rev() {
                    if let Some(val) = env.map.get(&sym) {
                        return Ok(val.clone());
                    }
                }

                Err(FuncError::UndeclaredSymbol(sym))
            },
            Token::List(mut tokens) => {
                self.sub_scope(); // each list has its own scope
                let func = try!(self.eval_token(tokens.remove(0)));

                match func {
                    Value::HardFunc(FnWrapper(func)) => {
                        let mut args = Vec::new();
                        for token in tokens {
                            args.push(try!(self.eval_token(token)));
                        }

                        func(args)
                    },
                    _ => Err(FuncError::AttemptToCallNonFunction),
                } 
            },
			Token::Quoted(tok) => Ok(Value::Quote(*tok)),
        }
    }

    pub fn set_global<T: ToLisp>(&mut self, name: &str, value: T) {
        self.scopes[0].set(name, value);
    }

    pub fn sub_scope(&mut self) {
        self.scopes.push(Env::new());
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::List(ref values) => write!(fmt, "{:?}", values),
            &Value::Str(ref string) => write!(fmt, "{:?}", string),
            &Value::Number(num) => write!(fmt, "{}", num),
            &Value::HardFunc(_) => write!(fmt, "HardFunc"),
            &Value::Nil => write!(fmt, "nil"),
            &Value::Bool(val) => write!(fmt, "{}", val),
			&Value::Quote(ref tok) => write!(fmt, "Quote({:?})", tok),
        }
    }
}

