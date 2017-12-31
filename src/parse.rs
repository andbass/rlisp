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
    InvalidHexLiteral,
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

fn tokenize(code: &str) -> VecDeque<String> {
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
        let (offset, _) = cap.get(0)
            .map(|text| (text.start(), text.end()))
            .unwrap_or((0, 0));

        let match_str = cap.get(0)
            .map(|text| text.as_str())
            .unwrap_or("");

        token_strs.push_back(match_str.to_string());
    }

    return token_strs;
}

pub fn parse_str(code: &str) -> Result<Vec<Value>, ParseError> {
    let mut values = Vec::new();

    let mut tokens = tokenize(code);

    while tokens.len() > 0 {
        values.push(try!(parse(&mut tokens)));
    }

    Ok(values)
}

/// Extracts a single token out of a list of strings, which may contain multiple tokens
fn parse(list: &mut VecDeque<String>) -> ParseResult {
    let head = match list.pop_front() {
        Some(string) => string,
        None => return Err(ParseError::UnreadableSourceCode),
    };

    match &head[..] {
        LIST_OPEN => {
            let tokens = try!(parse_list(list, LIST_CLOSE));
            Ok(Value::List(tokens))
        },
        QUOTE_OPEN => {
            let tokens = try!(parse_list(list, QUOTE_CLOSE));
            Ok(Value::Quote(box Value::List(tokens)))
        },
        LIST_CLOSE | QUOTE_CLOSE => Err(ParseError::InvalidListDelimitter),
        r"'" => {
            let token = try!(parse(list));
            Ok(Value::Quote(box token))
        },
        atom => parse_atom(atom.to_string()),
    }
}

fn parse_atom(atom: String) -> ParseResult {
    if atom.starts_with("#") {
        let value = usize::from_str_radix(&atom[1..], 16).map_err(|_| ParseError::InvalidHexLiteral)?;
        Ok(Value::Number(value as f32))
    } else if let Ok(n) = atom.parse() {
        Ok(Value::Number(n))
    } else if let Some(lit) = string_lit(&atom[..]) {
        Ok(Value::String(lit))
    } else {
        Ok(Value::Symbol(atom.to_string()))
    }
}

fn parse_list(list: &mut VecDeque<String>, delimit: &str) -> Result<Vec<Value>, ParseError> {
    let mut tokens = Vec::new();

    while let Some(item_str) = list.pop_front() {
        if &item_str[..] == delimit {
            return Ok(tokens);
        }

        // If not the end of the list, push the item back and continue to process the list
        list.push_front(item_str);
        tokens.push(try!(parse(list)));
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
