use std::str::FromStr;

use value::Value;

pub const SPACED_OPS: [char; 11] = ['+', '-', '*', '/', '(', ')', ',', '^', '=', '<', '>'];
pub const OPS: [Token; 9] = [Token::Add, Token::Sub, Token::Mul, Token::UnarySub,
                        Token::Div, Token::Pow, Token::Equals, Token::GreaterThan, Token::LesserThan];

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,
    Sub,
    UnarySub,
    SubMonad,
    Mul,
    Div,
    Pow,
    BrOpen,
    BrClose,
    Seperator,
    Equals,
    GreaterThan,
    LesserThan,
    Value(Value),
    Variable(String),
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
            Token::UnarySub => 4,
            Token::Equals => 5,
            Token::GreaterThan => 5,
            Token::LesserThan => 5,
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
        Token::UnarySub => String::from("neg"),
        Token::Mul => String::from("mul"),
        Token::Div => String::from("div"),
        Token::Pow => String::from("pow"),
        Token::Equals => String::from("eq"),
        Token::GreaterThan => String::from("gt"),
        Token::LesserThan => String::from("lt"),
        _ => panic!()
    }
}

pub fn tokenize(mut line: String) -> Vec<Token> {
    let mut result = Vec::new();
    line = space_ops(line);
    let mut enclose_next = false; //set () around next token for [fn]!
    let mut last_token = Token::Add;
    for word in line.split_whitespace() {
        let token = match_token(word, last_token);
        if enclose_next {
            result.push(Token::BrOpen);
            result.push(token.clone());
            result.push(Token::BrClose);
            enclose_next = false;
        }
        else {
            result.push(token.clone());
        }
        last_token = token;
        if word.ends_with("!") {
            enclose_next = true;
        }
    }
    result
}

fn match_token(word: &str, last_token: Token) -> Token {
    match word {
        "+" => Token::Add,
        "-" => {
            match last_token {
                Token::Value(_) | Token::Function(_) | Token::Variable(_) => Token::Sub,
                _ => Token::UnarySub
            }
        },
        "*" => Token::Mul,
        "/" => Token::Div,
        "(" => Token::BrOpen,
        ")" => Token::BrClose,
        "," => Token::Seperator,
        "^" => Token::Pow,
        "=" => Token::Equals,
        "<" => Token::LesserThan,
        ">" => Token::GreaterThan,
        _ => {
            match Value::from_str(word) {
                Ok(v) => Token::Value(v),
                Err(_) => {
                    let mut wordstr = word.to_owned();
                    if word.starts_with(char::is_uppercase) || word.starts_with('$'){
                        Token::Variable(wordstr)
                    }
                    else {
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
    SPACED_OPS.contains(&ch)
}
