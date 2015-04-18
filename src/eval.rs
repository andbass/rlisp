
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
    AttemptToCallLiteral,
    IoError(io::Error),
    
    ParsingErr(ParseError),
}

pub struct Lisp {
    env: Env,
}

impl Lisp {
    pub fn new() -> Lisp {
        Lisp {
            env: Env::new(), 
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
            Token::Sym(sym) => self.env.map.get(&sym).cloned().ok_or(FuncError::UndeclaredSymbol(sym)),
            Token::List(mut tokens) => {
                let func = try!(self.eval_token(tokens.remove(0)));

                match func {
                    Value::HardFunc(FnWrapper(func)) => {
                        let mut args = Vec::new();
                        for token in tokens {
                            args.push(try!(self.eval_token(token)));
                        }

                        func(args)
                    },
                    _ => Err(FuncError::AttemptToCallLiteral),
                } 
            }
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::List(ref values) => write!(fmt, "{:?}", values),
            &Value::Str(ref string) => write!(fmt, "{:?}", string),
            &Value::Number(num) => write!(fmt, "{:?}", num),
            &Value::HardFunc(_) => write!(fmt, "HardFunc"),
            &Value::Nil => write!(fmt, "nil"),
            &Value::Bool(val) => write!(fmt, "{}", val),
        }
    }
}

