use crate::lexer::Token;

#[derive(PartialEq, Clone)]
pub struct TreeNode {
    value: Token,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>
}

impl TreeNode {
    fn new(value: Token) -> Self {
        Self {
            value,
            left: None,
            right: None
        }
    }
}
impl ToString for TreeNode {
    fn to_string(&self) -> String {
        match (&self.left, &self.right) {
            (None, None) => format!("{}", self.value.short_print()),
            (Some(left), Some(right)) => format!(
                "( {} {} {} )",
                left.to_string(),
                self.value.short_print(),
                right.to_string()
            ),
            _ => panic!("Invalid tree structure"), // In case of a malformed tree with only one child
        }
    }
}

pub struct Tree {
    pub root: Option<Box<TreeNode>>
}

impl Tree {
    fn new() -> Self {
        Self {
            root: None
        }
    }
}

pub fn parse(mut tokens: Vec<Token>) -> Tree {
    tokens.pop(); // Removes EOF
    let mut ast = Tree::new();
    if tokens.len() == 1 {
        ast.root = Some(Box::new(TreeNode::new(tokens[0].clone())));
    }
    if tokens.len() == 3 {
        let mut root_node = TreeNode::new(tokens[0].clone());
        root_node.left = Some(Box::new(TreeNode::new(tokens[1].clone())));
        root_node.right = Some(Box::new(TreeNode::new(tokens[2].clone())));
        ast.root = Some(Box::new(root_node.clone()));
    }
    ast
}
