use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, BufWriter},
    rc::Rc,
};

use crate::{
    env::{Closure, Env},
    error::Error,
    util::intersperse,
};

pub static QUOTE: &str = "quote";
pub static TRUE: &str = "#t";
pub static FALSE: &str = "#f";

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IOFunc {
    Apply,
    MakeReadPort,
    MakeWritePort,
    ClosePort,
    Read,
    Write,
    ReadContents,
    ReadAll,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Atom(String),
    List(Vec<Value>),
    DottedList(Vec<Value>, Box<Value>),
    Number(i64),
    String(String),
    Bool(bool),
    PrimitiveFunc(fn(Vec<Value>) -> Result<Value>),
    Func {
        params: Vec<String>,
        vararg: Option<String>,
        body: Vec<Value>,
        closure: Closure,
    },
    IOFunc(IOFunc),
    Port(usize),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Atom(a) => write!(f, "{}", a),
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", if *b { TRUE } else { FALSE }),
            Value::List(l) => {
                write!(f, "({})", intersperse(l))
            }
            Value::DottedList(xs, x) => {
                write!(f, "({} . {})", intersperse(xs), x)
            }
            Value::PrimitiveFunc(_) => write!(f, "<primitive>"),
            Value::Func {
                params,
                vararg,
                body: _,
                closure: _,
            } => {
                let params = intersperse(params);
                let vararg = match vararg {
                    Some(arg) => format!(" . {}", arg),
                    None => String::new(),
                };
                write!(f, "(lambda ({}{}) ...)", params, vararg)
            }
            Value::IOFunc(_) => write!(f, "<IO primitive>"),
            Value::Port(_) => write!(f, "<IO port>"),
        }
    }
}
