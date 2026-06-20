use tiny_with_lib::{codegen, compile, parser::Parser, transformer::transform};

fn main() {
    let input = "(add 2 (subtract 4 2))";
    let tokens = tiny_with_lib::parser::tokenize(input);
    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();

    println!("Input: {}", input);
    println!("Tokens: {:?}", tokens);
    println!("AST: {:#?}", ast);

    let new_ast = transform(ast);
    println!("New AST: {:#?}", new_ast);

    let output = codegen::generate(&new_ast);
    println!("Output: {}", output);

    assert_eq!(compile(input), output);
}
