
use parse::Token;
use eval::{FuncError, FuncResult};

#[derive(Clone)]
pub struct FnWrapper(pub fn(Vec<Value>) -> FuncResult);

#[derive(Clone, PartialEq)]
pub enum Value {
    Number(f32),
    Bool(bool),
    Str(String),

    HardFunc(FnWrapper),

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

impl PartialEq for FnWrapper {
    // Checks to see if the addresses of both functions are the same
    fn eq(&self, rhs: &FnWrapper) -> bool {
        let selfPtr = self as *const Self;
        let rhsPtr = rhs as *const Self;

        selfPtr == rhsPtr
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
          FnWrapper: Value::HardFunc,
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
