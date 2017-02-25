use tokenize::Token;
use tokenize;
use value::Value;
use stdlib;

pub fn exec_fn(name: String, stack: &mut Vec<Value>) -> Result<Value, String> {
    stdlib::match_fn(name, stack)
}

pub fn interpret(code: Vec<Token>) -> Result<Value, String> {
    let mut valstack = Vec::new();
    for elem in code {
        match elem {
            Token::Value(v) => valstack.push(v),
            Token::Function(name) => {
                let val = exec_fn(name, &mut valstack)?;
                valstack.push(val);
            },
            _ => {
                if tokenize::OPS.contains(&elem) {
                    let val = exec_fn(tokenize::function_of_token(elem), &mut valstack)?;
                    valstack.push(val);
                }
            }
        }
    }
    if valstack.len() == 1 {
        Ok(valstack.pop().unwrap())
    }
    else {
        Err(format!("unused arguments: {:?}", valstack))
    }
}

//shunting yard
pub fn rearrange(token: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();
    let mut stack = Vec::new();
    for elem in token {
        match elem {
            Token::Value(_) => {
                result.push(elem);
            },
            Token::BrOpen => {
                stack.push(elem);
            },
            Token::Function(_) => {
                stack.push(elem);
            },
            Token::Seperator => {
                let mut success = false;
                while !stack.is_empty() {
                    if *stack.last().unwrap() == Token::BrOpen {
                        success = true;
                        break;
                    }
                    result.push(stack.pop().unwrap());
                }
                if success == false {
                    return Err(String::from("function call malformed"))
                }
            }
            Token::BrClose => {
                while !stack.is_empty() &&
                        *stack.last().unwrap() != Token::BrOpen {
                    result.push(stack.pop().unwrap());
                }
                match stack.pop() {
                    Some(_) => {},
                    None => return Err(String::from("mismatched brackets"))
                }
                if !stack.is_empty() {
                    let func = stack.pop().unwrap();
                    match func {
                        Token::Function(_) => result.push(func),
                        _ => {stack.push(func)}
                    }
                }
            }
            _ => {
                if tokenize::OPS.contains(&elem) {
                    while !stack.is_empty() &&
                            tokenize::OPS.contains(stack.last().unwrap()) &&
                            stack.last().unwrap().precedence() > elem.precedence() &&
                            stack.last().unwrap().left_assoc() {
                        result.push(stack.pop().unwrap());
                    }
                    stack.push(elem);
                }
            }
        };

    }
    while !stack.is_empty() {
        result.push(stack.pop().unwrap());
    }
    Ok(result)
}
