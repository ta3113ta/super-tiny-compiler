#[derive(Debug, PartialEq)]
pub enum Token {
    // Arithmetic operators
    Plus,
    Minus,
    OpenParen,
    CloseParen,

    // Keywords
    Add,
    Subtract,

    // Literals
    Number(i32),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut input = input.chars().peekable();

    while let Some(c) = input.next() {
        match c {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '0'..='9' => {
                let mut num = c.to_string();
                while let Some('0'..='9') = input.peek() {
                    num.push(input.next().unwrap());
                }
                let num = num.parse().unwrap();
                tokens.push(Token::Number(num));
            }
            'a'..='z' => {
                let mut keyword = c.to_string();
                while let Some('a'..='z') = input.peek() {
                    keyword.push(input.next().unwrap());
                }
                match keyword.as_str() {
                    "add" => tokens.push(Token::Add),
                    "subtract" => tokens.push(Token::Subtract),
                    _ => panic!("Unexpected keyword: {}", keyword),
                }
            }
            _ => {}
        }
    }

    tokens
}

#[derive(Debug)]
pub enum AstNode {
    Program(Vec<AstNode>),
    CallExpression { name: String, params: Vec<AstNode> },
    NumberLiteral(i32),
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> AstNode {
        let mut nodes = Vec::new();

        while self.pos < self.tokens.len() {
            nodes.push(self.walk());
        }

        AstNode::Program(nodes)
    }

    fn walk(&mut self) -> AstNode {
        let token = &self.tokens[self.pos];

        match token {
            Token::Number(num) => self.parse_number(num),
            Token::Plus => self.parse_keyword("add"),
            Token::Minus => self.parse_keyword("subtract"),
            Token::OpenParen => self.parse_open_paren(),
            Token::CloseParen => self.parse_close_paren(),
            Token::Add => self.parse_call_expression("add"),
            Token::Subtract => self.parse_call_expression("subtract"),
        }
    }

    fn parse_open_paren(&mut self) -> AstNode {
        self.pos += 1;
        let node = self.walk();
        self.pos += 1;
        node
    }

    fn parse_close_paren(&mut self) -> AstNode {
        self.pos += 1;
        AstNode::NumberLiteral(0)
    }

    fn parse_number(&mut self, num: &i32) -> AstNode {
        self.pos += 1;
        AstNode::NumberLiteral(*num)
    }

    fn parse_call_expression(&mut self, name: &str) -> AstNode {
        self.pos += 1;
        let left = self.walk();
        let right = self.walk();
        AstNode::CallExpression {
            name: name.to_string(),
            params: vec![left, right],
        }
    }

    fn parse_keyword(&mut self, keyword: &str) -> AstNode {
        self.pos += 1;
        let node = self.walk();
        AstNode::CallExpression {
            name: keyword.to_string(),
            params: vec![node],
        }
    }
}

