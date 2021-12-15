use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("add")]
    Add,

    #[token("subtract")]
    Subtract,

    #[token("(")]
    OpenBracket,

    #[token(")")]
    ClosingBracket,

    #[regex(r"[0-9][0-9_]*(\.[0-9_]+)?")]
    Number,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[derive(Debug)]
pub enum Node {
    Program { body: Vec<Node> },
    CallExpression { name: String, params: Vec<Node> },
    NumberLiteral(String),
}

struct Parser<'a> {
    tokens: Lexer<'a, Token>,
    stack: Vec<(Option<Token>, String)>,
    latest_value: Option<String>,
}

impl<'a> Parser<'a> {
    fn new(tokens: Lexer<'a, Token>) -> Self {
        Parser {
            tokens,
            stack: vec![],
            latest_value: None,
        }
    }

    fn next(&mut self) -> bool {
        match self.tokens.next() {
            Some(token) => {
                let value = self.tokens.slice().to_string();

                self.stack.push((Some(token), value.clone()));
                self.latest_value = Some(value);

                true
            }
            None => false,
        }
    }

    fn walk(&mut self) -> Result<Node, String> {
        let (token, value) = self.stack.pop().unwrap();

        if let Some(token_kind) = token {
            match token_kind {
                Token::Add | Token::Subtract => {
                    self.next();
                    let mut params: Vec<Node> = vec![];

                    while self.latest_value != Some(")".into()) {
                        params.push(self.walk().unwrap());
                        self.next();
                    }

                    return Ok(Node::CallExpression {
                        name: value,
                        params,
                    });
                }
                Token::OpenBracket => {
                    self.next();
                    return Ok(self.walk().unwrap());
                }
                Token::Number => {
                    return Ok(Node::NumberLiteral(value));
                }
                _ => {}
            }
        }

        Err("Some thing when wrong".to_string())
    }

    fn parse(&mut self) -> Node {
        let mut body: Vec<Node> = vec![];

        while self.next() {
            body.push(self.walk().unwrap());
        }

        Node::Program { body }
    }
}

fn main() {
    let input = "(add 2 (subtract 4 2))";
    let tokens = Token::lexer(input);
    let mut parser = Parser::new(tokens);

    let ast = parser.parse();
    println!("{:#?}", ast);
}
