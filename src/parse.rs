
use regex::Regex;

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum ParseError {
    UnclosedList,
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
    let sym_re = r"_*-*\w+_*-*\w*_*-*";
    let num_re = r"\d+\.*\d*";
    let list_re = r"\(|\)";
    let op_re = r"\+|-|\*|/|\^|&|\||=";
    
    let regex = format!("(?P<word>{}|{}|{}|{}|{})", string_re, num_re, sym_re, list_re, op_re);

    let re = match Regex::new(&regex) {
        Ok(re) => re,
        Err(e) => return VecDeque::new(),
    };

    let spaced_code = code.to_string()
        .replace("(", " ( ")
        .replace(")", " ) ");

    let spaced_code = spaced_code.trim();

    let mut spaced_vec = VecDeque::new();
    for cap in re.captures_iter(&spaced_code) {
        spaced_vec.push_back(cap.name("word").unwrap().to_string());
    }

    spaced_vec
}

pub fn tokenize_str(code: &str) -> ParseResult {
    let mut seperated_code = preprocess(code);

    tokenize(&mut seperated_code)
}

fn tokenize(list: &mut VecDeque<String>) -> ParseResult {
    let token_str = list.remove(0).unwrap_or(format!("42"));

    match &token_str[..] {
        "(" => {
            let mut tokens = Vec::new();

            while let Some(item_str) = list.remove(0) {
                if &item_str[..] == ")" {
                    return Ok(Token::List(tokens));
                }

                tokens.push(tokenize_atom(item_str));    
            }

            Err(ParseError::UnclosedList)
        },
        atom => {
            Ok(tokenize_atom(atom.to_string()))
        }
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
