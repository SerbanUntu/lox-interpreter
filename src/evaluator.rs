use crate::lexer::{Token, TokenVariant};
use crate::parser::{Tree, TreeNode};
use std::cell::RefCell;
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
