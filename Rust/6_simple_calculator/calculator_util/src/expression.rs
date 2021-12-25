#![allow(dead_code)]

/// Expression struct
/// 
/// An expression is basically a `String` but with an extra flag indicates
/// whether it is an Reverse Polish Notation (postfix notation) or not.
#[derive(Clone)]
pub struct Expression {
    pub content: String,
    is_postfix: bool
}

impl Expression {
    pub fn new(content: String, is_postfix: bool) -> Self {
        Self { content, is_postfix }
    }

    pub fn replace_content(&self, content: String) -> Self {
        Self { content, is_postfix: self.is_postfix }
    }

    pub fn is_postfix(&self) -> bool {
        self.is_postfix
    }
}

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.content)
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}