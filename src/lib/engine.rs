use std::collections::HashMap;
use std::io::{self, Read};
use std::fs::File;

use interpreter;
use value::Value;
use function::Function;
use tokenize;

pub struct Engine {
    //so that no empty hashmap has to be generated on every interpreted line (-> local vars)
    static_empty_map: HashMap<String, Value>,
    vars: HashMap<String, Value>,
    functions: HashMap<String, Function>
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            static_empty_map: HashMap::new(),
            vars: HashMap::new(),
            functions: HashMap::new()
        }
    }

    pub fn vars(&self) -> &HashMap<String, Value> {
        &self.vars
    }

    pub fn functions(&self) -> &HashMap<String, Function> {
        &self.functions
    }

    pub fn eval_stmt(&mut self, line: String) -> StatementResult{
        let token = tokenize::tokenize(line);
        let code = match interpreter::rearrange(token) {
            Ok(v) => v,
            Err(e) => return StatementResult::Error(e)
        };
        match interpreter::interpret(code, self, &self.static_empty_map) {
            Ok(v) => StatementResult::Value(v),
            Err(e) => StatementResult::Error(e)
        }
    }

    pub fn get_val(&self, name: &String) -> Option<&Value> {
        self.vars.get(name)
    }

    pub fn set_val(&mut self, name: String, val: Value) {
        self.vars.insert(name, val);
    }

    //true if already exists
    pub fn add_func(&mut self, name: String, func: Function) {
        let _ = self.functions.insert(name, func);
    }

    pub fn eval_line(&mut self, line: String) -> LineResult {
        if line.starts_with(":") {
            match line.find(char::is_whitespace) {
                Some(index) => {
                    let (cmd, param) = line.split_at(index);
                    match cmd {
                        ":var" => {
                            self.set_cmd(param.trim().to_owned())
                        },
                        ":fn" => {
                            self.fn_cmd(param.trim().to_owned())
                        },
                        ":load" => {
                            let script = match read_file(param.trim().to_owned()) {
                                Ok(v) => v,
                                Err(e) => return LineResult::Error(format!("file read error: {}", e))
                            };
                            for line in script.split('\n') {
                                if line.trim().len() == 0 || line.trim().starts_with("--") {
                                    continue;
                                }
                                match self.eval_line(line.to_owned()) {
                                    LineResult::Value(_) => {},
                                    LineResult::Error(e) => {
                                        return LineResult::Error(format!("error in script: {}", e))
                                    },
                                    LineResult::Success => {}
                                };
                            }
                            LineResult::Success
                        },
                        ":rvar" => {
                            match self.vars.remove(param.trim()) {
                                Some(_) => LineResult::Success,
                                None => LineResult::Error(format!("var {} not found", param))
                            }
                        },
                        ":rfn" => {
                            match self.functions.remove(param.trim()) {
                                Some(_) => LineResult::Success,
                                None => LineResult::Error(format!("function {} not found", param))
                            }
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

    fn fn_cmd(&mut self, param: String) -> LineResult {
        let split_index = match param.find("=") {
            Some(v) => v,
            None => return LineResult::Error(format!("no '=' found"))
        };
        let (args, line) = param.split_at(split_index);
        let formatted_line = line.replace("=", " ");
        let mut arg_vec = args.split_whitespace().map(|x| x.to_owned()).collect::<Vec<String>>();
        if arg_vec.len() < 2 {
            return LineResult::Error(format!("no arguments or name in function definition"))
        }
        let func_name = arg_vec.get(0).unwrap().clone();
        if func_name.starts_with(char::is_uppercase) {
            return LineResult::Error(format!("function names should start with a lowercase character"))
        }
        if func_name.ends_with('!') {
            return LineResult::Error(format!("functions cannot end with '!'"));
        }
        arg_vec.remove(0);
        let func = match Function::new(arg_vec, formatted_line){
            Ok(v) => v,
            Err(e) => return LineResult::Error(format!("function parse error: {}", e))
        };
        self.add_func(func_name.clone(), func);
        LineResult::Success
    }

    fn set_cmd(&mut self, param: String) -> LineResult {
        let split_index = match param.find(char::is_whitespace) {
            Some(v) => v,
            None => return LineResult::Error(format!("set usage: ':set [var] [expr]'"))
        };
        let (arg1, arg2) = param.split_at(split_index);
        if arg1.starts_with(char::is_lowercase) {
            return LineResult::Error(format!("variable names should start with an uppercase character"))
        }
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

fn read_file(path: String) -> Result<String, io::Error> {
    let mut result = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut result)?;
    Ok(result)
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
