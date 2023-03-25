mod parser;

use parser::Parser;

fn main() {
    let input = "(add 2 (subtract 4 2))";
    let tokens = parser::tokenize(input);
    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();

    println!("Input: {}", input);
    println!("Tokens: {:?}", tokens);
    println!("AST: {:#?}", ast);
}
