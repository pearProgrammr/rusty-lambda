#[macro_use]
extern crate nom;
extern crate regex;

mod ast;
mod parse;
mod type_check;
mod value;
mod eval;

fn main() {
    println!("Hello, world!");
}
