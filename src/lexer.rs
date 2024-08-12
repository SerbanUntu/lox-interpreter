use core::fmt;

#[derive(Debug)]
enum TokenVariant {
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Eof,

    Number(f64),
    String(String),
    Identifier,

    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl fmt::Display for TokenVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenVariant::LeftParen => write!(f, "LEFT_PAREN"),
            TokenVariant::RightParen => write!(f, "RIGHT_PAREN"),
            TokenVariant::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenVariant::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenVariant::Comma => write!(f, "COMMA"),
            TokenVariant::Dot => write!(f, "DOT"),
            TokenVariant::Minus => write!(f, "MINUS"),
            TokenVariant::Plus => write!(f, "PLUS"),
            TokenVariant::Semicolon => write!(f, "SEMICOLON"),
            TokenVariant::Slash => write!(f, "SLASH"),
            TokenVariant::Star => write!(f, "STAR"),
            TokenVariant::Bang => write!(f, "BANG"),
            TokenVariant::BangEqual => write!(f, "BANG_EQUAL"),
            TokenVariant::Equal => write!(f, "EQUAL"),
            TokenVariant::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenVariant::Greater => write!(f, "GREATER"),
            TokenVariant::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TokenVariant::Less => write!(f, "LESS"),
            TokenVariant::LessEqual => write!(f, "LESS_EQUAL"),
            TokenVariant::Identifier => write!(f, "IDENTIFIER"),
            TokenVariant::String(_) => write!(f, "STRING"),
            TokenVariant::Number(_) => write!(f, "NUMBER"),
            TokenVariant::And => write!(f, "AND"),
            TokenVariant::Class => write!(f, "CLASS"),
            TokenVariant::Else => write!(f, "ELSE"),
            TokenVariant::False => write!(f, "FALSE"),
            TokenVariant::Fun => write!(f, "FUN"),
            TokenVariant::For => write!(f, "FOR"),
            TokenVariant::If => write!(f, "IF"),
            TokenVariant::Nil => write!(f, "NIL"),
            TokenVariant::Or => write!(f, "OR"),
            TokenVariant::Print => write!(f, "PRINT"),
            TokenVariant::Return => write!(f, "RETURN"),
            TokenVariant::Super => write!(f, "SUPER"),
            TokenVariant::This => write!(f, "THIS"),
            TokenVariant::True => write!(f, "TRUE"),
            TokenVariant::Var => write!(f, "VAR"),
            TokenVariant::While => write!(f, "WHILE"),
            TokenVariant::Eof => write!(f, "EOF"),
        }
    }
}

#[derive(Debug)]
struct Token {
    variant: TokenVariant,
    lexeme: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut value = String::from("null");
        if let TokenVariant::String(x) = &self.variant {
            value = x.clone();
        }
        if let TokenVariant::Number(x) = self.variant {
            value = format!("{:?}", x);
        }
        write!(f, "{} {} {}", self.variant, self.lexeme, value)
    }
}

impl Token {
    fn from(literal: &str) -> Self {
        let variant = match literal {
            "=" => TokenVariant::Equal,
            "==" => TokenVariant::EqualEqual,
            "!" => TokenVariant::Bang,
            "!=" => TokenVariant::BangEqual,
            "<" => TokenVariant::Less,
            ">" => TokenVariant::Greater,
            "<=" => TokenVariant::LessEqual,
            ">=" => TokenVariant::GreaterEqual,
            "(" => TokenVariant::LeftParen,
            ")" => TokenVariant::RightParen,
            "{" => TokenVariant::LeftBrace,
            "}" => TokenVariant::RightBrace,
            "." => TokenVariant::Dot,
            "," => TokenVariant::Comma,
            ";" => TokenVariant::Semicolon,
            "+" => TokenVariant::Plus,
            "-" => TokenVariant::Minus,
            "*" => TokenVariant::Star,
            "/" => TokenVariant::Slash,
            "and" => TokenVariant::And,
            "class" => TokenVariant::Class,
            "else" => TokenVariant::Else,
            "false" => TokenVariant::False,
            "fun" => TokenVariant::Fun,
            "for" => TokenVariant::For,
            "if" => TokenVariant::If,
            "nil" => TokenVariant::Nil,
            "or" => TokenVariant::Or,
            "print" => TokenVariant::Print,
            "return" => TokenVariant::Return,
            "super" => TokenVariant::Super,
            "this" => TokenVariant::This,
            "true" => TokenVariant::True,
            "var" => TokenVariant::Var,
            "while" => TokenVariant::While,
            "" => TokenVariant::Eof,
            s if s.starts_with("\"") => TokenVariant::String(literal.trim_matches('\"').to_owned()),
            s if s.len() > 0 && ('0'..='9').contains(&s.chars().nth(0).unwrap()) => {
                TokenVariant::Number(s.parse().unwrap())
            }
            _ => TokenVariant::Identifier,
        };
        Self {
            variant,
            lexeme: literal.to_owned(),
        }
    }
}

