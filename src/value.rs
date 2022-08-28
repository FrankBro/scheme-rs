use std::fmt::Display;

use crate::{env::Env, error::Error, parser, util::intersperse};

pub static QUOTE: &str = "quote";
pub static TRUE: &str = "#t";
pub static FALSE: &str = "#f";

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Atom(String),
    List(Vec<Value>),
    DottedList(Vec<Value>, Box<Value>),
    Number(i64),
    String(String),
    Bool(bool),
    PrimitiveFunc(fn(Vec<Value>) -> Result<Value>),
    // PrimitiveFunc(fn(&'a [Value]) -> Result<Value>),
    Func {
        params: Vec<String>,
        vararg: Option<String>,
        body: Vec<Value>,
        closure: Env,
    },
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
        }
    }
}

/*
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Atom(l0), Self::Atom(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::DottedList(l0, l1), Self::DottedList(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::PrimitiveFunc(l0), Self::PrimitiveFunc(r0)) => l0 as usize == r0 as usize,
            (
                Self::Func {
                    params: l_params,
                    vararg: l_vararg,
                    body: l_body,
                    closure: l_closure,
                },
                Self::Func {
                    params: r_params,
                    vararg: r_vararg,
                    body: r_body,
                    closure: r_closure,
                },
            ) => {
                l_params == r_params
                    && l_vararg == r_vararg
                    && l_body == r_body
                    && l_closure == r_closure
            }
        }
    }
}
*/
