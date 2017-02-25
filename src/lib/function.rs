use tokenize::Token;
use tokenize;
use interpreter;

#[derive(Debug)]
pub struct Function {
    args: Vec<String>,
    code: Vec<Token>
}

impl Function {
    pub fn new(args: Vec<String>, line: String) -> Result<Function, String> {
        let code = interpreter::rearrange(tokenize::tokenize(line))?;
        Ok(Function {
            args: args,
            code: code
        })
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn arg_count(&self) -> usize {
        self.args.len()
    }

    pub fn code(&self) -> Vec<Token> {
        self.code.clone()
    }
}
