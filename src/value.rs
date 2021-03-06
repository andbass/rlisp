use std::rc::Rc;
use std::marker::Sized;
use std::any::{Any, TypeId};
use std::fmt::Debug;

use eval::{Lisp, FuncError, FuncResult};
use valtype::Type;

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

pub trait ForeignType: Any + Debug { }

fn cast<'a, T: Any>(from: Rc<ForeignType>) -> Option<&'a T> {
    if (*from).get_type_id() == TypeId::of::<T>() {
        unsafe {
            let ptr = &*from;
            Some(&*(ptr as *const ForeignType as *const T))
        }
    } else {
        None
    }
}

#[derive(Clone, Debug)]
pub struct Foreign(Rc<ForeignType>);

impl PartialEq for Foreign {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[derive(Clone, PartialEq)]
pub enum Value {
    Number(f32),
    Bool(bool),

    Symbol(String),

    String(String),

    // For some reason, fns that take reference arguments are not clonable on their own
    HardFunc(Func),
    Lambda {
        args: Vec<String>,
        body: Vec<Value>,
    },

    List(Vec<Value>),
    Nil,

    Quote(Box<Value>),
    Type(Type),

    Foreign(Foreign),
}

impl<'a> Value {
    pub fn as_sym(self) -> Result<String, FuncError> {
        match self {
            Value::Symbol(sym) => Ok(sym),
            _ => Err(FuncError::InvalidType {
                expected: vec![Type::Symbol],
                got: self,
            }),
        }
    }

    pub fn as_list(self) -> Result<Vec<Value>, FuncError> {
        match self {
            Value::List(list) => Ok(list),
            _ => Err(FuncError::InvalidType {
                expected: vec![Type::List],
                got: self,
            }),
        }
    }

    pub fn as_foreign<T: ForeignType>(&self) -> Result<&T, FuncError> {
        match self {
            &Value::Foreign(Foreign(ref rc)) => {
                cast(rc.clone()).map(Ok).unwrap_or_else(move || {
                    Err(FuncError::InvalidType {
                        expected: vec![Type::Foreign(TypeId::of::<T>())],
                        got: self.clone()
                    })
                })
            },
            _ => Err(FuncError::InvalidType {
                expected: vec![Type::Foreign(TypeId::of::<T>())],
                got: self.clone(),
            }),
        }
    }

    pub fn typ(&self) -> Type {
        match self {
            &Value::Number(_) => Type::Number,
            &Value::Bool(_) => Type::Bool,
            &Value::Symbol(_) => Type::Symbol,
            &Value::String(_) => Type::String,
            &Value::HardFunc(_) => Type::HardFunc,
            &Value::Lambda { .. } => Type::Lambda,
            &Value::List(_) => Type::List,
            &Value::Nil => Type::Nil,
            &Value::Quote(ref val) => Type::Quote(box val.typ()),
            &Value::Type(_) => Type::Type,
            &Value::Foreign(ref value) => Type::Foreign(value.get_type_id()),
        }
    }
}

pub trait ToLisp {
    fn to_lisp(self) -> Value;
}

pub trait FromLisp where Self: Sized {
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
    ($( $t:ty: $ident:ident ),+) => {
        $(
            impl FromLisp for $t {
                fn from_lisp(val: Value) -> Result<$t, FuncError> {
                    match val {
                        Value::$ident(val) => Ok(val),
                        _ => Err(FuncError::InvalidType {
                            expected: vec![Type::$ident],
                            got: val
                        }),
                    }
                }
            }

            impl ToLisp for $t {
                fn to_lisp(self) -> Value {
                    Value::$ident(self)
                }
            }
        )*
    }
}

lisp_impl!(bool: Bool,
          f32: Number,
          String: String,
          Func: HardFunc,
          Type: Type);

impl ToLisp for () {
    fn to_lisp(self) -> Value { Value::Nil }
}

impl FromLisp for () {
    fn from_lisp(val: Value) -> Result<(), FuncError> {
        match val {
            Value::Nil => Ok(()),
            _ => Err(FuncError::InvalidType {
                expected: vec![Type::Nil],
                got: val,
            }),
        }
    }
}

impl ToLisp for Value {
    fn to_lisp(self) -> Value { self }
}

impl<T: ForeignType> ToLisp for Rc<T> {
    fn to_lisp(self) -> Value {
        Value::Foreign(Foreign(self))
    }
}
