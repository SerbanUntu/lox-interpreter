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

    fn token_to_string(token: &str) -> String {
        let name = match token {
            "=" => "EQUAL",
            "==" => "EQUAL_EQUAL",
            "!" => "BANG",
            "!=" => "BANG_EQUAL",
            "<" => "LESS",
            ">" => "GREATER",
            "<=" => "LESS_EQUAL",
            ">=" => "GREATER_EQUAL",
            "(" => "LEFT_PAREN",
            ")" => "RIGHT_PAREN",
            "{" => "LEFT_BRACE",
            "}" => "RIGHT_BRACE",
            "." => "DOT",
            "," => "COMMA",
            ";" => "SEMICOLON",
            "+" => "PLUS",
            "-" => "MINUS",
            "*" => "STAR",
            "/" => "SLASH",
            _ => panic!("Invalid token!"),
        };
        format!("{name} {token} null")
    }

    fn match_char(
        before: &mut String,
        c: char,
        code: &mut i32,
        is_comment: &mut bool,
        current_line: &mut u32,
    ) {
        match (before.as_str(), c) {
            ("/", '/') => {
                *is_comment = true;
                *before = String::new();
            }
            (_, '\n') => {
                *is_comment = false;
                *current_line += 1;
            }
            ("=", '=') | ("!", '=') | ("<", '=') | (">", '=') => {
                before.push_str(String::from(c).as_str());
                println!("{}", token_to_string(before.as_str()));
                *before = String::new();
            }
            ("", '(')
            | ("", ')')
            | ("", '{')
            | ("", '}')
            | ("", ',')
            | ("", '.')
            | ("", '-')
            | ("", '+')
            | ("", ';')
            | ("", '*') => {
                println!("{}", token_to_string(String::from(c).as_str()));
                *before = String::new();
            }
            ("", '0') => {
                println!("NUMBER 0 0.0");
            }
            ("=", _) | ("!", _) | ("<", _) | (">", _) | ("/", _) => {
                println!("{}", token_to_string(&before));
                *before = String::new();
                match_char(before, c, code, is_comment, current_line);
            }
            ("", '=') | ("", '!') | ("", '<') | ("", '>') | ("", '/') | ("", '\"') => {
                *before = String::from(c);
            }
            (s, '"') if s.starts_with("\"") => {
                println!("STRING \"{0}\" {0}", before.trim_matches('"'));
                *before = String::new();
            }
            (s, c) if s.starts_with("\"") => {
                before.push_str(String::from(c).as_str());
            }
            ("", c) if ('1'..='9').contains(&c) => {
                *before = String::from(c);
            }
            (s, '.')
                if s.len() > 0
                    && ('1'..='9').contains(&s.chars().nth(0).unwrap())
                    && !s.contains('.') =>
            {
                before.push_str(".");
            }
            (s, c)
                if s.len() > 0
                    && ('1'..='9').contains(&s.chars().nth(0).unwrap())
                    && ('0'..='9').contains(&c)
                    && s.ends_with('.') =>
            {
                before.push_str(String::from(c).as_str());
            }
            (s, c)
                if s.len() > 0
                    && ('1'..='9').contains(&s.chars().nth(0).unwrap())
                    && s.ends_with('.') =>
            {
                println!("NUMBER {0} {0}.0", s.trim_matches('.'));
                println!("{}", token_to_string("."));
                *before = String::new();
                match_char(before, c, code, is_comment, current_line);
            }
            (s, c)
                if ('0'..='9').contains(&c)
                    && s.len() > 0
                    && ('1'..='9').contains(&s.chars().nth(0).unwrap()) =>
            {
                before.push_str(String::from(c).as_str());
            }
            (s, c) if s.len() > 0 && ('1'..='9').contains(&s.chars().nth(0).unwrap()) => {
                let mut new_s = String::from(s);
                if !s.contains('.') {
                    new_s.push_str(".0");
                }
                while new_s.contains('.') && new_s.ends_with("00") {
                    new_s.pop();
                }
                if new_s.contains('.')
                    && new_s.ends_with("0")
                    && ('1'..='9').contains(&new_s.chars().nth(new_s.len() - 2).unwrap())
                {
                    new_s.pop();
                }
                println!("NUMBER {} {}", s, new_s);
                *before = String::new();
                match_char(before, c, code, is_comment, current_line);
            }
            (_, c) if c.is_alphanumeric() || c == '_' => {
                before.push_str(String::from(c).as_str());
            }
            (s, _) if s.len() > 0 => {
                println!("IDENTIFIER {} null", s);
                *before = String::new();
                match_char(before, c, code, is_comment, current_line);
            }
            (_, w) if w.is_whitespace() => {}
            _ => {
                eprintln!("[line {0}] Error: Unexpected character: {c}", *current_line);
                *code = 65;
                *before = String::new();
            }
        }
    }

    fn tokenize(file_contents: &String) -> i32 {
        let mut code = 0;
        let mut before = String::new();
        let mut is_comment = false;
        let mut current_line = 1;
        for c in file_contents.chars() {
            match (c, is_comment) {
                ('\n', _) => {
                    is_comment = false;
                    current_line += 1;
                }
                (_, false) => {
                    match_char(
                        &mut before,
                        c,
                        &mut code,
                        &mut is_comment,
                        &mut current_line,
                    );
                }
                _ => {}
            }
        }
        if before.len() > 0 {
            match before.chars().nth(0).unwrap() {
                '\"' => {
                    eprintln!("[line {0}] Error: Unterminated string.", current_line);
                    code = 65;
                }
                '1'..='9' => {
                    if before.ends_with('.') {
                        println!("NUMBER {0} {0}.0", before.trim_matches('.'));
                        println!("{}", token_to_string("."));
                    } else {
                        let mut new_s = before.clone();
                        if !before.contains('.') {
                            new_s.push_str(".0");
                        }
                        while new_s.contains('.') && new_s.ends_with("00") {
                            new_s.pop();
                        }
                        if new_s.contains('.')
                            && new_s.ends_with("0")
                            && ('1'..='9').contains(&new_s.chars().nth(new_s.len() - 2).unwrap())
                        {
                            new_s.pop();
                        }
                        println!("NUMBER {} {}", before, new_s);
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    println!("IDENTIFIER {} null", before);
                }
                _ => {
                    println!("{}", token_to_string(&before));
                }
            }
        }
        println!("EOF  null");
        code
    }
}
