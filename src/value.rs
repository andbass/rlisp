
use std::rc::Rc;

use parse::Token;
use eval::{Lisp, FuncError, FuncResult};

pub type RawFunc = fn(Vec<Value>, &mut Lisp) -> FuncResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Args {
    Variant, // Any argument length is allowed
    Fixed(usize), // One possible number of arguments
    Multiple(Vec<usize>), // Multiple argument lengths are allowed
    Atleast(usize), // Must contain this many or greater args
}

#[derive(Clone)]
pub struct Func {
    pub func: Rc<RawFunc>,
    pub args: Args,
}

pub fn func(func: RawFunc, args: Args) -> Func {
    Func {
        func: Rc::new(func),
        args: args,
    }
}

#[derive(Clone, PartialEq)]
pub enum Value {
    Number(f32),
    Bool(bool),
    Str(String),

    // For some reason, fns that take reference arguments are not clonable on their own
    HardFunc(Func), 
    Lambda { 
        args: Vec<String>,
        body: Vec<Token>,
    },

    List(Vec<Value>),
    Nil,

	Quote(Token),
}

pub trait ToLisp {
    fn to_lisp(self) -> Value;
}

pub trait FromLisp {
    fn from_lisp(Value) -> Result<Self, FuncError>;
}

impl PartialEq for Func {
    // Checks to see if the addresses of both functions are the same
    fn eq(&self, rhs: &Func) -> bool {
        if self.args == rhs.args {
            let f1 = *self.func as *const fn(Vec<Value>, &mut Lisp) -> FuncResult;
            let f2 = *rhs.func as *const fn(Vec<Value>, &mut Lisp) -> FuncResult;

            return f1 == f2;
        }
         
        return false;
    }
}

macro_rules! lisp_impl {
    ($( $t:ty: $path:path ),+) => {
        $(
            impl FromLisp for $t {
                fn from_lisp(val: Value) -> Result<$t, FuncError> {
                    match val {
                        $path(val) => Ok(val),
                        _ => Err(FuncError::InvalidType),
                    }
                }
            }

            impl ToLisp for $t {
                fn to_lisp(self) -> Value {
                    $path(self)
                }
            }
        )*
    }
}

lisp_impl!(bool: Value::Bool, 
          f32: Value::Number, 
          String: Value::Str,
          Func: Value::HardFunc,
          Token: Value::Quote);

impl ToLisp for () {
    fn to_lisp(self) -> Value { Value::Nil }
}

impl FromLisp for () {
    fn from_lisp(val: Value) -> Result<(), FuncError> {
        match val {
            Value::Nil => Ok(()),
            _ => Err(FuncError::InvalidType),
        }
    }
}

impl ToLisp for Value {
    fn to_lisp(self) -> Value { self }
}
