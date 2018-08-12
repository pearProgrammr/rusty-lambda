#[macro_use]
extern crate nom;
extern crate regex;

mod ast;
mod eval;
mod parse;
mod type_check;
mod value;

use eval::eval;
use parse::parse_file;
use std::collections::HashMap;
use std::io;
use type_check::type_check;

fn err_str<E>(msg: E) -> io::Error
where
    E: Into<Box<std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::InvalidInput, msg)
}

fn main() -> io::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or_else(|| err_str("No filename argument provided"))?;
    let mut handle: Box<io::Read> = match filename.as_str() {
        "-" => Box::new(std::io::stdin()),
        _ => Box::new(std::fs::File::open(filename)?),
    };
    let mut contents = String::new();
    let _len = handle.read_to_string(&mut contents)?;
    let terms = parse_file(&contents).map_err(err_str)?;

    let mut type_env = type_check::TyEnv(HashMap::new());
    for term in &terms {
        match term {
            ast::Term::Assignm { var_name, expr } => {
                let expr_type = type_check(expr, &type_env).map_err(err_str)?;
                type_env.0.insert(var_name.to_string(), expr_type);
            }
            _ => {
                type_check(term, &type_env).map_err(err_str)?;
            }
        };
    }

    let mut eval_env = eval::EvalEnv(HashMap::new());
    for term in &terms {
        let val = eval(term, &eval_env).map_err(err_str)?;
        match val {
            value::Value::Assignm { name, val } => {
                eval_env.0.insert(name, *val);
            }
            _ => println!("{:?}", val),
        }
    }

    Ok(())
}
