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

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Results from scanner").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            exit(tokenize(&file_contents));
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }

    fn match_char(before: &str, c: char, code: &mut i32) -> &'static str {
        match (before, c) {
            ("=", '=') => {
                println!("EQUAL_EQUAL == null");
            }
            ("=", _) => {
                println!("EQUAL = null");
                return match_char("", c, code);
            }
            ("!", '=') => {
                println!("BANG_EQUAL != null");
            }
            ("!", _) => {
                println!("BANG ! null");
                return match_char("", c, code);
            }
            ("<", '=') => {
                println!("LESS_EQUAL <= null");
            }
            ("<", _) => {
                println!("LESS < null");
                return match_char("", c, code);
            }
            (">", '=') => {
                println!("GREATER_EQUAL >= null");
            }
            (">", _) => {
                println!("GREATER > null");
                return match_char("", c, code);
            }
            ("", '=') => return "=",
            ("", '!') => return "!",
            ("", '<') => return "<",
            ("", '>') => return ">",
            ("", '(') => {
                println!("LEFT_PAREN ( null");
            }
            ("", ')') => {
                println!("RIGHT_PAREN ) null");
            }
            ("", '{') => {
                println!("LEFT_BRACE {{ null");
            }
            ("", '}') => {
                println!("RIGHT_BRACE }} null");
            }
            ("", ',') => {
                println!("COMMA , null");
            }
            ("", '.') => {
                println!("DOT . null");
            }
            ("", '-') => {
                println!("MINUS - null");
            }
            ("", '+') => {
                println!("PLUS + null");
            }
            ("", ';') => {
                println!("SEMICOLON ; null");
            }
            ("", '*') => {
                println!("STAR * null");
            }
            ("", '/') => {
                println!("SLASH / null");
            }
            _ => {
                eprintln!("[line 1] Error: Unexpected character: {c}");
                *code = 65;
            }
        }
        ""
    }

    fn tokenize(file_contents: &String) -> i32 {
        let mut code = 0;
        let mut before = "";
        for c in file_contents.chars() {
            before = match_char(before, c, &mut code);
        }
        match before {
            "=" => println!("EQUAL = null"),
            "!" => println!("BANG ! null"),
            "<" => println!("LESS ! null"),
            ">" => println!("GREATER ! null"),
            _ => {}
        }
        println!("EOF  null");
        code
    }
}
