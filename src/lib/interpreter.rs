use std::collections::HashMap;

use tokenize::Token;
use tokenize;
use value::Value;
use stdlib;
use engine::Engine;

pub fn interpret(code: Vec<Token>, engine: &Engine, local_vars: &HashMap<String, Value>) -> Result<Value, String> {
    let mut valstack = Vec::new();
    for elem in code {
        match elem {
            Token::Value(v) => valstack.push(v),
            Token::Variable(var) => {
                valstack.push(match local_vars.get(&var) {
                    Some(a) => a.clone(),
                    None => {
                        match engine.get_val(&var) {
                            Some(b) => b.clone(),
                            None => return Err(format!("variable '{}' not found", var))
                        }
                    }
                })
            },
            Token::Function(name) => {
                if !engine.functions().contains_key(&name) { //stdlib functions
                    let val = exec_fn(name, &mut valstack)?;
                    valstack.push(val);
                }
                else {
                    let function = engine.functions().get(&name).unwrap();
                    if valstack.len() < function.arg_count() {
                        return Err(format!("not enough arguments for function '{}'", name))
                    }
                    //fetch arguments + argument names in HashMap
                    let mut local_stack = Vec::new();
                    for _ in 0..function.arg_count() {
                        local_stack.push(valstack.pop().unwrap())
                    }
                    local_stack = local_stack.into_iter().rev().collect::<Vec<Value>>();
                    let args_zip = local_stack.into_iter().zip(
                        function.args().into_iter().map(|x| x.clone())
                    );
                    let mut local_vars = HashMap::new();
                    for arg_pair in args_zip {
                        local_vars.insert(arg_pair.1, arg_pair.0);
                    }

                    match interpret(function.code(), engine, &local_vars) {
                        Ok(v) => valstack.push(v),
                        Err(e) => {
                            return Err(format!("error in function '{}': \n{}", name, e))
                        }
                    };
                }
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
        if valstack.len() == 0 {
            Err(format!("no arguments"))
        }
        else {
            Err(format!("unused arguments: {:?}", valstack))
        }
    }
}

pub fn exec_fn(name: String, stack: &mut Vec<Value>) -> Result<Value, String> {
    stdlib::match_fn(name, stack)
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
            Token::Variable(_) => {
                result.push(elem);
            } ,
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
