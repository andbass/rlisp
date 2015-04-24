
use std::io::{self, Read};
use std::fmt;
use std::collections::HashMap;

use parse::{self, Token, ParseError};
use value::{Value, Func, Args, FromLisp, ToLisp};
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
    pub scopes: Vec<Env>,
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

    pub fn eval_reader<R: Read>(&mut self, mut code: R) -> FuncResult {
        let mut read_string = String::new();
        code.read_to_string(&mut read_string);

        self.eval_raw(&read_string)
    }

    pub fn eval_raw(&mut self, code: &str) -> FuncResult {
        let mut tokens = match parse::tokenize_str(code) {
            Ok(tok) => tok,
            Err(err) => return Err(FuncError::ParsingErr(err)),
        };

        let ret_token = match tokens.pop() {
            Some(token) => token,
            None => unreachable!(),
        };

        for token in tokens {
            self.eval_token(token.clone());
        }

        self.eval_token(ret_token)
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
                    Value::HardFunc(hard_func) => {
                        let mut args = Vec::new();

                        for token in tokens {
                            args.push(try!(self.eval_token(token)));
                        }

                        match hard_func.args {
                            Args::Variant => (),
                            Args::Fixed(count) => {
                                if args.len() != count {
                                    self.exit_scope();
                                    return Err(FuncError::InvalidArguments);
                                }
                            },
                            Args::Multiple(possible_counts) => {
                                let mut arg_match = false;

                                // I wish I could label match statements and break out of them...
                                for count in possible_counts {
                                    if args.len() == count {
                                        arg_match = true;
                                        break;
                                    }
                                }

                                if !arg_match {
                                    self.exit_scope();
                                    return Err(FuncError::InvalidArguments);
                                }
                            },
                        }

                        let result = (hard_func.func)(args, self);
                        self.exit_scope();

                        result
                    },
                    Value::Lambda { args: args, body: body } => {
                        if tokens.len() != args.len() {
                            self.exit_scope();
                            return Err(FuncError::InvalidArguments);
                        }

                        for sym in args.iter() {
                            let value = try!(self.eval_token(tokens.remove(0)));
                            self.cur_scope().set(sym, value);
                        }

                        let result = self.eval_token(body);
                        self.exit_scope();
                        result
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

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn cur_scope(&mut self) -> &mut Env {
        let index = self.scopes.len() - 1;
        &mut self.scopes[index]
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::List(ref values) => write!(fmt, "{:?}", values),
            &Value::Str(ref string) => write!(fmt, "{:?}", string),
            &Value::Number(num) => write!(fmt, "{}", num),
            &Value::HardFunc(ref func) => write!(fmt, "HardFunc({:?})", func.args),
            &Value::Lambda { args: ref args, body: ref body }=> write!(fmt, "λ {:?} => {:?}", args, body),
            &Value::Nil => write!(fmt, "nil"),
            &Value::Bool(val) => write!(fmt, "{}", val),
			&Value::Quote(ref tok) => write!(fmt, "'{:?}", tok),
        }
    }
}

