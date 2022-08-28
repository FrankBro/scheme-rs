use crate::{
    env::Env,
    error::Error,
    value::{Value, QUOTE},
};

type Result<T> = std::result::Result<T, Error>;

fn apply(env: &mut Env, val: &Value, args: &[Value]) -> Result<Value> {
    match val {
        Value::PrimitiveFunc(func) => func(args.to_vec()),
        Value::Func {
            params,
            vararg,
            body,
            closure,
        } => {
            if params.len() != args.len() && vararg == &None {
                return Err(Error::NumArgs(params.len(), args.to_vec()));
            }
            env.with_closure(closure);
            let mut last = 0;
            for i in 0..params.len() {
                last = i;
                let param = &params[i];
                let arg = &args[i];
                env.define_var(param.to_owned(), arg.clone());
            }
            if let Some(vararg) = vararg {
                env.define_var(vararg.to_owned(), Value::List(args[last..].to_vec()));
            }
            let mut ret = None;
            for val in body {
                ret = Some(eval(env, val)?);
            }
            ret.ok_or(Error::EmptyBody)
        }
        _ => todo!(),
    }
}

pub fn eval(env: &mut Env, val: &Value) -> Result<Value> {
    match val {
        Value::String(_) => Ok(val.clone()),
        Value::Number(_) => Ok(val.clone()),
        Value::Bool(_) => Ok(val.clone()),
        Value::Atom(id) => env.get_var(id).cloned(),
        Value::List(vals) => match &vals[..] {
            [Value::Atom(atom), val] if atom == QUOTE => Ok(val.clone()),
            [Value::Atom(atom), pred, conseq, alt] if atom == "if" => {
                let result = eval(env, pred)?;
                match result {
                    Value::Bool(false) => eval(env, alt),
                    _ => eval(env, conseq),
                }
            }
            [Value::Atom(atom), Value::Atom(var), form] if atom == "set!" => {
                let val = eval(env, form)?;
                env.set_var(var, val)
            }
            [Value::Atom(atom), Value::Atom(var), form] if atom == "define" => {
                let val = eval(env, form)?;
                Ok(env.define_var(var.clone(), val))
            }
            [Value::Atom(atom), Value::List(name_args), body @ ..] if atom == "define" => {
                let (name, args) = match &name_args[..] {
                    [Value::Atom(name), args @ ..] => (name.clone(), args.to_vec()),
                    _ => {
                        return Err(Error::BadSpecialForm(
                            "unrecognized special form".to_owned(),
                            val.clone(),
                        ));
                    }
                };
                let closure = env.make_closure();
                let params = args.into_iter().map(|arg| arg.to_string()).collect();
                let vararg = None;
                let body = body.to_vec();
                let func = Value::Func {
                    params,
                    vararg,
                    body,
                    closure,
                };
                Ok(env.define_var(name, func))
            }
            [Value::Atom(atom), Value::DottedList(name_args, vararg), body @ ..]
                if atom == "define" =>
            {
                let (name, args) = match &name_args[..] {
                    [Value::Atom(name), args @ ..] => (name.clone(), args.to_vec()),
                    _ => {
                        return Err(Error::BadSpecialForm(
                            "unrecognized special form".to_owned(),
                            val.clone(),
                        ));
                    }
                };
                let closure = env.make_closure();
                let params = args.into_iter().map(|arg| arg.to_string()).collect();
                let vararg = Some(vararg.clone().to_string());
                let body = body.to_vec();
                let func = Value::Func {
                    params,
                    vararg,
                    body,
                    closure,
                };
                Ok(env.define_var(name, func))
            }
            [Value::Atom(atom), Value::List(params), body @ ..] if atom == "lambda" => {
                let closure = env.make_closure();
                let params = params.iter().map(|param| param.to_string()).collect();
                let vararg = None;
                let body = body.to_vec();
                Ok(Value::Func {
                    params,
                    vararg,
                    body,
                    closure,
                })
            }
            [Value::Atom(atom), Value::DottedList(params, vararg), body @ ..]
                if atom == "lambda" =>
            {
                let closure = env.make_closure();
                let params = params.iter().map(|param| param.to_string()).collect();
                let vararg = Some(vararg.clone().to_string());
                let body = body.to_vec();
                Ok(Value::Func {
                    params,
                    vararg,
                    body,
                    closure,
                })
            }
            [Value::Atom(atom), Value::Atom(vararg), body @ ..] if atom == "lambda" => {
                let closure = env.make_closure();
                let params = Vec::new();
                let vararg = Some(vararg.clone());
                let body = body.to_vec();
                Ok(Value::Func {
                    params,
                    vararg,
                    body,
                    closure,
                })
            }
            [func, args @ ..] => {
                let func = eval(env, func)?;
                let args = args
                    .iter()
                    .map(|arg| eval(env, arg))
                    .collect::<Result<Vec<_>>>()?;
                let closure = env.make_closure();
                let ret = apply(env, &func, &args);
                env.load_closure(closure);
                ret
            }
            _ => Err(Error::BadSpecialForm(
                "unrecognized special form".to_owned(),
                val.clone(),
            )),
        },
        Value::DottedList(_, _) => todo!(),
        Value::PrimitiveFunc(_) => todo!(),
        Value::Func {
            params,
            vararg,
            body,
            closure,
        } => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{eval::Env, parser::parse, value::Value};

    use super::Error;

    #[test]
    fn eval() {
        let cases = vec![
            ("'atom", Ok("atom")),
            ("2", Ok("2")),
            ("\"a string\"", Ok("\"a string\"")),
            ("(+ 2 2)", Ok("4")),
            ("(+ 2 (- 4 1))", Ok("5")),
            ("(- (+ 4 6 3) 3 5 2)", Ok("3")),
            ("(< 2 3)", Ok("#t")),
            ("(> 2 3)", Ok("#f")),
            ("(>= 3 3)", Ok("#t")),
            ("(string=? \"test\" \"test\")", Ok("#t")),
            ("(string<? \"abc\" \"bba\")", Ok("#t")),
            ("(if (> 2 3) \"no\" \"yes\")", Ok("\"yes\"")),
            ("(if (= 3 3) (+ 2 3 (- 5 1)) \"unequal\")", Ok("9")),
            ("(cdr '(a simple test))", Ok("(simple test)")),
            ("(car (cdr '(a simple test)))", Ok("simple")),
            ("(car '((this is) a test))", Ok("(this is)")),
            ("(cons '(this is) 'test)", Ok("((this is) . test)")),
            ("(cons '(this is) '())", Ok("((this is))")),
            ("(eqv? 1 3)", Ok("#f")),
            ("(eqv? 3 3)", Ok("#t")),
            ("(eqv? 'atom 'atom)", Ok("#t")),
            ("(define x 3)", Ok("3")),
            ("(+ x 2)", Ok("5")),
            (
                "(+ y 2)",
                Err(Error::UnboundVar(
                    "Getting an unbound variable".to_owned(),
                    "y".to_owned(),
                )),
            ),
            ("(define y 5)", Ok("5")),
            ("(+ x (- y 2))", Ok("6")),
            ("(define str \"A string\")", Ok("\"A string\"")),
            (
                "(< str \"The string\")",
                Err(Error::TypeMismatch(
                    "number".to_owned(),
                    Value::String("A string".to_owned()),
                )),
            ),
            ("(string<? str \"The string\")", Ok("#t")),
            ("(define (f x y) (+ x y))", Ok("(lambda (x y) ...)")),
            ("(f 1 2)", Ok("3")),
            (
                "(f 1 2 3)",
                Err(Error::NumArgs(
                    2,
                    vec![Value::Number(1), Value::Number(2), Value::Number(3)],
                )),
            ),
            ("(f 1)", Err(Error::NumArgs(2, vec![Value::Number(1)]))),
            (
                "(define (factorial x) (if (= x 1) 1 (* x (factorial (- x 1)))))",
                Ok("(lambda (x) ...)"),
            ),
            ("(factorial 10)", Ok("3628800")),
            (
                "(define (counter inc) (lambda (x) (set! inc (+ x inc)) inc))",
                Ok("(lambda (inc) ...)"),
            ),
            ("(define my-count (counter 5))", Ok("(lambda (x) ...)")),
            ("(my-count 3)", Ok("8")),
            ("(my-count 6)", Ok("14")),
            ("(my-count 5)", Ok("19")),
        ];
        let mut env = Env::primitive_bindings();
        for (input, expected) in cases {
            let val = parse(input).unwrap();
            let actual = super::eval(&mut env, &val).map(|val| val.to_string());
            let expected = expected.map(|str| str.to_owned());
            assert_eq!(expected, actual,);
        }
    }
}
