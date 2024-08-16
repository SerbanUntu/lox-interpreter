[![progress-banner](https://backend.codecrafters.io/progress/interpreter/066ff837-6033-4841-b2ae-19d073a1a7b8)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

# Overview

This is my solution for the
["Build your own Interpreter" Challenge](https://app.codecrafters.io/courses/interpreter/overview) on [CodeCrafters](https://app.codecrafters.io).

The challenge follows the book
[Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom.

For this challenge I built an interpreter for [Lox](https://craftinginterpreters.com/the-lox-language.html), a simple scripting language.

This challenge covers topics such as **tokenization**, **AST**s, **tree-walk interpreters** and more.

# My approach

**Disclaimer**: *Don't expect any of the code to be even remotely decent.*

The interpreter consists of 3 main stages.

First, the raw text is transformed into a series of tokens (whitespace and comments are removed, strings and identifiers are processed). 

Then, an Abstract Syntax Tree (AST), which represents an expression, is built up from those tokens. 

Finally, the expression is evaluated and the result is printed to stdout.

The functionality related to each stage is stored in its own file. There is also the `main.rs` file, which processes the terminal commands and prints the results to stdout.

## Lexer (or tokenizer)

The first stage of the interpreter takes the raw text file and produces a list of useful tokens *(useful for stages 2 and 3 that is)*.

The `tokenize()` function is the only function that does not return a `Result<T>` because the tests require both a list of errors and a list of tokens to be printed at the same time *(if errors occur)*.

```rs
// test.lox
"Hello" == "World" ?
---
// stdout
[line 1] Error: Unexpected character: ?
STRING "Hello" Hello
EQUAL_EQUAL == null
STRING "World" World
EOF  null
```

Reading some tokens is easier than others. For example, characters such as `+` or `-` can be automatically processed. Characters such as `=` and `!` might be or not be followed by another `=`, so they wait one more turn before they are processed.

Numbers have the most edge cases:

```rs
12 => Token::Number(12)
12.4 => Token::Number(12.4)
0 => Token::Number(0)
012 => Token::Number(0), Token::Number(12)
13. => Token::Number(13), Token::Dot
1.2.3 => Token::Number(1.2), Token::Dot, Token::Number(3)
```

The way I reduced my `lexer.rs` file by almost half was to attempt to convert the accumulated characters into a token using the implementation of the `From<&str>` trait:

```rs
impl From<(&str, u32)> for Token {
    fn from((literal, line): (&str, u32)) -> Self {
        Self {
            variant: match literal {
                // some cases skipped
                "=" => TokenVariant::Equal, // Operators
                "==" => TokenVariant::EqualEqual,
                "!" => TokenVariant::Bang,
                "!=" => TokenVariant::BangEqual,
                "<" => TokenVariant::Less,
                ">" => TokenVariant::Greater,
                "print" => TokenVariant::Print, // Keywords
                "return" => TokenVariant::Return,
                "super" => TokenVariant::Super,
                "this" => TokenVariant::This,
                "true" => TokenVariant::True,
                "" => TokenVariant::Eof,
                s if s.starts_with("\"") && s.ends_with("\"") && s.len() > 1 => {
                    TokenVariant::String(literal.trim_matches('\"').to_owned())
                }
                s if s.parse::<f64>().is_ok() => TokenVariant::Number(s.parse().unwrap()),
                _ => TokenVariant::Identifier, // Invalid identifiers are still processed as identifiers!
            }, // ...
        }
    }
}
```

If the token that comes out the other way is a `Token::Number()` or a `Token::Identifier`, then edge cases must be accounted for and this method call was not sufficient. But most of the time, this form of manual processing works.

## Parser

The second stage of the interpreter is the stage at which the **AST** is built.

The `parse()` function takes in a vector of `Token`s and produces a `Tree` (or syntax errors, of course).

In Rust, dealing with recursive data types such as trees is a bit more *interesting*.

```rs
pub struct TreeNode {
    pub value: Token,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
    group_count: u32, // Only used to make the test runner happy
}
```

So, the `Option` part is pretty straight forward; either there is a left/right node, or there isn't.

The `Rc<RefCell>` part allows me to mutate nodes indirectly, through other nodes that hold a reference to them. For example, if I have the root of a tree (`ast.root`) and I copy it into `new_root` using `Rc::clone()`, then, if I mutate `new_root`, it will also mutate `ast.root`, since they reference the same data. 

Rust allows me to do this since `Rc` and `RefCell` count references and do all sorts of calculations to ensure I am not breaking any of the borrow checker's rules.

An alternative would have been wrapping everything in `unsafe {}` and using raw pointers *the C++ way*.

The way these trees represent expressions is simple.
- A root node holds a unary or a binary operator.
- If it holds a binary operator, then the two operands are stored in the left and right child nodes.
- If it holds a unary operator, then the operand is stored in the right child node.
- The child nodes can themselves store operators and not values, indicating nested expressions.
- Therefore, parantheses are not required, since nested expressions are represented by these subtrees.

When a `Token::LeftParen` is met, the `parse()` function is called recursively to produce the root of the AST of the subexpression, which is then added to the main tree.

The bulk of the code is concerned with adding new nodes in the right places in the tree.

### Operator precedence

Operator precedence was really fun to implement because the parser cannot look in the future to see what operations come next, so the whole tree must be rearranged if an operation with higher precedence is met.

An example when parsing `1 + 2 * 3`:

```rs
                                                                  Token::Plus
                 Token::Plus                                     /           \
                /           \                 => Token::Number(1)             Token::Star
Token::Number(1)             Token::Number(2)                                /           \
                                                             Token::Number(2)             Token::Number(3)
```

And this is how it's done:

```rs
_ if tokens[i].is_binary_operator() => match tm { 
    // tm, short for tree_manipulation, is somewhere between a descriptor for the expected token and the way in which the tree is mutated. It's an enum.
    Operator if tokens[i].get_precedence() > last_precedence => {
        let new_node = Rc::new(RefCell::new(TreeNode::new(tokens[i].clone(), 0)));
        match &ast.root {
            Some(root_node) => {
                match &root_node.borrow().right {
                    Some(right_node) => {
                        // The right child of the previous operation becomes the left child of the current operation (see Token::Number(2) in the example)
                        new_node.borrow_mut().left = Some(Rc::clone(&right_node));
                        current = Some(Rc::clone(&new_node));
                        // Now an expression is expected to fill the right child of the current operation
                        tm = RightChild;
                        last_precedence = tokens[i].get_precedence();
                    }
                }
                root_node.borrow_mut().right = Some(Rc::clone(&new_node));
            }
        }
    }
```

## Evaluator

The third stage of the interpreter is the stage at which expressions are evaluated.

The `evaluate()` function turns a tree (or a subtree when used recursively) into a single token that represents the result of the expression encoded by that tree.

```rs
                 Token::Star
                /           \                 => Token::Number(12)
Token::Number(3)             Token::Number(4)
```

```rs
Token::Bang
           \            => Token::False
            Token::True
// Bang is '!'
```

It was by far the easiest to implement (partly because CodeCrafters' tests didn't cover every possible edge case) and it mostly consists of converting trees into Rust expressions. *(Eventually)* I'll write a macro to reduce this boilerplate.

```rs
match (
    &root_node.borrow().value.variant,
    &l.borrow().value.variant,
    &r.borrow().value.variant,
) {
    (Plus, Number(a), Number(b)) => {
        final_token = Token::from((Number(a + b), 0)); // Instead of '0' it should determine the line number of the result of the expression (somehow)
    }
    (Minus, Number(a), Number(b)) => {
        final_token = Token::from((Number(a - b), 0));
    }
    // ...and many more cases.
}
```
