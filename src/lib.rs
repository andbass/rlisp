
#![feature(box_syntax, step_by)]

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
pub mod env;
mod default_env;

pub use eval::{Lisp, FuncResult, FuncError};
pub use env::Env;
pub use value::{Value, ToLisp, FromLisp};
pub use parse::{ParseResult, ParseError};
