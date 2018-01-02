
#![feature(box_syntax, iterator_step_by, get_type_id, rc_downcast)]

extern crate regex;

macro_rules! invalid_args {
    ($lisp:expr, $expected:expr, $args:expr) => {
        $lisp.exit_scope();
        return Err(FuncError::InvalidArguments {
            expected: $expected,
            got: $args.len(),
        });
    }
}

pub mod parse;
pub mod eval;
pub mod value;
pub mod valtype;
pub mod env;

mod error_msg;
mod default_env;

pub use eval::{Lisp, FuncResult, FuncError};
pub use env::Env;
pub use value::{Value, ToLisp, FromLisp, ForeignType};
pub use parse::{ParseResult, ParseError};
