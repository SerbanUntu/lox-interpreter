use core::fmt;

use crate::lexer::{Token, TokenVariant};

#[derive(PartialEq, Clone, Debug)]
pub struct TreeNode {
    value: Token,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
    parent: Option<Box<TreeNode>>,
    group_count: u32,
}

impl TreeNode {
    fn new(value: Token, group_count: u32, parent: Option<Box<TreeNode>>) -> Self {
        Self {
            value,
            left: None,
            right: None,
            parent,
            group_count,
        }
    }
}

impl ToString for TreeNode {
    fn to_string(&self) -> String {
        match (&self.left, &self.right, self.group_count) {
            (None, None, 0) => format!("{}", self.value.short_print()),
            (None, None, 1) => format!("(group {})", self.value.short_print()),
            (Some(left), Some(right), 0) => format!(
                "({} {} {})",
                self.value.short_print(),
                left.to_string(),
                right.to_string()
            ),
            (Some(left), Some(right), 1) => format!(
                "(group ({} {} {}))",
                self.value.short_print(),
                left.to_string(),
                right.to_string()
            ),
            (_, _, x) if x > 1 => format!("(group {})", {
                let mut copy = self.clone();
                copy.group_count -= 1;
                copy.to_string()
            }),
            _ => {
                panic!("Invalid tree structure");
            }
        }
    }
}

#[derive(Debug)]
pub struct Tree {
    pub root: Option<Box<TreeNode>>,
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

enum TreePart {
    Left,
    Right,
}

// Change return type to result
pub fn parse(mut tokens: Vec<Token>) -> (Tree, Option<Vec<ParserError>>) {
    tokens.pop();
    if tokens.is_empty() {
        return (
            Tree::new(),
            Some(vec![ParserError::new(
                ParserErrorVariant::UnmatchedParentheses,
            )]),
        );
    }
    let mut ast = Tree::new();
    let mut i = 0;
    let mut current_node: TreeNode = TreeNode::new(tokens[0].clone(), 0, None);
    let mut current_adding: TreePart = TreePart::Left;
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
                let (new_tree, errors) = parse(tokens[i + 1..=pos].to_vec());
                if errors.is_some() {
                    return (Tree::new(), errors);
                }
                if let Some(r) = new_tree.root {
                    match current_adding {
                        TreePart::Left => {
                            current_node = *r;
                            current_node.group_count += 1;
                            ast.root = Some(Box::new(current_node.clone()));
                        }
                        _ => {
                            let mut new_node = *r;
                            new_node.group_count += 1;
                            current_node.right = Some(Box::new(new_node.clone()));
                            ast.root = Some(Box::new(current_node.clone()));
                        }
                    }
                }
                i = pos + i;
            } else {
                return (
                    Tree::new(),
                    Some(vec![ParserError::new(
                        ParserErrorVariant::UnmatchedParentheses,
                    )]),
                );
            }
        } else if tokens[i].is_binary_operator() {
            let mut new_node = TreeNode::new(tokens[i].clone(), 0, None);
            new_node.left = Some(Box::new(current_node.clone()));
            current_node.parent = Some(Box::new(new_node.clone()));
            current_adding = TreePart::Right;
            current_node = new_node.clone();
            ast.root = Some(Box::new(new_node));
        } else {
            match current_adding {
                TreePart::Left => {
                    current_node = TreeNode::new(tokens[i].clone(), 0, None);
                    ast.root = Some(Box::new(current_node.clone()));
                }
                _ => {
                    let new_node =
                        TreeNode::new(tokens[i].clone(), 0, Some(Box::new(current_node.clone())));
                    current_node.right = Some(Box::new(new_node.clone()));
                    ast.root = Some(Box::new(current_node.clone()));
                }
            }
        }
        i += 1;
    }
    (ast, None)
}

/*
()
T
U T
U ()
T B T
() B T
T B ()
() B ()
*/
