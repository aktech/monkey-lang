use core::fmt;
use std::collections::HashMap;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new(input);

    let mut tokens = vec![];
    while let Some(token) = tokenizer.get_next_token()  {
        tokens.push(token);
    }

    tokens
}

#[derive(Clone)]
pub enum Token {
    // Identifier
    Identifier(String),

    // Fundamental data types
    Integer(i32),
    Boolean(bool),

    // Keywords
    Let,
    Fn,
    If,
    Else,

    // Punctuation
    Plus,
    Minus,
    Star,
    Slash,
    Assignment,
    Equals,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(data) => write!(f, "<identifier, {}>", data),
            Self::Integer(data) => write!(f, "<integer, {}>", data),
            Self::Boolean(data) => write!(f, "<boolean, {}>", data),
            Self::Let => write!(f, "<let, let>"),
            Self::Fn => write!(f, "<fn, fn>"),
            Self::If => write!(f, "<if, if>"),
            Self::Else => write!(f, "<else, else>"),
            Self::Plus => write!(f, "<+, +>"),
            Self::Minus => write!(f, "<-, ->"),
            Self::Star => write!(f, "<*, *>"),
            Self::Slash => write!(f, "</, />"),
            Self::Assignment => write!(f, "<=, =>"),
            Self::Equals => write!(f, "<==, ==>"),
            Self::Semicolon => write!(f, "<;, ;>"),
            Self::LeftParen => write!(f, "<(, (>"),
            Self::RightParen => write!(f, "<), )>"),
            Self::LeftBrace => write!(f, "<{{, {{>"),
            Self::RightBrace => write!(f, "<}}, }}>"),
        }
    }
}

struct Tokenizer {
    remaining_input: Vec<char>,
    punctuation_to_token: HashMap<String, Token>,
    keyword_to_token: HashMap<String, Token>,
}

impl Tokenizer {
    fn new(input: &str) -> Self {
        Tokenizer {
            remaining_input: input.chars().collect(),
            punctuation_to_token: HashMap::from([
                (String::from("+"), Token::Plus),
                (String::from("-"), Token::Minus),
                (String::from("*"), Token::Star),
                (String::from("/"), Token::Slash),
                (String::from("="), Token::Assignment),
                (String::from("=="), Token::Equals),
                (String::from(";"), Token::Semicolon),
                (String::from("("), Token::LeftParen),
                (String::from(")"), Token::RightParen),
                (String::from("{"), Token::LeftBrace),
                (String::from("}"), Token::RightBrace),
            ]),
            keyword_to_token: HashMap::from([
                (String::from("true"), Token::Boolean(true)),
                (String::from("false"), Token::Boolean(false)),
                (String::from("let"), Token::Let),
                (String::from("fn"), Token::Fn),
                (String::from("if"), Token::If),
                (String::from("else"), Token::Else),
            ]),
        }
    }

    fn get_next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.remaining_input.len() == 0 {
            return None;
        }

        let token;
        if self.remaining_input[0].is_ascii_alphabetic() {
            token = self.chop_identifer_or_keyword_token();
        } else if self.remaining_input[0].is_numeric() {
            token = self.chop_integer_token();
        } else if self.is_current_character_punctuation() {
            token = self.chop_punctuation_token();
        } else {
            return None;
        }

        Some(token)
    }

    fn skip_whitespace(&mut self) {
        let mut idx = 0;
        while idx < self.remaining_input.len() && self.remaining_input[idx].is_ascii_whitespace() {
            idx += 1;
        }
        self.remaining_input = self.remaining_input[idx..].to_vec();
    }

    fn chop_identifer_or_keyword_token(&mut self) -> Token {
        let mut idx = 0;
        assert!(self.remaining_input[0].is_ascii_alphabetic());
        while idx < self.remaining_input.len() && self.remaining_input[idx].is_ascii_alphanumeric() {
            idx += 1;
        }

        let data_vector = self.remaining_input[..idx].to_vec();
        self.remaining_input = self.remaining_input[idx..].to_vec();
        let data: String = data_vector.into_iter().collect();

        match self.keyword_to_token.get(&data) {
            None => Token::Identifier(data),
            Some(keyword_token) => (*keyword_token).clone()
        }
    }

    fn chop_integer_token(&mut self) -> Token {
        let mut idx = 0;
        while idx < self.remaining_input.len() && self.remaining_input[idx].is_numeric() {
            idx += 1;
        }

        let integer_data_vector = self.remaining_input[..idx].to_vec();
        let integer_data_string: String = integer_data_vector.into_iter().collect();
        let integer_data = integer_data_string.parse::<i32>().unwrap();

        self.remaining_input = self.remaining_input[idx..].to_vec();

        Token::Integer(integer_data)
    }

    fn is_current_character_punctuation(&self) -> bool {
        match self.punctuation_to_token.get(&self.remaining_input[0].to_string()) {
            None => false,
            _ => true,
        }
    }

    fn chop_punctuation_token(&mut self) -> Token {
        let keyword_data;
        if self.remaining_input.len() > 1 && self.remaining_input[0] == '=' && self.remaining_input[1] == '=' {
            keyword_data = String::from("==");
            self.remaining_input = self.remaining_input[2..].to_vec();
        } else {
            keyword_data = self.remaining_input[0].to_string();
            self.remaining_input = self.remaining_input[1..].to_vec();
        }
        let punctuation_token = self.punctuation_to_token.get(&keyword_data).unwrap();
        return (*punctuation_token).clone();
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_on_arithmetic_expression() {
        let tokens = tokenize("(abc + 123) * 34;");
        assert_eq!(
            tokens.iter().map(|token| format!("{:?}", token)).collect::<Vec<String>>(),
            Vec::from([
                "<(, (>",
                "<identifier, abc>",
                "<+, +>",
                "<integer, 123>",
                "<), )>",
                "<*, *>",
                "<integer, 34>",
                "<;, ;>",
            ]),
        );
    }

    #[test]
    fn it_works_on_assignment_statement() {
        let tokens = tokenize("let x = 123 / 12;");
        assert_eq!(
            tokens.iter().map(|token| format!("{:?}", token)).collect::<Vec<String>>(),
            Vec::from([
                "<let, let>",
                "<identifier, x>",
                "<=, =>",
                "<integer, 123>",
                "</, />",
                "<integer, 12>",
                "<;, ;>",
            ]),
        );
    }

    #[test]
    fn it_works_on_equality_statement() {
        let tokens = tokenize("23 == 342 - 12");
        assert_eq!(
            tokens.iter().map(|token| format!("{:?}", token)).collect::<Vec<String>>(),
            Vec::from([
                "<integer, 23>",
                "<==, ==>",
                "<integer, 342>",
                "<-, ->",
                "<integer, 12>",
            ])
        )
    }

    #[test]
    fn it_works_on_if_else_statement() {
        let tokens = tokenize("if (true) { 34 } else { 43 }");
        assert_eq!(
            tokens.iter().map(|token| format!("{:?}", token)).collect::<Vec<String>>(),
            Vec::from([
                "<if, if>",
                "<(, (>",
                "<boolean, true>",
                "<), )>",
                "<{, {>",
                "<integer, 34>",
                "<}, }>",
                "<else, else>",
                "<{, {>",
                "<integer, 43>",
                "<}, }>",
            ])
        )
    }
}