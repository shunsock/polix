mod token_generator;

use core::source_code::{Line, Position};
use core::token::Token;

#[derive(Clone, Debug)]
pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub buffer: String,
    pub line: Line,
    pub position: Position,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            buffer: String::new(),
            line: Line::new(1).unwrap(),
            position: Position::new(1).unwrap(),
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.source.is_empty()
    }
}


fn scan(s: Scanner) -> Scanner {
    match s.clone().is_at_end() {
        true => s.clone(),
        false => {
            // calling unwrap() is safe here because we know that the string is not empty
            let first_char: char = s.source.chars().next().unwrap();
            let rest_characters: String = s.source.chars().skip(1).collect();
            let buffer_updated: String = format!("{}{}", s.buffer, first_char);
            let token: Option<Token> = token_generator::generate_token(
                first_char,
                rest_characters.clone(),
                s.buffer.clone(),
                s.line.clone(),
                s.position.clone(),
            );
            match token {
                Some(t) => {
                    let tokens_updated: Vec<Token> = s.tokens
                        .into_iter()
                        .chain(std::iter::once(t))
                        .collect();

                    let s_ = Scanner {
                        source: rest_characters,
                        tokens: tokens_updated,
                        buffer: buffer_updated,
                        line: s.line,
                        position: s.position,
                    };
                    scan(s_)
                },
                None => {
                    let s_ = Scanner {
                        source: rest_characters,
                        tokens: s.tokens,
                        buffer: buffer_updated,
                        line: s.line,
                        position: s.position,
                    };
                    scan(s_)
                }
            }
        }
    }
}
