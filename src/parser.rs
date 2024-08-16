use core::fmt;

use crate::lexer::{Token, TokenVariant};

#[derive(PartialEq, Clone, Debug)]
pub struct TreeNode {
    value: Token,
    left: Option<Box<TreeNode>>, //TODO rewrite using Option<Rc<RefCell<TreeNode>>>
    right: Option<Box<TreeNode>>,
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
                left.to_string(),
                right.to_string()
            ),
            (None, Some(right), 0) => {
                format!("({} {})", self.value.short_print(), right.to_string())
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

#[derive(PartialEq)]
enum TreePart {
    Left,
    Right,
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
    let mut current_node: TreeNode = TreeNode::new(tokens[0].clone(), 0);
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
                let result = parse(tokens[i + 1..=pos].to_vec());
                if let Ok(new_tree) = result {
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
                    return result;
                }
            } else {
                return Err(vec![ParserError::new(
                    ParserErrorVariant::UnmatchedParentheses,
                )]);
            }
        } else if tokens[i].is_unary_operator() {
            match current_adding {
                TreePart::Left => {
                    let new_node = TreeNode::new(tokens[i].clone(), 0);
                    current_adding = TreePart::Right;
                    current_node = new_node.clone();
                    ast.root = Some(Box::new(new_node));
                }
                _ => {
                    let mut new_node = TreeNode::new(tokens[i].clone(), 0);
                    new_node.right = Some(Box::new(current_node.clone()));
                    ast.root = Some(Box::new(new_node));
                }
            }
        } else if tokens[i].is_binary_operator() {
            let mut new_node = TreeNode::new(tokens[i].clone(), 0);
            new_node.left = Some(Box::new(current_node.clone()));
            current_adding = TreePart::Right;
            current_node = new_node.clone();
            ast.root = Some(Box::new(new_node));
        } else {
            match current_adding {
                TreePart::Left => {
                    current_node = TreeNode::new(tokens[i].clone(), 0);
                    ast.root = Some(Box::new(current_node.clone()));
                }
                _ => {
                    let new_node = TreeNode::new(tokens[i].clone(), 0);
                    current_node.right = Some(Box::new(new_node.clone()));
                    ast.root = Some(Box::new(current_node.clone()));
                }
            }
        }
        i += 1;
    }
    Ok(ast)
}
