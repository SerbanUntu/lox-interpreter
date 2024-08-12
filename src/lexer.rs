use core::fmt;
use std::iter;

#[derive(Debug, PartialEq)]
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
    Comment,

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
            TokenVariant::Comment => write!(f, "COMMENT"),
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

#[derive(Debug, PartialEq)]
pub struct Token {
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

impl From<&str> for Token {
    fn from(literal: &str) -> Self {
        Self {
            variant: match literal {
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
                "//" => TokenVariant::Comment,
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
                s if s.starts_with("\"") && s.ends_with("\"") && s.len() > 1 => {
                    TokenVariant::String(literal.trim_matches('\"').to_owned())
                }
                s if s.parse::<f64>().is_ok() => TokenVariant::Number(s.parse().unwrap()),
                _ => TokenVariant::Identifier,
            },
            lexeme: literal.to_owned(),
        }
    }
}

impl From<TokenVariant> for Token {
    fn from(variant: TokenVariant) -> Self {
        Self {
            variant,
            lexeme: "".to_owned(),
        }
    }
}

#[derive(Debug)]
enum LexicalErrorVariant {
    UnexpectedCharacter(char),
    UnterminatedString,
}

impl fmt::Display for LexicalErrorVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexicalErrorVariant::UnexpectedCharacter(c) => write!(f, "Unexpected character: {c}"),
            LexicalErrorVariant::UnterminatedString => write!(f, "Unterminated string."),
        }
    }
}

#[derive(Debug)]
pub struct LexicalError {
    variant: LexicalErrorVariant,
    line: u32,
}

impl LexicalError {
    fn new(variant: LexicalErrorVariant, line: u32) -> Self {
        Self { variant, line }
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.variant)
    }
}

fn process_char(buf: &mut String, c: Option<char>) -> (Vec<Token>, Vec<LexicalErrorVariant>) {
    let mut tokens: Vec<Token> = Vec::new();
    let mut errors: Vec<LexicalErrorVariant> = Vec::new();
    let joined = format!("{}{}", buf.as_str(), c.unwrap_or('\0'));
    let try_token = Token::from(joined.as_str());
    match try_token.variant {
        TokenVariant::Bang
        | TokenVariant::Equal
        | TokenVariant::Greater
        | TokenVariant::Less
        | TokenVariant::Slash => {
            buf.push(c.unwrap());
        }
        TokenVariant::Number(_) | TokenVariant::Identifier | TokenVariant::Eof => {
            match (buf.as_str(), c) {
                (s, _) if matches!(s, "=" | "!" | "<" | ">" | "/") => {
                    tokens.push(Token::from(s));
                    buf.clear();
                    let (mut t, mut e) = process_char(buf, c);
                    tokens.append(&mut t);
                    errors.append(&mut e);
                }
                ("", Some(c)) if matches!(c, '\"' | '1'..='9') => {
                    buf.push(c);
                }
                (s, Some(any_char)) if s.starts_with('\"') => {
                    buf.push(any_char);
                }
                (s, Some('.')) if s.parse::<u64>().is_ok() => {
                    buf.push('.');
                }
                (s, Some(digit)) if s.parse::<f64>().is_ok() && digit.is_digit(10) => {
                    buf.push(digit);
                }
                (s, _) if s.parse::<f64>().is_ok() => {
                    tokens.push(Token::from(s.trim_matches('.')));
                    if s.ends_with('.') {
                        tokens.push(Token::from("."));
                    }
                    buf.clear();
                    let (mut t, mut e) = process_char(buf, c);
                    tokens.append(&mut t);
                    errors.append(&mut e);
                }
                (_, Some(c)) if c.is_alphanumeric() || c == '_' => {
                    buf.push(c);
                }
                (s, _) if !s.is_empty() && !s.starts_with('\"') => {
                    tokens.push(Token::from(s));
                    buf.clear();
                    let (mut t, mut e) = process_char(buf, c);
                    tokens.append(&mut t);
                    errors.append(&mut e);
                }
                (s, None) if s.starts_with("\"") => {
                    errors.push(LexicalErrorVariant::UnterminatedString);
                    buf.clear();
                }
                (_, Some(whitespace)) if whitespace.is_whitespace() => {}
                (_, Some(unexpected)) => {
                    errors.push(LexicalErrorVariant::UnexpectedCharacter(unexpected));
                    buf.clear();
                }
                (_, None) => {}
            }
        }
        _ => {
            buf.clear();
            tokens.push(try_token);
        }
    }
    (tokens, errors)
}

pub fn tokenize(file_contents: &String) -> (Vec<Token>, Option<Vec<LexicalError>>) {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut buf = String::new();
    let mut is_comment = false;
    let mut current_line = 1;
    for c in file_contents
        .chars()
        .map(|x| Some(x))
        .chain(iter::once(None))
    {
        match (c, is_comment) {
            (Some('\n'), _) => {
                is_comment = false;
                current_line += 1;
            }
            (_, false) => {
                let (t, e) = process_char(&mut buf, c);
                for token in t {
                    if token.variant == TokenVariant::Comment {
                        is_comment = true;
                    } else {
                        tokens.push(token);
                    }
                }
                for error in e {
                    errors.push(LexicalError::new(error, current_line));
                }
            }
            _ => {}
        }
    }
    tokens.push(Token::from(TokenVariant::Eof));
    (tokens, (!errors.is_empty()).then(|| errors))
}
