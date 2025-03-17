use core::source_code::{Line, Position};
use core::token::Token;
use core::token::TokenKind;

pub struct Tokenizer {
    pub rest: Vec<Token>,
    pub processed: Vec<Token>,
}

impl Tokenizer {
    pub fn new(rest: Vec<Token>) -> Self {
        Tokenizer {
            rest,
            processed: Vec::new(),
        }
    }

    pub fn get_processed(&self) -> Vec<Token> {
        self.processed.clone()
    }

    pub fn tokenize(self) -> Self {
        let mut rest = self.rest.clone();
        let mut processed = self.processed.clone();

        while !rest.is_empty() {
            let t = rest.remove(0);
            match t.kind {
                TokenKind::Unrecognized(value) => {
                    let first_char = value.clone().chars().next().unwrap();
                    if first_char.is_whitespace() || value == "\n" {
                        continue;
                    }
                    let token = Tokenizer::read_token(&mut rest, value, t.line, t.position);
                    processed.push(token);
                }
                _ => {
                    processed.push(t);
                }
            }
        }

        Tokenizer { rest, processed }
    }

    fn read_token(
        v: &mut Vec<Token>,
        start_string: String,
        start_line: Line,
        start_position: Position,
    ) -> Token {
        let mut token_string = start_string;

        while !v.is_empty() {
            let t = v.remove(0);
            match t.kind {
                TokenKind::Unrecognized(value) => {
                    if let Some(first_char) = value.chars().next() {
                        if first_char.is_whitespace() || value == "\n" {
                            break;
                        }
                    }
                    token_string.push_str(&value);
                }
                _ => {
                    break;
                }
            }
        }

        Token {
            kind: TokenKind::Unrecognized(token_string),
            line: start_line,
            position: start_position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::source_code::{Line, Position};
    use core::token::TokenKind;

    #[test]
    /// Tokenize a string without any special characters
    ///
    /// ex. "abc" -> "abc"
    fn test_tokenize_string_without_special_characters() {
        // Arrange
        let token_queue = vec![
            Token::new(
                TokenKind::Unrecognized("a".to_string()),
                Line::new(1).unwrap(),
                Position::new(1).unwrap(),
            ),
            Token::new(
                TokenKind::Unrecognized("b".to_string()),
                Line::new(1).unwrap(),
                Position::new(2).unwrap(),
            ),
            Token::new(
                TokenKind::Unrecognized("c".to_string()),
                Line::new(1).unwrap(),
                Position::new(3).unwrap(),
            ),
        ];

        // Act
        let tokenizer: Tokenizer = Tokenizer::new(token_queue).tokenize();
        let processed: Vec<Token> = tokenizer.get_processed();

        // Assert
        assert_eq!(processed.len(), 1);
        assert_eq!(
            processed[0],
            Token::new(
                TokenKind::Unrecognized("abc".to_string()),
                Line::new(1).unwrap(),
                Position::new(1).unwrap(),
            )
        );
    }

    #[test]
    /// Tokenize a string with prefix whitespace
    ///
    /// ex. " abc" -> "abc"
    fn test_tokenize_string_with_prefix_whitespace() {
        // Arrange
        let token_queue = vec![
            Token::new(
                TokenKind::Unrecognized(" ".to_string()),
                Line::new(1).unwrap(),
                Position::new(1).unwrap(),
            ),
            Token::new(
                TokenKind::Unrecognized("a".to_string()),
                Line::new(1).unwrap(),
                Position::new(2).unwrap(),
            ),
            Token::new(
                TokenKind::Unrecognized("b".to_string()),
                Line::new(1).unwrap(),
                Position::new(3).unwrap(),
            ),
            Token::new(
                TokenKind::Unrecognized("c".to_string()),
                Line::new(1).unwrap(),
                Position::new(4).unwrap(),
            ),
        ];

        // Act
        let tokenizer: Tokenizer = Tokenizer::new(token_queue).tokenize();
        let processed: Vec<Token> = tokenizer.get_processed();

        // Assert
        assert_eq!(processed.len(), 1);
        assert_eq!(
            processed[0],
            Token::new(
                TokenKind::Unrecognized("abc".to_string()),
                Line::new(1).unwrap(),
                Position::new(2).unwrap(),
            )
        );
    }

    #[test]
    /// Tokenize a string with postfix whitespace
    ///
    /// ex. "abc " -> "abc"
    fn test_tokenize_string_with_postfix_whitespace() {
        // Arrange
        let token_queue = vec![
            Token::new(
                TokenKind::Unrecognized("a".to_string()),
                Line::new(1).unwrap(),
                Position::new(1).unwrap(),
            ),
            Token::new(
                TokenKind::Unrecognized("b".to_string()),
                Line::new(1).unwrap(),
                Position::new(2).unwrap(),
            ),
            Token::new(
                TokenKind::Unrecognized("c".to_string()),
                Line::new(1).unwrap(),
                Position::new(3).unwrap(),
            ),
            Token::new(
                TokenKind::Unrecognized(" ".to_string()),
                Line::new(1).unwrap(),
                Position::new(4).unwrap(),
            ),
        ];

        // Act
        let tokenizer: Tokenizer = Tokenizer::new(token_queue).tokenize();
        let processed: Vec<Token> = tokenizer.get_processed();

        // Assert
        assert_eq!(processed.len(), 1);
        assert_eq!(
            processed[0],
            Token::new(
                TokenKind::Unrecognized("abc".to_string()),
                Line::new(1).unwrap(),
                Position::new(1).unwrap(),
            )
        );
    }
}
