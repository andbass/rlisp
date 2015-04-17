
#![feature(str_words, plugin)]

extern crate regex;

pub mod parse;
pub mod eval;
mod default_env;

pub use eval::{Lisp, Value, FuncResult, FuncError};
pub use parse::{ParseResult, ParseError};
