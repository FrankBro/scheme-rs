use std::fmt::Display;

use crate::{lexer::Token, util::intersperse, value::Value};

#[derive(Debug, PartialEq, Eq)]
pub enum ParserError {
    NoMoreTokens,
    UnexpectedToken(Token),
    ExpectedToken(Token, Token),
    TokensLeft(Vec<Token>),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::NoMoreTokens => write!(f, "No more tokens"),
            ParserError::UnexpectedToken(t) => write!(f, "Unexpected token: {}", t),
            ParserError::ExpectedToken(expected, found) => {
                write!(f, "Expected token {}, found {}", expected, found)
            }
            ParserError::TokensLeft(tokens) => write!(f, "Tokens left: {:?}", tokens),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NumArgs(usize, Vec<Value>),
    TypeMismatch(String, Value),
    Parser(ParserError),
    BadSpecialForm(String, Value),
    NotFunction(String, String),
    UnboundVar(String, String),
    EmptyBody, // Default(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnboundVar(msg, name) => write!(f, "{}: {}", msg, name),
            Error::BadSpecialForm(msg, form) => write!(f, "{}: {}", msg, form),
            Error::NotFunction(msg, func) => write!(f, "{}: {}", msg, func),
            Error::NumArgs(expected, found) => write!(
                f,
                "Expected {} args; found values {}",
                expected,
                intersperse(found)
            ),
            Error::TypeMismatch(expected, found) => {
                write!(f, "Invalid type: expected {}, found {}", expected, found)
            }
            Error::Parser(e) => write!(f, "Parse error at {}", e),
            Error::EmptyBody => write!(f, "Function has empty body"),
            // Error::Default(_) => todo!(),
        }
    }
}
