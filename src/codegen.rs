use crate::transformer::NewAstNode;

pub fn generate(node: &NewAstNode) -> String {
    match node {
        NewAstNode::Program { body } => body.iter().map(generate).collect::<Vec<_>>().join("\n"),
        NewAstNode::ExpressionStatement { expression } => format!("{};", generate(expression)),
        NewAstNode::CallExpression { callee, arguments } => {
            let args = arguments
                .iter()
                .map(generate)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({})", generate(callee), args)
        }
        NewAstNode::Identifier { name } => name.clone(),
        NewAstNode::NumberLiteral { value } => value.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformer::NewAstNode;

    #[test]
    fn generates_identifier() {
        let node = NewAstNode::Identifier {
            name: "add".to_string(),
        };
        assert_eq!(generate(&node), "add");
    }

    #[test]
    fn generates_number_literal() {
        let node = NewAstNode::NumberLiteral { value: 42 };
        assert_eq!(generate(&node), "42");
    }

    #[test]
    fn generates_nested_call_expression() {
        let node = NewAstNode::CallExpression {
            callee: Box::new(NewAstNode::Identifier {
                name: "subtract".to_string(),
            }),
            arguments: vec![
                NewAstNode::NumberLiteral { value: 4 },
                NewAstNode::NumberLiteral { value: 2 },
            ],
        };
        assert_eq!(generate(&node), "subtract(4, 2)");
    }

    #[test]
    fn generates_full_program() {
        let node = NewAstNode::Program {
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
        };
        assert_eq!(generate(&node), "add(2, subtract(4, 2));");
    }
}
