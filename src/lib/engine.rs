use std::collections::HashMap;

use interpreter;
use value::Value;
use tokenize;

pub struct Engine {
    vars: HashMap<String, Value>
}

impl Engine {
    pub fn new() -> Engine {
        let mut vars = HashMap::new();
        vars.insert(String::from("e"), Value::Float(2.718281828459045));
        vars.insert(String::from("pi"), Value::Float(3.141592653589793));
        Engine {
            vars: vars
        }
    }

    pub fn eval_stmt(&mut self, line: String) -> StatementResult{
        let token = tokenize::tokenize(line, &self.vars);
        let code = match interpreter::rearrange(token) {
            Ok(v) => v,
            Err(e) => return StatementResult::Error(e)
        };
        match interpreter::interpret(code) {
            Ok(v) => StatementResult::Value(v),
            Err(e) => StatementResult::Error(e)
        }
    }

    pub fn set_val(&mut self, name: String, val: Value) {
        self.vars.insert(name, val);
    }

    pub fn eval_line(&mut self, line: String) -> LineResult {
        if line.starts_with(":") {
            match line.find(char::is_whitespace) {
                Some(index) => {
                    let (cmd, param) = line.split_at(index);
                    match cmd {
                        ":set" => {
                            self.set_cmd(param.trim().to_owned())
                        },
                        _ => {
                            LineResult::Error(format!("unknown command: '{}'", cmd))
                        }
                    }
                },
                None => {
                    LineResult::Error(format!("expected arguments after '{}'", line))
                }
            }
        }
        else {
            self.eval_stmt(line).to_line_result()
        }
    }

    fn set_cmd(&mut self, param: String) -> LineResult {
        let split_index = match param.find(char::is_whitespace) {
            Some(v) => v,
            None => return LineResult::Error(format!("set usage: ':set [var] [expr]'"))
        };
        let (arg1, arg2) = param.split_at(split_index);
        let val = match self.eval_stmt(arg2.to_owned()) {
            StatementResult::Error(e) => {
                return LineResult::Error(e)
            },
            StatementResult::Value(v) => v
        };
        self.set_val(arg1.to_owned(), val);
        LineResult::Success
    }
}

pub enum StatementResult {
    Error(String),
    Value(Value)
}

impl StatementResult {
    pub fn to_line_result(self) -> LineResult {
        match self {
            StatementResult::Error(e) => LineResult::Error(e),
            StatementResult::Value(v) => LineResult::Value(v)
        }
    }
}

pub enum LineResult {
    Value(Value),
    Error(String),
    Success
}
