use std::collections::HashMap;

use crate::{error::Error, primitive, value::Value};

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Env {
    vars: HashMap<String, Value>,
}

impl Env {
    pub fn get_var(&self, var: &str) -> Result<&Value> {
        self.vars.get(var).ok_or_else(|| {
            Error::UnboundVar("Getting an unbound variable".to_owned(), var.to_owned())
        })
    }

    pub fn set_var(&mut self, var: &str, val: Value) -> Result<Value> {
        match self.vars.get_mut(var) {
            Some(prev) => {
                *prev = val.clone();
                Ok(val)
            }
            None => Err(Error::UnboundVar(
                "Setting an unbound var".to_owned(),
                var.to_owned(),
            )),
        }
    }

    pub fn define_var(&mut self, var: String, val: Value) -> Value {
        self.vars.insert(var, val.clone());
        val
    }

    pub fn primitive_bindings() -> Self {
        let mut env = Env::default();
        fn define_primitive_func(env: &mut Env, name: &str, func: fn(Vec<Value>) -> Result<Value>) {
            env.define_var(name.to_owned(), Value::PrimitiveFunc(func));
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::numeric_binop(&vals, |acc, val| acc + val)
            }
            define_primitive_func(&mut env, "+", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::numeric_binop(&vals, |acc, val| acc - val)
            }
            define_primitive_func(&mut env, "-", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::numeric_binop(&vals, |acc, val| acc * val)
            }
            define_primitive_func(&mut env, "*", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::numeric_binop(&vals, |acc, val| acc / val)
            }
            define_primitive_func(&mut env, "/", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::numeric_binop(&vals, |acc, val| acc % val)
            }
            define_primitive_func(&mut env, "mod", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::numeric_binop(&vals, |acc, val| acc / val)
            }
            define_primitive_func(&mut env, "quotient", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::numeric_binop(&vals, |acc, val| acc % val)
            }
            define_primitive_func(&mut env, "remainder", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::numeric_bool_binop(&vals, |lhs, rhs| lhs == rhs)
            }
            define_primitive_func(&mut env, "=", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::numeric_bool_binop(&vals, |lhs, rhs| lhs < rhs)
            }
            define_primitive_func(&mut env, "<", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::numeric_bool_binop(&vals, |lhs, rhs| lhs > rhs)
            }
            define_primitive_func(&mut env, ">", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::numeric_bool_binop(&vals, |lhs, rhs| lhs != rhs)
            }
            define_primitive_func(&mut env, "/=", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::numeric_bool_binop(&vals, |lhs, rhs| lhs >= rhs)
            }
            define_primitive_func(&mut env, ">=", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::numeric_bool_binop(&vals, |lhs, rhs| lhs <= rhs)
            }
            define_primitive_func(&mut env, "<=", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::bool_bool_binop(&vals, |lhs, rhs| lhs && rhs)
            }
            define_primitive_func(&mut env, "&&", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::bool_bool_binop(&vals, |lhs, rhs| lhs || rhs)
            }
            define_primitive_func(&mut env, "||", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::string_bool_binop(&vals, |lhs, rhs| lhs == rhs)
            }
            define_primitive_func(&mut env, "string=?", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::string_bool_binop(&vals, |lhs, rhs| lhs < rhs)
            }
            define_primitive_func(&mut env, "string<?", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::string_bool_binop(&vals, |lhs, rhs| lhs > rhs)
            }
            define_primitive_func(&mut env, "string>?", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::string_bool_binop(&vals, |lhs, rhs| lhs <= rhs)
            }
            define_primitive_func(&mut env, "string<=?", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::string_bool_binop(&vals, |lhs, rhs| lhs >= rhs)
            }
            define_primitive_func(&mut env, "string>=?", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::car(&vals)
            }
            define_primitive_func(&mut env, "car", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::cdr(&vals)
            }
            define_primitive_func(&mut env, "cdr", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::cons(&vals)
            }
            define_primitive_func(&mut env, "cons", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::eqv(&vals)
            }
            define_primitive_func(&mut env, "eq?", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::eqv(&vals)
            }
            define_primitive_func(&mut env, "eqv?", func);
        }
        {
            fn func(vals: Vec<Value>) -> Result<Value> {
                primitive::equal(&vals)
            }
            define_primitive_func(&mut env, "equal?", func);
        }
        env
    }
}
