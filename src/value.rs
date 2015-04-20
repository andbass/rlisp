
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
        let &FnWrapper(self_f) = self; 
        let &FnWrapper(rhs_f) = rhs;
        
        let self_ptr = (self_f as *const fn(Vec<Value>) -> FuncResult);
        let rhs_ptr = (rhs_f as *const fn(Vec<Value>) -> FuncResult);

        self_ptr == rhs_ptr
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