fn match_char(
    before: &mut String,
    c: Option<char>,
    code: &mut i32,
    is_comment: &mut bool,
    current_line: &mut u32,
    tokens: &mut Vec<Token>,
) {
    match (before.as_str(), c) {
        ("/", Some('/')) => {
            *is_comment = true;
            *before = String::new();
        }
        ("=", Some('=')) | ("!", Some('=')) | ("<", Some('=')) | (">", Some('=')) => {
            before.push_str(c.unwrap().encode_utf8(&mut [0; 4]));
            print_data(before.as_str());
            *before = String::new();
        }
        ("", Some('('))
        | ("", Some(')'))
        | ("", Some('{'))
        | ("", Some('}'))
        | ("", Some(','))
        | ("", Some('.'))
        | ("", Some('-'))
        | ("", Some('+'))
        | ("", Some(';'))
        | ("", Some('*'))
        | ("", Some('0')) => {
            print_data(c.unwrap().encode_utf8(&mut [0; 4]));
        }
        ("=", _) | ("!", _) | ("<", _) | (">", _) | ("/", _) => {
            print_data(&before);
            *before = String::new();
            match_char(before, c, code, is_comment, current_line, tokens);
        }
        ("", Some('='))
        | ("", Some('!'))
        | ("", Some('<'))
        | ("", Some('>'))
        | ("", Some('/'))
        | ("", Some('\"')) => {
            *before = String::from(c.unwrap());
        }
        (s, Some('"')) if s.starts_with("\"") => {
            before.push_str("\"");
            print_data(before);
            *before = String::new();
        }
        (s, Some(c)) if s.starts_with("\"") => {
            before.push_str(c.encode_utf8(&mut [0; 4]));
        }
        ("", Some(c)) if ('1'..='9').contains(&c) => {
            *before = String::from(c);
        }
        (s, Some('.'))
            if s.len() > 0
                && ('1'..='9').contains(&s.chars().nth(0).unwrap())
                && !s.contains('.') =>
        {
            before.push_str(".");
        }
        (s, Some(c))
            if s.len() > 0
                && ('1'..='9').contains(&s.chars().nth(0).unwrap())
                && ('0'..='9').contains(&c)
                && s.ends_with('.') =>
        {
            before.push_str(c.encode_utf8(&mut [0; 4]));
        }
        (s, c)
            if s.len() > 0
                && ('1'..='9').contains(&s.chars().nth(0).unwrap())
                && s.ends_with('.') =>
        {
            print_data(s.trim_matches('.'));
            print_data(".");
            *before = String::new();
            match_char(before, c, code, is_comment, current_line, tokens);
        }
        (s, Some(c))
            if ('0'..='9').contains(&c)
                && s.len() > 0
                && ('1'..='9').contains(&s.chars().nth(0).unwrap()) =>
        {
            before.push_str(c.encode_utf8(&mut [0; 4]));
        }
        (s, c) if s.len() > 0 && ('1'..='9').contains(&s.chars().nth(0).unwrap()) => {
            print_data(s);
            *before = String::new();
            match_char(before, c, code, is_comment, current_line, tokens);
        }
        (_, Some(c)) if c.is_alphanumeric() || c == '_' => {
            before.push_str(c.encode_utf8(&mut [0; 4]));
        }
        (s, _) if s.len() > 0 && !s.starts_with("\"") => {
            print_data(s);
            *before = String::new();
            match_char(before, c, code, is_comment, current_line, tokens);
        }
        (s, None) if s.starts_with("\"") => {
            eprintln!("[line {0}] Error: Unterminated string.", current_line);
            *code = 65;
            *before = String::new();
        }
        (_, Some(w)) if w.is_whitespace() => {}
        (_, None) => {}
        (_, Some(c)) => {
            eprintln!("[line {0}] Error: Unexpected character: {c}", *current_line);
            *code = 65;
            *before = String::new();
        }
    }
}

fn print_data(literal: &str) {
    //TODO change this
    println!("{}", Token::from(literal));
}

pub fn tokenize(file_contents: &String) -> i32 {
    let mut code = 0;
    let mut before = String::new();
    let mut is_comment = false;
    let mut current_line = 1;
    let mut tokens: Vec<Token> = Vec::new();
    for c in file_contents.chars() {
        match (c, is_comment) {
            ('\n', _) => {
                is_comment = false;
                current_line += 1;
            }
            (_, false) => {
                match_char(
                    &mut before,
                    Some(c),
                    &mut code,
                    &mut is_comment,
                    &mut current_line,
                    &mut tokens,
                );
            }
            _ => {}
        }
    }

    if before.len() > 0 {
        match_char(
            &mut before,
            None,
            &mut code,
            &mut is_comment,
            &mut current_line,
            &mut tokens,
        )
    }

    print_data("");
    code
}
