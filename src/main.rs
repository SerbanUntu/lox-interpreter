mod lexer;
mod parser;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    match command.as_str() {
        "tokenize" => {
            writeln!(io::stderr(), "Results from lexer").unwrap();
            let (tokens, errors) = lexer::tokenize(&file_contents);
            for token in &tokens {
                println!("{}", token);
            }
            if let Some(e) = errors {
                for error in e {
                    eprintln!("{}", error);
                }
                exit(65);
            }
        }
        "parse" => {
            writeln!(io::stderr(), "Results from parser").unwrap();
            let (tokens, _) = lexer::tokenize(&file_contents);
            match parser::parse(tokens) {
                Ok(abstract_syntax_tree) => {
                    if let Some(x) = abstract_syntax_tree.root {
                        println!("{}", x.borrow().to_string());
                    }
                }
                Err(e) => {
                    for error in e {
                        eprintln!("{}", error);
                    }
                    exit(65);
                }
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
