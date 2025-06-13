use std::collections::HashMap;

struct Token {
    token_type: TokenType,
    text: String,
    position: u64,
}

impl Token {
    fn new(token_type: TokenType, text: String, position: u64) -> Self {
        Token {
            token_type,
            text,
            position,
        }
    }
}

struct TokenType {
    name: String,
    regex: String,
}

impl TokenType {
    fn new(name: String, regex: String) -> Self {
        TokenType { name, regex }
    }
}

struct TokenTypes {
    types: HashMap<String, TokenType>,
}

impl TokenTypes {
    fn new() -> Self {
        let mut collection: HashMap<String, TokenType> = HashMap::new();
        collection.insert(
            String::from("NUMBER"),
            TokenType::new("NUMBER".to_string(), "[0-9]*".to_string()),
        );

        collection.insert(
            String::from("VARIABLE"),
            TokenType::new("VARIABLE".to_string(), "[а-я]*".to_string()),
        );

        collection.insert(
            String::from("SEMICOLON"),
            TokenType::new("SEMICOLON".to_string(), ";".to_string()),
        );

        collection.insert(
            String::from("SPACE"),
            TokenType::new("SPACE".to_string(), "[ \\n\\r\\t]".to_string()),
        );

        collection.insert(
            String::from("ASSIGN"),
            TokenType::new("ASSIGN".to_string(), "РАВНО".to_string()),
        );

        collection.insert(
            String::from("LOG"),
            TokenType::new("LOG".to_string(), "ЛОГ".to_string()),
        );

        collection.insert(
            String::from("PLUS"),
            TokenType::new("PLUS".to_string(), "ПЛЮС".to_string()),
        );

        collection.insert(
            String::from("MINUS"),
            TokenType::new("MINUS".to_string(), "МИНУС".to_string()),
        );

        collection.insert(
            String::from("LPAR"),
            TokenType::new("LPAR".to_string(), "\\(".to_string()),
        );

        collection.insert(
            String::from("RPAR"),
            TokenType::new("RPAR".to_string(), "\\)".to_string()),
        );

        TokenTypes { types: collection }
    }

    fn get_data(&self, data_type: &String) -> Option<&TokenType> {
        self.types.get(data_type)
    }
}
