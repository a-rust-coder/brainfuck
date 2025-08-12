#![feature(ascii_char)]

mod compiler;
mod interpreter;

use std::{env::args, fs, process::exit};

use crate::compiler::compiler;

fn main() {
    let args = args().collect::<Vec<String>>();

    let program = match args.get(1) {
        None => {
            eprintln!("Please provide a file.");
            exit(1);
        }
        Some(p) => match fs::read(p) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error while reading the file {}: {:?}", p, e);
                exit(1)
            }
        },
    };

    compiler(&program, "prog", true);
}
