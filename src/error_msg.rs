
use std::fmt;

use eval::FuncError;
use parse;

impl fmt::Debug for FuncError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &FuncError::InvalidArguments { ref expected, got } => {
                write!(fmt, "expected {:?} arguments, but got {}", expected, got)
            },
            &FuncError::InvalidType { ref expected, ref got } => {
                try!(write!(fmt, "expected "));
                try!(parse::write_list(fmt, expected, "", " or ", ", "));
                write!(fmt, "but got a {:?} with a value of {:?}", got.typ(), got)
            },
            &FuncError::UndeclaredSymbol(ref sym) => {
                write!(fmt, "{} does not refer to a valid value stored in any currently accessible scope", sym)
            },
            &FuncError::AttemptToCallNonFunction(ref val) => {
                write!(fmt, "{:?} is not a callable function", val)
            },
            &FuncError::AttemptToEvalEmptyList => {
                write!(fmt, "Attempt to eval empty list")
            },
            &FuncError::GivenEmptyList => {
                write!(fmt, "Cannot take any elements out of any empty list")
            },
            &FuncError::IoError(ref error) => {
                write!(fmt, "An IO error occured: {:?}", error)
            },
            &FuncError::ParsingErr(ref err) => {
                write!(fmt, "Error while parsing source code: {:?}", err)
            },
        }
    }
}
