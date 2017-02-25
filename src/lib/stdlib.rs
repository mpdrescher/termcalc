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
        "eq" => equals(valstack),
        "lt" => lesserthan(valstack),
        "gt" => greaterthan(valstack),
        "sin" => sin(valstack),
        "cos" => cos(valstack),
        "tan" => tan(valstack),
        "sinh" => sinh(valstack),
        "cosh" => cosh(valstack),
        "tanh" => tanh(valstack),
        "abs" => abs(valstack),
        "floor" => floor(valstack),
        "ceil" => ceil(valstack),
        "if" => fnif(valstack),
        _ => Err(format!("unknown function: {}", name))
    }
}

//FUNCTIONS

fn floor(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_one("floor", valstack)?;
    let p1 = pop!(valstack);
    if p1.is_int() {
        Ok(p1)
    }
    else if p1.is_float() {
        let p1val = p1.to_float();
        Ok(Value::Float(p1val.floor()))
    }
    else {
        throw_invalid_type!("floor", &p1.type_str())
    }
}

fn ceil(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_one("ceil", valstack)?;
    let p1 = pop!(valstack);
    if p1.is_int() {
        Ok(p1)
    }
    else if p1.is_float() {
        let p1val = p1.to_float();
        Ok(Value::Float(p1val.ceil()))
    }
    else {
        throw_invalid_type!("ceil", &p1.type_str())
    }
}

fn abs(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_one("abs", valstack)?;
    let p1 = pop!(valstack);
    if p1.is_int() {
        let p1val = p1.to_int();
        Ok(Value::Integer(p1val.abs()))
    }
    else if p1.is_float() {
        let p1val = p1.to_float();
        Ok(Value::Float(p1val.abs()))
    }
    else {
        throw_invalid_type!("abs", &p1.type_str())
    }
}

fn sin(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_one("sin", valstack)?;
    let p1 = pop!(valstack);
    if p1.is_float_or_int() {
        let p1val = p1.to_float();
        Ok(Value::Float(p1val.sin()))
    }
    else {
        throw_invalid_type!("sin", &p1.type_str())
    }
}

fn sinh(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_one("sinh", valstack)?;
    let p1 = pop!(valstack);
    if p1.is_float_or_int() {
        let p1val = p1.to_float();
        Ok(Value::Float(p1val.sinh()))
    }
    else {
        throw_invalid_type!("sinh", &p1.type_str())
    }
}

fn cos(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_one("cos", valstack)?;
    let p1 = pop!(valstack);
    if p1.is_float_or_int() {
        let p1val = p1.to_float();
        Ok(Value::Float(p1val.cos()))
    }
    else {
        throw_invalid_type!("cos", &p1.type_str())
    }
}

fn cosh(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_one("cosh", valstack)?;
    let p1 = pop!(valstack);
    if p1.is_float_or_int() {
        let p1val = p1.to_float();
        Ok(Value::Float(p1val.cosh()))
    }
    else {
        throw_invalid_type!("cosh", &p1.type_str())
    }
}

fn tan(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_one("tan", valstack)?;
    let p1 = pop!(valstack);
    if p1.is_float_or_int() {
        let p1val = p1.to_float();
        Ok(Value::Float(p1val.tan()))
    }
    else {
        throw_invalid_type!("tan", &p1.type_str())
    }
}

fn tanh(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_one("tanh", valstack)?;
    let p1 = pop!(valstack);
    if p1.is_float_or_int() {
        let p1val = p1.to_float();
        Ok(Value::Float(p1val.tanh()))
    }
    else {
        throw_invalid_type!("tanh", &p1.type_str())
    }
}

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

fn equals(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_two("equals", valstack)?;
    let p2 = pop!(valstack);
    let p1 = pop!(valstack);
    if p1.is_int() && p2.is_int() {
        let p1val = p1.to_int();
        let p2val = p2.to_int();
        if p1val == p2val {
            Ok(Value::Integer(1))
        }
        else {
            Ok(Value::Integer(0))
        }
    }
    else if p1.is_float() && p2.is_float() {
        let p1val = p1.to_float();
        let p2val = p2.to_float();
        if p1val == p2val {
            Ok(Value::Integer(1))
        }
        else {
            Ok(Value::Integer(0))
        }
    }
    else {
        throw_invalid_type!("equals", &p1.type_str(), &p2.type_str())
    }
}

fn lesserthan(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_two("lt", valstack)?;
    let p2 = pop!(valstack);
    let p1 = pop!(valstack);
    if p1.is_float_or_int() && p2.is_float_or_int() {
        let p1val = p1.to_float();
        let p2val = p2.to_float();
        if p1val < p2val {
            Ok(Value::Integer(1))
        }
        else {
            Ok(Value::Integer(0))
        }
    }
    else {
        throw_invalid_type!("lt", &p1.type_str(), &p2.type_str())
    }
}

fn greaterthan(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_two("gt", valstack)?;
    let p2 = pop!(valstack);
    let p1 = pop!(valstack);
    if p1.is_float_or_int() && p2.is_float_or_int() {
        let p1val = p1.to_float();
        let p2val = p2.to_float();
        if p1val > p2val {
            Ok(Value::Integer(1))
        }
        else {
            Ok(Value::Integer(0))
        }
    }
    else {
        throw_invalid_type!("gt", &p1.type_str(), &p2.type_str())
    }
}

fn fnif(valstack: &mut Vec<Value>) -> Result<Value, String> {
    check_n(3, "if", valstack)?;
    let p3 = pop!(valstack);
    let p2 = pop!(valstack);
    let p1 = pop!(valstack);
    if p1.is_int() {
        if p1.to_int() != 0 {
            Ok(p2)
        }
        else {
            Ok(p3)
        }
    }
    else {
        throw_invalid_type!("if", &p1.type_str(), &p2.type_str(), &p3.type_str())
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
        if p2val == 0.0 {
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

fn check_n(count: usize, fnname: &str, valstack: &mut Vec<Value>) -> Result<(), String> {
    if valstack.len() < count {
        Err(format!("expected {} arguments in function '{}'", count, fnname))
    }
    else {
        Ok(())
    }
}

fn check_two(fnname: &str, valstack: &mut Vec<Value>) -> Result<(), String> {
    check_n(2, fnname, valstack)
}

fn check_one(fnname: &str, valstack: &mut Vec<Value>) -> Result<(), String> {
    check_n(1, fnname, valstack)
}
