use std::collections::HashMap;
use std::str::FromStr;

use value::Value;

pub const OPS: [Token; 5] = [Token::Add, Token::Sub, Token::Mul, Token::Div, Token::Pow];

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,
    Sub,
    SubMonad,
    Mul,
    Div,
    Pow,
    BrOpen,
    BrClose,
    Assign,
    Seperator,
    Value(Value),
    Function(String)
}

impl Token {
    pub fn precedence(&self) -> usize {
        match *self {
            Token::Add => 1,
            Token::Sub => 1,
            Token::Mul => 2,
            Token::Div => 2,
            Token::Pow => 3,
            _ => 0
        }
    }

    pub fn left_assoc(&self) -> bool {
        *self != Token::Pow
    }
}

pub fn function_of_token(token: Token) -> String {
    match token {
        Token::Add => String::from("add"),
        Token::Sub => String::from("sub"),
        Token::Mul => String::from("mul"),
        Token::Div => String::from("div"),
        Token::Pow => String::from("pow"),
        _ => panic!()
    }
}

pub fn is_assignment(vec: Vec<Token>) -> bool {
    vec.iter().any(|x| *x == Token::Assign)
}

pub fn tokenize(mut line: String, vars: &HashMap<String, Value>) -> Vec<Token> {
    let mut result = Vec::new();
    line = space_ops(line);
    let mut enclose_next = false; //set () around next token for [fn]!
    for word in line.split_whitespace() {
        let token = match_token(word, vars);
        if enclose_next {
            result.push(Token::BrOpen);
            result.push(token);
            result.push(Token::BrClose);
            enclose_next = false;
        }
        else {
            result.push(token);
        }
        if word.ends_with("!") {
            enclose_next = true;
        }
    }
    result
}

fn match_token(word: &str, vars: &HashMap<String, Value>) -> Token {
    match word {
        "+" => Token::Add,
        "-" => Token::Sub,
        "*" => Token::Mul,
        "/" => Token::Div,
        "(" => Token::BrOpen,
        ")" => Token::BrClose,
        "," => Token::Seperator,
        "^" => Token::Pow,
        _ => {
            if vars.contains_key(word) {
                Token::Value(vars.get(word).unwrap().clone())
            }
            else {
                match Value::from_str(word) {
                    Ok(v) => Token::Value(v),
                    Err(_) => {
                        let mut wordstr = word.to_owned();
                        if word.ends_with("!") {
                            let _ = wordstr.pop();
                        }
                        Token::Function(wordstr)
                    }
                }
            }
        }
    }
}

fn space_ops(line: String) -> String {
    let mut result = String::new();
    for ch in line.chars() {
        if is_spaced(ch) {
            result.push(' ');
            result.push(ch);
            result.push(' ');
        }
        else {
            result.push(ch);
        }
    }
    result
}

fn is_spaced(ch: char) -> bool {
    ['+', '-', '*', '/', '(', ')', ',', '^'].contains(&ch)
}
