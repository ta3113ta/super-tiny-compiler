use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::parser::AstNode;
use crate::traverser::traverse;
use crate::visitor::Visitor;

#[derive(Debug, PartialEq)]
pub enum NewAstNode {
    Program {
        body: Vec<NewAstNode>,
    },
    ExpressionStatement {
        expression: Box<NewAstNode>,
    },
    CallExpression {
        callee: Box<NewAstNode>,
        arguments: Vec<NewAstNode>,
    },
    Identifier {
        name: String,
    },
    NumberLiteral {
        value: i32,
    },
}

struct TransformContext {
    root: Rc<RefCell<Vec<NewAstNode>>>,
    call_contexts: RefCell<HashMap<*const AstNode, Rc<RefCell<Vec<NewAstNode>>>>>,
}

impl TransformContext {
    fn new() -> (Self, Rc<RefCell<Vec<NewAstNode>>>) {
        let root = Rc::new(RefCell::new(Vec::new()));
        let ctx = Self {
            root: Rc::clone(&root),
            call_contexts: RefCell::new(HashMap::new()),
        };
        (ctx, root)
    }

    fn parent_context(&self, parent: Option<&AstNode>) -> Rc<RefCell<Vec<NewAstNode>>> {
        match parent {
            Some(node @ AstNode::CallExpression { .. }) => {
                let ptr = node as *const AstNode;
                self.call_contexts
                    .borrow()
                    .get(&ptr)
                    .expect("call expression context not registered")
                    .clone()
            }
            Some(AstNode::Program(_)) | None => Rc::clone(&self.root),
            Some(AstNode::NumberLiteral(_)) => {
                panic!("number literal cannot be a parent node")
            }
        }
    }

    fn register_call(&self, node: &AstNode, arguments: Rc<RefCell<Vec<NewAstNode>>>) {
        let ptr = node as *const AstNode;
        self.call_contexts.borrow_mut().insert(ptr, arguments);
    }

    fn take_arguments(&self, node: &AstNode) -> Vec<NewAstNode> {
        let ptr = node as *const AstNode;
        std::mem::take(
            &mut *self
                .call_contexts
                .borrow()
                .get(&ptr)
                .expect("call expression context not registered")
                .borrow_mut(),
        )
    }
}

pub fn transform(ast: AstNode) -> NewAstNode {
    let (ctx, root) = TransformContext::new();
    let ctx = Rc::new(ctx);

    let ctx_for_number = Rc::clone(&ctx);
    let ctx_for_call_enter = Rc::clone(&ctx);
    let ctx_for_call_exit = Rc::clone(&ctx);

    let mut visitor = Visitor::new();

    visitor.number_literal.enter = Some(Box::new(move |node, parent| {
        let AstNode::NumberLiteral(value) = node else {
            return;
        };

        ctx_for_number
            .parent_context(parent)
            .borrow_mut()
            .push(NewAstNode::NumberLiteral { value: *value });
    }));

    visitor.call_expression.enter = Some(Box::new(move |node, _parent| {
        let AstNode::CallExpression { .. } = node else {
            return;
        };

        let arguments = Rc::new(RefCell::new(Vec::new()));
        ctx_for_call_enter.register_call(node, arguments);
    }));

    visitor.call_expression.exit = Some(Box::new(move |node, parent| {
        let AstNode::CallExpression { name, .. } = node else {
            return;
        };

        let call = NewAstNode::CallExpression {
            callee: Box::new(NewAstNode::Identifier {
                name: name.clone(),
            }),
            arguments: ctx_for_call_exit.take_arguments(node),
        };

        let output = if matches!(parent, Some(AstNode::CallExpression { .. })) {
            call
        } else {
            NewAstNode::ExpressionStatement {
                expression: Box::new(call),
            }
        };

        ctx_for_call_exit
            .parent_context(parent)
            .borrow_mut()
            .push(output);
    }));

    traverse(&ast, &mut visitor);

    let body = std::mem::take(&mut *root.borrow_mut());
    NewAstNode::Program { body }
}

#[cfg(test)]
mod tests {
    use crate::parser::{AstNode, Parser};

    use super::*;

    fn parse(input: &str) -> AstNode {
        let tokens = crate::parser::tokenize(input);
        let mut parser = Parser::new(&tokens);
        parser.parse()
    }

    #[test]
    fn transform_builds_c_style_ast() {
        let ast = parse("(add 2 (subtract 4 2))");

        assert_eq!(
            transform(ast),
            NewAstNode::Program {
                body: vec![NewAstNode::ExpressionStatement {
                    expression: Box::new(NewAstNode::CallExpression {
                        callee: Box::new(NewAstNode::Identifier {
                            name: "add".to_string(),
                        }),
                        arguments: vec![
                            NewAstNode::NumberLiteral { value: 2 },
                            NewAstNode::CallExpression {
                                callee: Box::new(NewAstNode::Identifier {
                                    name: "subtract".to_string(),
                                }),
                                arguments: vec![
                                    NewAstNode::NumberLiteral { value: 4 },
                                    NewAstNode::NumberLiteral { value: 2 },
                                ],
                            },
                        ],
                    }),
                }],
            }
        );
    }
}
