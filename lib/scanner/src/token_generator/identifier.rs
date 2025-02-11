use core::token::{Token, TokenType};
use core::source_code::{ Line, Position };

pub(super) fn generate(
    rest: String,
    buffer_updated: String,
    line: Line,
    position: Position
) -> Option<Token> {
    let next_char: char = match rest.chars().next() {
        Some(next_char) => next_char,
        None => {
            let identifier: String = buffer_updated;
            return Some(Token::new(
                TokenType::Identifier(identifier),
                line,
                position,
            ))
        },
    };
    let identifier: String = match next_char.is_whitespace() {
        true => buffer_updated,
        false => return None,
    };
    Some(Token::new(
        TokenType::Identifier(identifier),
        line,
        position,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::source_code::{Line, Position};

    /// Test: When the first character of `rest` is whitespace,
    /// the function should return a token with the identifier equal to buffer.
    #[test]
    fn test_generate_with_whitespace_next() {
        // Given: a non-empty buffer, and a `rest` string starting with whitespace.
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();
        let buffer = "id".to_string();
        let rest = " hello".to_string();

        // When: we call the generate function.
        let result = generate(rest, buffer, line.clone(), position.clone());

        // Then: a token should be produced with the identifier "id" = "id".
        let expected_token =
            Token::new(TokenType::Identifier("id".to_string()), line, position);
        assert_eq!(result, Some(expected_token));
    }

    /// Test: If the first character of `rest` is not whitespace,
    /// the function should return None.
    #[test]
    fn test_generate_with_non_whitespace_next() {
        // Given: a non-empty buffer, and a `rest` string starting with a non-whitespace character.
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();
        let buffer = "id".to_string();
        let rest = "non-whitespace".to_string();

        // When: we call the generate function.
        let result = generate(rest, buffer, line, position);

        // Then: no token should be produced.
        assert_eq!(result, None);
    }

    /// Test: When `rest` is empty,
    /// the function should return a token with the identifier equal to buffer.
    #[test]
    fn test_generate_with_empty_rest() {
        // Given: a non-empty buffer, and an empty `rest` string.
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();
        let buffer = "id".to_string();
        let rest = "".to_string();

        // When: we call the generate function.
        let result = generate(rest, buffer, line.clone(), position.clone());

        // Then: a token should be produced with the identifier "id".
        let expected_token =
            Token::new(TokenType::Identifier("id".to_string()), line, position);
        assert_eq!(result, Some(expected_token));
    }
}
