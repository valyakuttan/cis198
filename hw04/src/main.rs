#![feature(plugin)]
#![plugin(clippy)]

extern crate rand;

pub mod parser;
pub mod rpn;

use parser::read_eval_print_loop;

fn main() {
    if let Err(err) = read_eval_print_loop() {
        println!("Error: {:?}", err);
    }
}
