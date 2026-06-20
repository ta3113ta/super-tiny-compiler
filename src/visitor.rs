use crate::parser::AstNode;

pub struct NodeVisitor {
    pub enter: Option<Box<dyn FnMut(&AstNode, Option<&AstNode>)>>,
    pub exit: Option<Box<dyn FnMut(&AstNode, Option<&AstNode>)>>,
}

impl NodeVisitor {
    pub fn new() -> Self {
        Self {
            enter: None,
            exit: None,
        }
    }
}

pub struct Visitor {
    pub program: NodeVisitor,
    pub call_expression: NodeVisitor,
    pub number_literal: NodeVisitor,
}

impl Visitor {
    pub fn new() -> Self {
        Self {
            program: NodeVisitor::new(),
            call_expression: NodeVisitor::new(),
            number_literal: NodeVisitor::new(),
        }
    }
}
