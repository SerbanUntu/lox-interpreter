use crate::lexer::{Token, TokenVariant};
use crate::parser::Tree;
use std::rc::Rc;
use TokenVariant::*;

pub struct RuntimeError {}

pub fn evaluate(ast: &mut Tree) -> (Token, Vec<RuntimeError>) {
    match &ast.root {
        Some(root_node) => {
            let left = &root_node.borrow().left;
            let right = &root_node.borrow().right;
            if let Some(l) = left {
                let mut left_tree = Tree::new();
                left_tree.root = Some(Rc::clone(l));
                (l.borrow_mut().value, _) = evaluate(&mut left_tree);
            }
            if let Some(r) = right {
                let mut right_tree = Tree::new();
                right_tree.root = Some(Rc::clone(r));
                (r.borrow_mut().value, _) = evaluate(&mut right_tree);
            }
            match (left, right) {
                (Some(l), Some(r)) => {
                    match (
                        &root_node.borrow().value.variant,
                        &l.borrow().value.variant,
                        &r.borrow().value.variant,
                    ) {
                        (Plus, Number(a), Number(b)) => {
                            (Token::from((Number(a + b), 0)), Vec::new())
                        }
                        (Minus, Number(a), Number(b)) => {
                            (Token::from((Number(a - b), 0)), Vec::new())
                        }
                        (Star, Number(a), Number(b)) => {
                            (Token::from((Number(a * b), 0)), Vec::new())
                        }
                        (Slash, Number(a), Number(b)) => {
                            (Token::from((Number(a / b), 0)), Vec::new())
                        }
                        (Plus, String(a), String(b)) => {
                            (Token::from((String(format!("{a}{b}")), 0)), Vec::new())
                        }
                        _ => {
                            panic!("Unhandled operation");
                        }
                    }
                }
                (None, Some(v)) => {
                    match (&root_node.borrow().value.variant, &v.borrow().value.variant) {
                        (Bang, Nil) | (Bang, False) => (Token::from((True, 0)), Vec::new()),
                        (Bang, Number(_)) | (Bang, True) => (Token::from((False, 0)), Vec::new()),
                        (Minus, Number(x)) => (Token::from((Number(-x), 0)), Vec::new()),
                        _ => {
                            panic!("Unhandled operation");
                        }
                    }
                }
                (None, None) => (root_node.borrow().value.clone(), Vec::new()),
                _ => todo!(),
            }
        }
        None => todo!(),
    }
}
