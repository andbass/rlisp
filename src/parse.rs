
use regex::Regex;

use std::collections::VecDeque;
use std::fmt;

use eval::FuncError;
use value::Value;

#[derive(Debug, Clone)]
pub enum ParseError {
    UnclosedList,
    InvalidListDelimitter,
    UnreadableSourceCode,
}

#[derive(Debug, Clone)]
pub struct FilePos {
    line: usize,
    col: usize,
}

impl FilePos {
    fn from_offset(text: &str, mut offset: usize) -> FilePos {
        let mut line_no = 1;
       
        for (cur_offset, ch) in text.bytes().enumerate() {
            if cur_offset >= offset {
                break;
            }

            if ch == '\n' as u8 {
                line_no += 1;
            }
        }

        FilePos {
            line: line_no,
            col: 0,
        }
    }
}

pub type TokenizeResult = Result<Vec<Value>, ParseError>; // result of tokenzing a collection of tokens
pub type ParseResult = Result<Value, ParseError>; // result of tokenizing a single token

const LIST_OPEN: &'static str = "(";
const LIST_CLOSE: &'static str = ")";

const QUOTE_OPEN: &'static str = "{";
const QUOTE_CLOSE: &'static str = "}";

pub fn write_list<T>(fmt: &mut fmt::Formatter, list: &Vec<T>, start: &str, sep: &str, end: &str) -> fmt::Result where T: fmt::Debug {
    if list.len() == 0 {
        return write!(fmt, "{}{}", start, end);
    }

    write!(fmt, "{}{:?}", start, list[0]);

    for token in &list[1..] {
        write!(fmt, "{}{:?}", sep, token);
    }

    write!(fmt, "{}", end)
}

fn preprocess(code: &str) -> VecDeque<String> {
    let string_re = r#""[^"]*""#;
    let sym_re = r"[-!?#\w\.]+";
    let num_re = r"\d+\.?\d*e?\d*";
    let list_re = r"[(){}\[\]]";
    let op_re = r"\+|-|\*|/|\^|&|\||=|\\|<|>";
	let quote_re = r"'";
    
    let regex = format!("{}|{}|{}|{}|{}|{}", string_re, num_re, sym_re, list_re, op_re, quote_re);

    let re = match Regex::new(&regex) {
        Ok(re) => re,
        Err(_) => return VecDeque::new(),
    };

    let spaced_code = code.to_string()
        .replace("(", " ( ")
        .replace(")", " ) ");

    let mut token_strs = VecDeque::new();

    for cap in re.captures_iter(&spaced_code) {
        let (offset, _) = cap.pos(0).unwrap_or((0, 0));
        let match_str = cap.at(0).unwrap_or("");

        token_strs.push_back(match_str.to_string());
    }

    return token_strs;
}

pub fn tokenize_str(code: &str) -> TokenizeResult {
    let mut tokens = Vec::new();
    
    let mut seperated_code = preprocess(code);

    while seperated_code.len() > 0 {
        tokens.push(try!(tokenize(&mut seperated_code)));
    }

    Ok(tokens)
}

/// Extracts a single token out of a list of strings, which may contain multiple tokens
fn tokenize(list: &mut VecDeque<String>) -> ParseResult {
    let token_str = match list.pop_front() {
        Some(string) => string,
        None => return Err(ParseError::UnreadableSourceCode),
    };

    match &token_str[..] {
        LIST_OPEN => { 
            let tokens = try!(tokenize_list(list, LIST_CLOSE));
            Ok(Value::List(tokens))
        },
        QUOTE_OPEN => {
            let tokens = try!(tokenize_list(list, QUOTE_CLOSE));
            Ok(Value::Quote(box Value::List(tokens)))
        },
        LIST_CLOSE | QUOTE_CLOSE => Err(ParseError::InvalidListDelimitter),
		r"'" => {
			let token = try!(tokenize(list));
			Ok(Value::Quote(box token))
		},
        atom => Ok(tokenize_atom(atom.to_string())),
    }
}

fn tokenize_atom(atom: String) -> Value {
    if let Ok(n) = atom.parse() {
        Value::Number(n)
    } else if let Some(lit) = string_lit(&atom[..]) {
        Value::String(lit) 
    } else {
        Value::Symbol(atom.to_string())
    }
}

fn tokenize_list(list: &mut VecDeque<String>, delimit: &str) -> Result<Vec<Value>, ParseError> {
    let mut tokens = Vec::new();

    while let Some(item_str) = list.pop_front() {
        if &item_str[..] == delimit {
            return Ok(tokens);
        }

        // If not the end of the list, push the item back and continue to process the list
        list.push_front(item_str);
        tokens.push(try!(tokenize(list)));
    }

    Err(ParseError::UnclosedList)
}

fn string_lit(slice: &str) -> Option<String> {
    let len = slice.len();
    let end = len - 1;

    if &slice[0..1] == "\"" && &slice[end .. len] == "\"" {
        let lit = &slice[1..end];

        let lit_str = lit.to_string()
            .replace(r"\n", "\n")
            .replace(r"\t", "\t")
            .replace(r"\r", "\r");

        return Some(lit_str);
    }

    return None;
}
