pub mod codegen;
pub mod parser;
pub mod traverser;
pub mod transformer;
pub mod visitor;

pub fn compile(input: &str) -> String {
    let tokens = parser::tokenize(input);
    let mut parser = parser::Parser::new(&tokens);
    let ast = parser.parse();
    let new_ast = transformer::transform(ast);
    codegen::generate(&new_ast)
}
