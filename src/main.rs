#[macro_use]
extern crate nom;
extern crate regex;

mod ast;
mod eval;
mod parse;
mod type_check;
mod value;

fn main() {
    println!("Hello, world!");
}
