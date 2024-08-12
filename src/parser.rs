use crate::lexer::{Token, TokenVariant};

fn get_number_from_token(token: &Token) -> Option<f64> {
    if let TokenVariant::Number(x) = token.variant {
        Some(x)
    } else {
        None
    }
}

pub fn parse(mut tokens: Vec<Token>) {
    tokens.pop(); // Removes EOF
    if tokens.len() == 1 {
        println!("{}", tokens[0].lexeme);
    }
    if tokens.len() == 3 {
        println!(
            "({} {:?} {:?})",
            tokens[1].lexeme,
            get_number_from_token(&tokens[0]).unwrap(),
            get_number_from_token(&tokens[2]).unwrap()
        )
    }
}
