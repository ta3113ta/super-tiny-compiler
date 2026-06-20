use crate::parser::AstNode;
use crate::visitor::Visitor;

pub fn traverse(ast: &AstNode, visitor: &mut Visitor) {
    traverse_node(ast, None, visitor);
}

fn traverse_node(node: &AstNode, parent: Option<&AstNode>, visitor: &mut Visitor) {
    match node {
        AstNode::Program(_) => {
            if let Some(enter) = &mut visitor.program.enter {
                enter(node, parent);
            }
        }
        AstNode::CallExpression { .. } => {
            if let Some(enter) = &mut visitor.call_expression.enter {
                enter(node, parent);
            }
        }
        AstNode::NumberLiteral(_) => {
            if let Some(enter) = &mut visitor.number_literal.enter {
                enter(node, parent);
            }
        }
    }

    match node {
        AstNode::Program(body) => {
            for child in body {
                traverse_node(child, Some(node), visitor);
            }
        }
        AstNode::CallExpression { params, .. } => {
            for child in params {
                traverse_node(child, Some(node), visitor);
            }
        }
        AstNode::NumberLiteral(_) => {}
    }

    match node {
        AstNode::Program(_) => {
            if let Some(exit) = &mut visitor.program.exit {
                exit(node, parent);
            }
        }
        AstNode::CallExpression { .. } => {
            if let Some(exit) = &mut visitor.call_expression.exit {
                exit(node, parent);
            }
        }
        AstNode::NumberLiteral(_) => {
            if let Some(exit) = &mut visitor.number_literal.exit {
                exit(node, parent);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::parser::AstNode;
    use crate::visitor::Visitor;

    use super::traverse;

    fn fixture_ast() -> AstNode {
        AstNode::Program(vec![AstNode::CallExpression {
            name: "add".to_string(),
            params: vec![
                AstNode::NumberLiteral(2),
                AstNode::CallExpression {
                    name: "subtract".to_string(),
                    params: vec![AstNode::NumberLiteral(4), AstNode::NumberLiteral(5)],
                },
            ],
        }])
    }

    #[test]
    fn traverse_calls_enter_before_exit_and_visits_children_between() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let events_for_program = Rc::clone(&events);
        let events_for_call = Rc::clone(&events);
        let events_for_number = Rc::clone(&events);
        let events_for_program_exit = Rc::clone(&events);
        let events_for_call_exit = Rc::clone(&events);
        let events_for_number_exit = Rc::clone(&events);

        let mut visitor = Visitor::new();

        visitor.program.enter = Some(Box::new(move |_, _| {
            events_for_program.borrow_mut().push("enter:Program".to_string());
        }));
        visitor.call_expression.enter = Some(Box::new(move |node, parent| {
            let AstNode::CallExpression { name, .. } = node else {
                return;
            };
            let parent_label = match parent {
                None => "None".to_string(),
                Some(AstNode::Program(_)) => "Program".to_string(),
                Some(AstNode::CallExpression { name, .. }) => format!("CallExpression({name})"),
                Some(AstNode::NumberLiteral(_)) => "NumberLiteral".to_string(),
            };
            events_for_call
                .borrow_mut()
                .push(format!("enter:CallExpression({name}) parent={parent_label}"));
        }));
        visitor.number_literal.enter = Some(Box::new(move |node, parent| {
            let AstNode::NumberLiteral(value) = node else {
                return;
            };
            let parent_label = match parent {
                None => "None".to_string(),
                Some(AstNode::Program(_)) => "Program".to_string(),
                Some(AstNode::CallExpression { name, .. }) => format!("CallExpression({name})"),
                Some(AstNode::NumberLiteral(_)) => "NumberLiteral".to_string(),
            };
            events_for_number
                .borrow_mut()
                .push(format!("enter:NumberLiteral({value}) parent={parent_label}"));
        }));

        visitor.program.exit = Some(Box::new(move |_, _| {
            events_for_program_exit
                .borrow_mut()
                .push("exit:Program".to_string());
        }));
        visitor.call_expression.exit = Some(Box::new(move |node, _| {
            let AstNode::CallExpression { name, .. } = node else {
                return;
            };
            events_for_call_exit
                .borrow_mut()
                .push(format!("exit:CallExpression({name})"));
        }));
        visitor.number_literal.exit = Some(Box::new(move |node, _| {
            let AstNode::NumberLiteral(value) = node else {
                return;
            };
            events_for_number_exit
                .borrow_mut()
                .push(format!("exit:NumberLiteral({value})"));
        }));

        let ast = fixture_ast();
        traverse(&ast, &mut visitor);

        assert_eq!(
            *events.borrow(),
            vec![
                "enter:Program".to_string(),
                "enter:CallExpression(add) parent=Program".to_string(),
                "enter:NumberLiteral(2) parent=CallExpression(add)".to_string(),
                "exit:NumberLiteral(2)".to_string(),
                "enter:CallExpression(subtract) parent=CallExpression(add)".to_string(),
                "enter:NumberLiteral(4) parent=CallExpression(subtract)".to_string(),
                "exit:NumberLiteral(4)".to_string(),
                "enter:NumberLiteral(5) parent=CallExpression(subtract)".to_string(),
                "exit:NumberLiteral(5)".to_string(),
                "exit:CallExpression(subtract)".to_string(),
                "exit:CallExpression(add)".to_string(),
                "exit:Program".to_string(),
            ]
        );
    }
}
