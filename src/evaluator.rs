use crate::lexer::{Token, TokenVariant};
use crate::parser::Tree;
use core::fmt;
use std::rc::Rc;
use RuntimeErrorVariant::*;
use TokenVariant::*;

#[derive(Debug)]
enum RuntimeErrorVariant {
    MustBeNumber,
    MustBeNumbers,
    MustBeNumbersOrStrings,
}

impl fmt::Display for RuntimeErrorVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MustBeNumber => write!(f, "Operand must be a number."),
            MustBeNumbers => write!(f, "Operands must be numbers."),
            MustBeNumbersOrStrings => write!(f, "Operands must be two numbers or two strings."),
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    variant: RuntimeErrorVariant,
    line: u32,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n[line {}]", self.variant, self.line)
    }
}

impl RuntimeError {
    fn new(variant: RuntimeErrorVariant, line: u32) -> Self {
        Self {
            variant,
            line
        }
    }
}

pub fn evaluate(ast: &mut Tree) -> Result<Token, Vec<RuntimeError>> {
    let mut final_token = Token::from((Eof, 0)); //TODO Replace with Option<Token>
    let mut errors = Vec::new();
    match &ast.root {
        Some(root_node) => {
            let left = &root_node.borrow().left;
            let right = &root_node.borrow().right;
            if let Some(l) = left {
                let mut left_tree = Tree::new();
                left_tree.root = Some(Rc::clone(l));
                match evaluate(&mut left_tree) {
                    Ok(ft) => l.borrow_mut().value = ft,
                    Err(e) => return Err(e)
                }
            }
            if let Some(r) = right {
                let mut right_tree = Tree::new();
                right_tree.root = Some(Rc::clone(r));
                match evaluate(&mut right_tree) {
                    Ok(ft) => r.borrow_mut().value = ft,
                    Err(e) => return Err(e)
                }
            }
            match (left, right) {
                (Some(l), Some(r)) => {
                    match (
                        &root_node.borrow().value.variant,
                        &l.borrow().value.variant,
                        &r.borrow().value.variant,
                    ) {
                        (Plus, Number(a), Number(b)) => {
                            final_token = Token::from((Number(a + b), 0));
                        }
                        (Minus, Number(a), Number(b)) => {
                            final_token = Token::from((Number(a - b), 0));
                        }
                        (Star, Number(a), Number(b)) => {
                            final_token = Token::from((Number(a * b), 0));
                        }
                        (Slash, Number(a), Number(b)) => {
                            final_token = Token::from((Number(a / b), 0));
                        }
                        (Plus, String(a), String(b)) => {
                            final_token = Token::from((String(format!("{a}{b}")), 0));
                        }
                        (Less, Number(a), Number(b)) => {
                            final_token = Token::from((if a < b { True } else { False }, 0));
                        }
                        (LessEqual, Number(a), Number(b)) => {
                            final_token = Token::from((if a <= b { True } else { False }, 0));
                        }
                        (Greater, Number(a), Number(b)) => {
                            final_token = Token::from((if a > b { True } else { False }, 0));
                        }
                        (GreaterEqual, Number(a), Number(b)) => {
                            final_token = Token::from((if a >= b { True } else { False }, 0));
                        }
                        (EqualEqual, Number(a), Number(b)) => {
                            final_token = Token::from((if a == b { True } else { False }, 0));
                        }
                        (BangEqual, Number(a), Number(b)) => {
                            final_token = Token::from((if a != b { True } else { False }, 0));
                        }
                        (EqualEqual, String(a), String(b)) => {
                            final_token = Token::from((if a == b { True } else { False }, 0));
                        }
                        (BangEqual, String(a), String(b)) => {
                            final_token = Token::from((if a != b { True } else { False }, 0));
                        }
                        (EqualEqual, Number(_), String(_)) | (EqualEqual, String(_), Number(_)) => {
                            final_token = Token::from((False, 0));
                        }
                        (BangEqual, Number(_), String(_)) | (BangEqual, String(_), Number(_)) => {
                            final_token = Token::from((True, 0));
                        }
                        (Plus, _, _) => {
                            errors.push(RuntimeError::new(MustBeNumbersOrStrings, l.borrow().value.line));
                        }
                        (Minus, _, _) => {
                            errors.push(RuntimeError::new(MustBeNumbersOrStrings, l.borrow().value.line));
                        }
                        (Star, Number(_), _) => {
                            errors.push(RuntimeError::new(MustBeNumbers, r.borrow().value.line));
                        }
                        (Star, _, _) => {
                            errors.push(RuntimeError::new(MustBeNumbers, l.borrow().value.line));
                        }
                        (Slash, Number(_), _) => {
                            errors.push(RuntimeError::new(MustBeNumbers, r.borrow().value.line));
                        }
                        (Slash, _, _) => {
                            errors.push(RuntimeError::new(MustBeNumbers, l.borrow().value.line));
                        }
                        (Less, Number(_), _) => {
                            errors.push(RuntimeError::new(MustBeNumbers, r.borrow().value.line));
                        }
                        (Less, _, _) => {
                            errors.push(RuntimeError::new(MustBeNumbers, l.borrow().value.line));
                        }
                        (LessEqual, Number(_), _) => {
                            errors.push(RuntimeError::new(MustBeNumbers, r.borrow().value.line));
                        }
                        (LessEqual, _, _) => {
                            errors.push(RuntimeError::new(MustBeNumbers, l.borrow().value.line));
                        }
                        (Greater, Number(_), _) => {
                            errors.push(RuntimeError::new(MustBeNumbers, r.borrow().value.line));
                        }
                        (Greater, _, _) => {
                            errors.push(RuntimeError::new(MustBeNumbers, l.borrow().value.line));
                        }
                        (GreaterEqual, Number(_), _) => {
                            errors.push(RuntimeError::new(MustBeNumbers, r.borrow().value.line));
                        }
                        (GreaterEqual, _, _) => {
                            errors.push(RuntimeError::new(MustBeNumbers, l.borrow().value.line));
                        }
                        _ => {
                            panic!("Unhandled operation");
                        }
                    }
                }
                (None, Some(v)) => {
                    match (&root_node.borrow().value.variant, &v.borrow().value.variant) {
                        (Bang, Nil) | (Bang, False) => final_token = Token::from((True, 0)),
                        (Bang, Number(_)) | (Bang, True) => final_token = Token::from((False, 0)),
                        (Minus, Number(x)) => final_token = Token::from((Number(-x), 0)),
                        (Minus, _) => errors.push(RuntimeError::new(MustBeNumber, v.borrow().value.line)),
                        _ => {
                            panic!("Unhandled operation");
                        }
                    }
                }
                (None, None) => final_token = root_node.borrow().value.clone(),
                _ => panic!("Invalid tree structure. (Should have gotten a syntax error)"),
            }
        }
        None => todo!(),
    }
    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(final_token)
    }
}
