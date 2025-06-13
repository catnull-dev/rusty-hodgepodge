use regex::Regex;

use crate::lang::token::{self, Token, TokenType, TokenTypes};

pub struct Lexer {
    code: String,
    position: usize,
    tokenList: Vec<Token>,
    tokenTypes: TokenTypes,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer {
            code,
            position: 0,
            tokenList: Vec::<Token>::new(),
            tokenTypes: TokenTypes::new(),
        }
    }

    pub fn analyse(&mut self) -> Vec<Token> {
        while self.next_token() {}

        self.tokenList.clone()
    }

    fn next_token(&mut self) -> bool {
        if self.position >= self.code.len() {
            return false;
        }

        println!("{}", self.position);
        println!("{}", self.code.len());

        for token_type in self.tokenTypes.types.values() {
            let expression = format!(r"^{}", token_type.regex);
            let regexp = Regex::new(&expression).unwrap();

            let result = &self.code[self.position..];

            if let Some(captures) = regexp.captures(result) {
                let data = captures.get(0).unwrap().as_str();
                self.position += data.len();
                let token = Token::new(token_type.clone(), data.to_string(), self.position as u64);
                self.tokenList.push(token);
                return true;
            }
        }

        panic!("Была обнаружена ошибка на позиции: {}", self.position);
    }
}
