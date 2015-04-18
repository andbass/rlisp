
use regex::Regex;

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum ParseError {
    UnclosedList,
    InvalidListDelimitter,
}

pub type ParseResult = Result<Token, ParseError>;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f32),
    StrLit(String),

    Sym(String),

    List(Vec<Token>),
}

fn preprocess(code: &str) -> VecDeque<String> {
    let string_re = r#""[^"]*""#;
    let sym_re = r"\w+";
    let num_re = r"\d+\.*\d*";
    let list_re = r"\(|\)";
    let op_re = r"\+|-|\*|/|\^|&|\||=";
    
    let regex = format!("{}|{}|{}|{}|{}", string_re, num_re, sym_re, list_re, op_re);

    let re = match Regex::new(&regex) {
        Ok(re) => re,
        Err(e) => return VecDeque::new(),
    };

    let spaced_code = code.to_string()
        .replace("(", " ( ")
        .replace(")", " ) ");

    let spaced_code = spaced_code.trim();

    let mut token_strs = VecDeque::new();

    for cap in re.captures_iter(&spaced_code) {
        let match_str = cap.at(0).unwrap_or("");
        token_strs.push_back(match_str.to_string());
    }

    return token_strs;
}

pub fn tokenize_str(code: &str) -> ParseResult {
    let mut seperated_code = preprocess(code);

    tokenize(&mut seperated_code)
}

fn tokenize(list: &mut VecDeque<String>) -> ParseResult {
    let token_str = list.pop_front().unwrap_or(format!("42"));

    match &token_str[..] {
        "(" => {
            let mut tokens = Vec::new();

            while let Some(item_str) = list.pop_front() {
                if &item_str[..] == ")" {
                    return Ok(Token::List(tokens));
                }

                // If not the end of the list, push the item back and continue to process the list
                list.push_front(item_str);
                tokens.push(try!(tokenize(list)));
            }

            Err(ParseError::UnclosedList)
        },
        ")" => Err(ParseError::InvalidListDelimitter),
        atom => Ok(tokenize_atom(atom.to_string())),
    }
}

fn tokenize_atom(atom: String) -> Token {
    if let Ok(n) = atom.parse() {
        Token::Number(n)
    } else if let Some(lit) = string_lit(&atom[..]) {
        Token::StrLit(lit) 
    } else {
        Token::Sym(atom.to_string())
    }
}

fn string_lit(slice: &str) -> Option<String> {
    let len = slice.len();
    let end = len - 1;

    if &slice[0..1] == "\"" && &slice[end .. len] == "\"" {
        let lit = &slice[1..end];
        return Some(lit.to_string());
    }

    return None;
}
