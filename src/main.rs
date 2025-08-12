#![feature(ascii_char)]

mod compiler_linux_x86_64;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use compiler_linux_x86_64::compiler;

mod interpreter;

use std::{env::args, fs::read, process::exit};

use crate::interpreter::interpreter;

fn main() {
    let args = args().collect::<Vec<String>>();

    match args.get(1) {
        None => eprintln!("{HELP}"),
        Some(s) => match &s[..] {
            "build" => compile(false, args),
            "run" => compile(true, args),
            "interpret" => interpreter(&read_file(args.get(2))),
            "help" => println!("{HELP}"),
            other => {
                eprintln!("Unknown subcommand {other}\n\n{HELP}");
                exit(1)
            }
        },
    }
}

fn compile(execute: bool, args: Vec<String>) {
    let program = read_file(args.get(2));
    let mut output = args.get(2).unwrap().clone();

    if output.ends_with(".bf") {
        output = output[..output.len() - 3].to_string();
    } else {
        output += ".o";
    }

    for (i, arg) in args.iter().enumerate() {
        if ["-o", "--output"].contains(&&arg[..]) {
            match args.get(i + 1) {
                None => {
                    eprintln!("Please provide a filename after the --output option.\n\n{HELP}");
                    exit(1)
                }
                Some(f) => output = f.clone(),
            }
        }
    }

    if !output.starts_with("/") && !output.starts_with("~") {
        output = "./".to_owned() + &output;
    }

    compiler(&program, &output, execute);
}

fn read_file(file: Option<&String>) -> Vec<u8> {
    match file {
        None => {
            eprintln!("Please provide a file in input.\n\n{HELP}");
            exit(1)
        }
        Some(f) => match read(f) {
            Err(_) => {
                eprintln!("Error reading file {f}.\n\n{HELP}");
                exit(1)
            }
            Ok(content) => content,
        },
    }
}

const HELP: &str = "Usage: brainfuck <SUBCOMMAND> <FILE> <OPTIONS>

Available subcommands:
  
  build          Build the BrainFuck code if tha target is supported.
                 See the doc for more information about the needed 
                 tools.

  run            Build the code and execute it.

  interpret      Interpret the code instead of building it. Works 
                 everywhere, but may be slower than compilation and
                 will always need the brainfuck executable.

  help           Display this help.

Available options:

  --output -o    With build, specify the output binary file.
";
