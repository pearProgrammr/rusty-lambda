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
use std::io::{self, Read, Write};
use type_check::type_check;

fn err_str<E>(msg: E) -> io::Error
where
    E: Into<Box<std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::InvalidInput, msg)
}

fn exec(contents: &str, output: &mut Write) -> io::Result<()> {
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
            _ => writeln!(output, "{:?}", val).or_else(|_| Err(err_str("write failed")))?,
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or_else(|| err_str("No filename argument provided"))?;
    let mut handle: Box<Read> = match filename.as_str() {
        "-" => Box::new(std::io::stdin()),
        _ => Box::new(std::fs::File::open(filename)?),
    };
    let mut contents = String::new();
    let _len = handle.read_to_string(&mut contents)?;

    exec(&contents, &mut std::io::stdout())?;

    Ok(())
}

#[cfg(test)]
fn test_script(path: &str) {
    let mut script = String::new();
    std::fs::File::open(path)
        .unwrap()
        .read_to_string(&mut script)
        .unwrap();

    let mut expected_path = std::path::PathBuf::from(path);
    expected_path.set_extension("out");

    let mut expected = Vec::new();
    std::fs::File::open(expected_path)
        .unwrap()
        .read_to_end(&mut expected)
        .unwrap();

    let mut output = Vec::new();
    exec(&script, &mut output).unwrap();

    assert_eq!(expected, output);
}

include!(concat!(env!("OUT_DIR"), "/scripts.rs"));
