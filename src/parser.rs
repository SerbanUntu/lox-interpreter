use core::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use crate::lexer::{Token, TokenVariant};

#[derive(PartialEq, Clone, Debug)]
pub struct TreeNode {
    value: Token,
    left: Option<Rc<RefCell<TreeNode>>>, //TODO rewrite using Option<Rc<RefCell<TreeNode>>>
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

#[derive(PartialEq)]
enum WhereToAdd {
    Left,
    Right,
    RightChild,
}

pub fn parse(mut tokens: Vec<Token>) -> Result<Tree, Vec<ParserError>> {
    tokens.pop();
    if tokens.is_empty() {
        return Err(vec![ParserError::new(
            ParserErrorVariant::UnmatchedParentheses,
        )]);
    }
    let mut ast = Tree::new();
    let mut i = 0;
    let mut current_node: Rc<RefCell<TreeNode>> =
        Rc::new(RefCell::new(TreeNode::new(tokens[0].clone(), 0)));
    let mut wh: WhereToAdd = WhereToAdd::Left;
    while i < tokens.len() {
        if tokens[i].variant == TokenVariant::LeftParen {
            let mut stack_size = 0;
            let mut desired_index: Option<usize> = None;
            for (t_index, t) in tokens.iter().enumerate().skip(i) {
                match t.variant {
                    TokenVariant::LeftParen => {
                        stack_size += 1;
                    }
                    TokenVariant::RightParen => {
                        stack_size -= 1;
                        if stack_size == 0 {
                            desired_index = Some(t_index);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if let Some(pos) = desired_index {
                let result = parse(tokens[i + 1..=pos].to_vec());
                if let Ok(new_tree) = result {
                    if let Some(r) = new_tree.root {
                        match wh {
                            WhereToAdd::Left => {
                                current_node = Rc::clone(&r);
                                ast.root = Some(Rc::clone(&current_node));
                                current_node.borrow_mut().group_count += 1;
                            }
                            _ => {
                                let new_node = Rc::clone(&r);
                                new_node.borrow_mut().group_count += 1;
                                current_node.borrow_mut().right = Some(Rc::clone(&new_node));
                            }
                        }
                    }
                    i = pos + i;
                } else {
                    return result;
                }
            } else {
                return Err(vec![ParserError::new(
                    ParserErrorVariant::UnmatchedParentheses,
                )]);
            }
        } else if tokens[i].is_unary_operator() {
            match wh {
                WhereToAdd::Left => {
                    let new_node = TreeNode::new(tokens[i].clone(), 0);
                    wh = WhereToAdd::Right;
                    current_node = Rc::new(RefCell::new(new_node));
                    ast.root = Some(Rc::clone(&current_node));
                }
                _ => {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(tokens[i].clone(), 0)));
                    current_node.borrow_mut().right = Some(Rc::clone(&new_node));
                    if ast.root.is_none() {
                        ast.root = Some(Rc::clone(&current_node));
                    }
                    current_node = Rc::clone(&new_node);
                }
            }
        } else if tokens[i].is_binary_operator() {
            let mut new_node = TreeNode::new(tokens[i].clone(), 0);
            new_node.left = Some(Rc::clone(&ast.root.unwrap()));
            wh = WhereToAdd::Right;
            current_node = Rc::new(RefCell::new(new_node));
            ast.root = Some(Rc::clone(&current_node));
        } else {
            match wh {
                WhereToAdd::Left => {
                    current_node = Rc::new(RefCell::new(TreeNode::new(tokens[i].clone(), 0)));
                    ast.root = Some(Rc::clone(&current_node));
                }
                WhereToAdd::Right => {
                    let new_node = TreeNode::new(tokens[i].clone(), 0);
                    current_node.borrow_mut().right = Some(Rc::new(RefCell::new(new_node)));
                }
                WhereToAdd::RightChild => {
                    let new_node_ref = Rc::new(RefCell::new(TreeNode::new(tokens[i].clone(), 0)));
                    current_node.borrow_mut().right = Some(Rc::clone(&new_node_ref));
                    current_node = Rc::clone(&new_node_ref);
                    wh = WhereToAdd::Right;
                }
            }
        }
        i += 1;
    }
    Ok(ast)
}
