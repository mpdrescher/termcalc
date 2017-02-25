use value::Value;

macro_rules! pop {
    ($e:expr) => {
        $e.pop().unwrap();
    };
}

macro_rules! throw_invalid_type {
    ($e:expr, $($t:expr),+) => {
        {
            let mut message = String::new();
            message.push_str(&format!("invalid types in function '{}': ", $e));
            $ (
                message.push_str($t);
                message.push_str(" ");
            ) +
            Err(message)
        }
    };
}

pub fn match_fn(name: String, valstack: &mut Vec<Value>) -> Result<Value, String> {
    match name.as_str() {
        "add" => add(valstack),
        "sub" => sub(valstack),
        "div" => div(valstack),
        "mul" => mul(valstack),
        "neg" => negate(valstack),
        "pow" => pow(valstack),
        "sqrt" => sqrt(valstack),
        _ => Err(format!("unknown function: {}", name))
    }
}

//FUNCTIONS

fn negate(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_one("neg", valstack)?;
    let p1 = pop!(valstack);
    if p1.is_int() {
        let p1val = p1.to_int();
        Ok(Value::Integer(-p1val))
    }
    else if p1.is_float() {
        let p1val = p1.to_float();
        Ok(Value::Float(-p1val))
    }
    else {
        throw_invalid_type!("neg", &p1.type_str())
    }
}

fn sqrt(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_one("sqrt", valstack)?;
    let p1 = pop!(valstack);
    if p1.is_float_or_int() {
        let p1val = p1.to_float();
        if p1val < 0.0 {
            return Err(format!("square root of a negative number"))
        }
        Ok(Value::Float(p1val.sqrt()))
    }
    else {
        throw_invalid_type!("sqrt", &p1.type_str())
    }
}

fn add(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_two("add", valstack)?;
    let p2 = pop!(valstack);
    let p1 = pop!(valstack);
    if p1.is_int() && p2.is_int() {
        let p1val = p1.to_int();
        let p2val = p2.to_int();
        Ok(Value::Integer(p1val + p2val))
    }
    else if p1.is_float_or_int() && p2.is_float_or_int(){
        let p1val = p1.to_float();
        let p2val = p2.to_float();
        Ok(Value::Float(p1val + p2val))
    }
    else {
        throw_invalid_type!("add", &p1.type_str(), &p2.type_str())
    }
}

fn sub(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_two("sub", valstack)?;
    let p2 = pop!(valstack);
    let p1 = pop!(valstack);
    if p1.is_int() && p2.is_int() {
        let p1val = p1.to_int();
        let p2val = p2.to_int();
        Ok(Value::Integer(p1val - p2val))
    }
    else if p1.is_float_or_int() && p2.is_float_or_int(){
        let p1val = p1.to_float();
        let p2val = p2.to_float();
        Ok(Value::Float(p1val - p2val))
    }
    else {
        throw_invalid_type!("sub", &p1.type_str(), &p2.type_str())
    }
}

fn mul(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_two("mul", valstack)?;
    let p2 = pop!(valstack);
    let p1 = pop!(valstack);
    if p1.is_int() && p2.is_int() {
        let p1val = p1.to_int();
        let p2val = p2.to_int();
        Ok(Value::Integer(p1val * p2val))
    }
    else if p1.is_float_or_int() && p2.is_float_or_int(){
        let p1val = p1.to_float();
        let p2val = p2.to_float();
        Ok(Value::Float(p1val * p2val))
    }
    else {
        throw_invalid_type!("mul", &p1.type_str(), &p2.type_str())
    }
}

fn div(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_two("div", valstack)?;
    let p2 = pop!(valstack);
    let p1 = pop!(valstack);
    if p1.is_float_or_int() && p2.is_float_or_int(){
        let p1val = p1.to_float();
        let p2val = p2.to_float();
        if p1val == 0.0 {
            return Err(format!("divide by zero"));
        }
        Ok(Value::Float(p1val / p2val))
    }
    else {
        throw_invalid_type!("div", &p1.type_str(), &p2.type_str())
    }
}

fn pow(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_two("pow", valstack)?;
    let p2 = pop!(valstack);
    let p1 = pop!(valstack);
    if p1.is_int() && p2.is_int() {
        let p1val = p1.to_int();
        let p2val = p2.to_int();
        Ok(Value::Integer(p1val.pow(p2val as u32)))
    }
    else if p1.is_float() && p2.is_float(){
        let p1val = p1.to_float();
        let p2val = p2.to_float();
        Ok(Value::Float(p1val.powf(p2val)))
    }
    else if p1.is_float() && p2.is_int() {
        let p1val = p1.to_float();
        let p2val = p2.to_int();
        Ok(Value::Float(p1val.powi(p2val as i32)))
    }
    else {
        throw_invalid_type!("pow", &p1.type_str(), &p2.type_str())
    }
}

//UTILS

fn check_two(fnname: &str, valstack: &mut Vec<Value>) -> Result<(), String> {
    if valstack.len() < 2 {
        Err(format!("expected two arguments in function '{}'", fnname))
    }
    else {
        Ok(())
    }
}

fn check_one(fnname: &str, valstack: &mut Vec<Value>) -> Result<(), String> {
    if valstack.len() < 1 {
        Err(format!("expected one argument in function '{}'", fnname))
    }
    else {
        Ok(())
    }
}
