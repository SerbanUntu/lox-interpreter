use core::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use crate::lexer::{Token, TokenVariant};

#[derive(PartialEq, Clone, Debug)]
pub struct TreeNode {
    value: Token,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
    group_count: u32,
}

impl TreeNode {
    fn new(value: Token, group_count: u32) -> Self {
        Self {
            value,
            left: None,
            right: None,
            group_count,
        }
    }
}

impl ToString for TreeNode {
    fn to_string(&self) -> String {
        match (&self.left, &self.right, self.group_count) {
            (None, None, 0) => format!("{}", self.value.short_print()),
            (Some(left), Some(right), 0) => format!(
                "({} {} {})",
                self.value.short_print(),
                left.borrow().to_string(),
                right.borrow().to_string()
            ),
            (None, Some(right), 0) => {
                format!(
                    "({} {})",
                    self.value.short_print(),
                    right.borrow().to_string()
                )
            }
            (Some(_), None, _) => {
                panic!("Invalid tree structure");
            }
            (_, _, _) => format!("(group {})", {
                let mut copy = self.clone();
                copy.group_count -= 1;
                copy.to_string()
            }),
        }
    }
}

#[derive(Debug)]
pub struct Tree {
    pub root: Option<Rc<RefCell<TreeNode>>>,
}

impl Tree {
    fn new() -> Self {
        Self { root: None }
    }
}

#[derive(Debug)]
enum ParserErrorVariant {
    UnmatchedParentheses,
}

impl fmt::Display for ParserErrorVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserErrorVariant::UnmatchedParentheses => write!(f, "Unmatched parentheses."),
        }
    }
}

#[derive(Debug)]
pub struct ParserError {
    variant: ParserErrorVariant,
}

impl ParserError {
    fn new(variant: ParserErrorVariant) -> Self {
        Self { variant }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.variant)
    }
}

#[derive(Debug, PartialEq)]
enum TreeManipulation {
    Root,
    Operator,
    RightChild,
}

use TreeManipulation::*;

fn get_index_of_closing_paren(tokens: &Vec<Token>, start: usize) -> Option<usize> {
    let mut stack_size = 0;
    for (t_index, t) in tokens.iter().enumerate().skip(start) {
        match t.variant {
            TokenVariant::LeftParen => {
                stack_size += 1;
            }
            TokenVariant::RightParen => {
                stack_size -= 1;
                if stack_size == 0 {
                    return Some(t_index);
                }
            }
            _ => {}
        }
    }
    None
}

fn parse_sub_expression(
    tokens: &Vec<Token>,
    index: &mut usize,
) -> Result<Rc<RefCell<TreeNode>>, Vec<ParserError>> {
    if let Some(pos) = get_index_of_closing_paren(tokens, *index) {
        match parse(&tokens[*index + 1..pos].to_vec()) {
            Ok(ref new_tree) => {
                if let Some(ref new_node) = new_tree.root {
                    new_node.borrow_mut().group_count += 1;
                    *index += pos;
                    return Ok(Rc::clone(&new_node));
                }
            }
            Err(e) => return Err(e),
        }
    }
    Err(vec![ParserError::new(
        ParserErrorVariant::UnmatchedParentheses,
    )])
}

pub fn parse(tokens: &Vec<Token>) -> Result<Tree, Vec<ParserError>> {
    if tokens.is_empty() {
        return Err(vec![ParserError::new(
            ParserErrorVariant::UnmatchedParentheses,
        )]);
    }
    let mut ast = Tree::new();
    let mut i = 0;
    let mut current: Option<Rc<RefCell<TreeNode>>> = None;
    let mut tm = Root;
    let mut last_precedence: u32 = 99;
    while i < tokens.len() {
        match tokens[i].variant {
            TokenVariant::Eof => {}
            _ if tokens[i].is_unary_operator() => match tm {
                Root => {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(tokens[i].clone(), 0)));
                    ast.root = Some(Rc::clone(&new_node));
                    current = Some(Rc::clone(&new_node));
                    tm = RightChild;
                }
                RightChild => match &current {
                    Some(current_node) => {
                        let new_node = Rc::new(RefCell::new(TreeNode::new(tokens[i].clone(), 0)));
                        current_node.borrow_mut().right = Some(Rc::clone(&new_node));
                        current = Some(Rc::clone(&new_node));
                    }
                    _ => panic!("Expected the current node to be populated at this point."),
                },
                _ => panic!("Unhandled unary case: {tm:?}"),
            },
            _ if tokens[i].is_binary_operator() => match tm {
                Root => panic!("Expected the tree to be initialized at this point."),
                Operator if tokens[i].get_precedence() > last_precedence => {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(tokens[i].clone(), 0)));
                    match &ast.root {
                        Some(root_node) => {
                            match &root_node.borrow().right {
                                Some(right_node) => {
                                    new_node.borrow_mut().left = Some(Rc::clone(&right_node));
                                    current = Some(Rc::clone(&new_node));
                                    tm = RightChild;
                                    last_precedence = tokens[i].get_precedence();
                                }
                                _ => {
                                    panic!("Expected the root to have a right node at this point.")
                                }
                            }
                            root_node.borrow_mut().right = Some(Rc::clone(&new_node));
                        }
                        _ => panic!("Expected the tree to be initialized at this point."),
                    }
                }
                Operator => {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(tokens[i].clone(), 0)));
                    let root_node = ast.root.unwrap();
                    new_node.borrow_mut().left = Some(Rc::clone(&root_node));
                    ast.root = Some(Rc::clone(&new_node));
                    current = Some(Rc::clone(&new_node));
                    tm = RightChild;
                    last_precedence = tokens[i].get_precedence();
                }
                _ => panic!("Unhandled binary case: {tm:?}"),
            },
            _ => {
                let new_node;
                if tokens[i].variant == TokenVariant::LeftParen {
                    let result = parse_sub_expression(tokens, &mut i);
                    match result {
                        Ok(n) => new_node = n,
                        Err(e) => return Err(e),
                    }
                } else {
                    new_node = Rc::new(RefCell::new(TreeNode::new(tokens[i].clone(), 0)));
                }
                match tm {
                    Root => {
                        ast.root = Some(Rc::clone(&new_node));
                        current = Some(Rc::clone(&new_node));
                        tm = Operator;
                    }
                    RightChild => match (&ast.root, &current) {
                        (Some(_), Some(current_node)) => {
                            current_node.borrow_mut().right = Some(Rc::clone(&new_node));
                            current = Some(Rc::clone(&new_node));
                            tm = Operator;
                        }
                        _ => {
                            panic!("Expected the nodes to be populated at this point.")
                        }
                    },
                    _ => panic!("Unhandled regular case: {tm:?}"),
                }
                tm = Operator;
            }
        }
        i += 1;
    }
    Ok(ast)
}
