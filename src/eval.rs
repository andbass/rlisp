
use std::fmt;
use std::collections::HashMap;

use parse::{self, Token, ParseError};
use default_env;

pub type FuncResult = Result<Value, FuncError>;

#[derive(Debug, Clone)]
pub enum FuncError {
    InvalidArguments,
    InvalidType,
    UndeclaredSymbol(String),
    AttemptToCallLiteral,
    ParsingErr(ParseError),
}

#[derive(Clone)]
pub struct FnWrapper(fn(Vec<Value>) -> FuncResult);

pub fn hard_fn(f: fn(Vec<Value>) -> FuncResult) -> Value {
    Value::HardFunc(FnWrapper(f))
}

#[derive(Clone, PartialEq)]
pub enum Value {
    Number(f32),
    Bool(bool),
    Str(String),

    HardFunc(FnWrapper),

    List(Vec<Value>),
    Void,
}

pub struct Lisp {
    env: HashMap<String, Value>,
}

impl Lisp {
    pub fn new() -> Lisp {
        Lisp {
            env: default_env::env(),
        }
    }

    pub fn eval(&mut self, code: &str) -> FuncResult {
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
            Token::Sym(sym) => self.env.get(&sym).cloned().ok_or(FuncError::UndeclaredSymbol(sym)),
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

impl Value {
    pub fn to_f32(self) -> Result<f32, FuncError> {
        match self {
            Value::Number(n) => Ok(n),
            _ => Err(FuncError::InvalidType),
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
            &Value::Void => write!(fmt, "Void"),
            &Value::Bool(val) => write!(fmt, "{}", val),
        }
    }
}

impl PartialEq for FnWrapper {
    fn eq(&self, rhs: &FnWrapper) -> bool {
        let selfPtr = self as *const Self;
        let rhsPtr = rhs as *const Self;

        selfPtr == rhsPtr
    }
}
