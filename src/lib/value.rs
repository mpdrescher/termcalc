use std::str::FromStr;
use std::fmt::{Display, Formatter, self};

const NUMBERS: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64)
}

impl Display for Value {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            Value::Integer(v) => {
                write!(fmt, "{} [int]", v)
            },
            Value::Float(v) => {
                write!(fmt, "{} [float]", v)
            }
        }
    }
}

impl Value {
    pub fn is_int(&self) -> bool {
        match *self {
            Value::Integer(_) => true,
            _ => false
        }
    }

    pub fn is_float(&self) -> bool {
        match *self {
            Value::Float(_) => true,
            _ => false
        }
    }

    pub fn is_float_or_int(&self) -> bool {
        match *self {
            Value::Integer(_) => true,
            Value::Float(_) => true,
            //_ => false
        }
    }

    pub fn to_float(&self) -> f64 {
        match *self {
            Value::Integer(v) => v as f64,
            Value::Float(v) => v,
            //_ => panic!("to float failed")
        }
    }

    pub fn to_int(&self) -> i64 {
        match *self {
            Value::Integer(v) => v,
            Value::Float(v) => v as i64,
            //_ => panic!("to int failed")
        }
    }

    pub fn type_str(&self) -> String {
        match *self {
            Value::Integer(_) => String::from("integer"),
            Value::Float(_) => String::from("float")
        }
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        if is_int(src) {
            Ok(Value::Integer(src.parse::<i64>().unwrap()))
        }
        else {
            match src.parse::<f64>() {
                Ok(v) => Ok(Value::Float(v)),
                Err(_) => Err(())
            }
        }
    }
}

fn is_int(src: &str) -> bool {
    let mut chars = src.chars();
    let minus_check = match chars.next() {
        Some(v) => {
            if v == '-' {
                true
            }
            else if NUMBERS.contains(&v) {
                true
            }
            else {
                false
            }
        },
        None => true
    };
    minus_check && src.chars().all(|x| NUMBERS.contains(&x))
}
