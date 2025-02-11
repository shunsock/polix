use crate::buffer::Buffer;
use core::token::raw_token::{RawToken, RawTokenType};

pub(super) fn generate(rest: String, buffer: Buffer) -> Option<RawToken> {
    let next_char: char = match rest.chars().next() {
        Some(next_char) => next_char,
        None => {
            let identifier: String = buffer.text;
            return Some(RawToken::new(
                RawTokenType::Identifier(identifier),
                buffer.start_line,
                buffer.start_position,
            ));
        }
    };
    let identifier: String = match next_char.is_whitespace() {
        true => buffer.text,
        false => return None,
    };
    Some(RawToken::new(
        RawTokenType::Identifier(identifier),
        buffer.start_line,
        buffer.start_position,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::source_code::{Line, Position};

    fn create_buffer_has_text_id() -> Buffer {
        Buffer {
            text: "id".to_string(),
            current_line: Line::new(1).unwrap(),
            current_position: Position::new(1).unwrap(),
            start_line: Line::new(1).unwrap(),
            start_position: Position::new(1).unwrap(),
        }
    }

    /// Test: When the first character of `rest` is whitespace,
    /// the function should return a token with the identifier equal to buffer.
    #[test]
    fn test_generate_with_whitespace_next() {
        // Given: a non-empty buffer, and a `rest` string starting with whitespace.
        let buffer: Buffer = create_buffer_has_text_id();
        let rest: String = " hello".to_string();

        // When: we call the generate function.
        let result: Option<RawToken> = generate(rest, buffer);

        // Then: a token should be produced with the identifier "id" = "id".
        let expected_token = RawToken::new(
            RawTokenType::Identifier("id".to_string()),
            Line::new(1).unwrap(),
            Position::new(1).unwrap(),
        );
        assert_eq!(result, Some(expected_token));
    }

    /// Test: If the first character of `rest` is not whitespace,
    /// the function should return None.
    #[test]
    fn test_generate_with_non_whitespace_next() {
        // Given: a non-empty buffer, and a `rest` string starting with a non-whitespace character.
        let buffer: Buffer = create_buffer_has_text_id();
        let rest: String = "non-whitespace".to_string();

        // When: we call the generate function.
        let result: Option<RawToken> = generate(rest, buffer);

        // Then: no token should be produced.
        assert_eq!(result, None);
    }

    /// Test: When `rest` is empty,
    /// the function should return a token with the identifier equal to buffer.
    #[test]
    fn test_generate_with_empty_rest() {
        // Given: a non-empty buffer, and an empty `rest` string.
        let buffer: Buffer = create_buffer_has_text_id();
        let rest: String = "".to_string();

        // When: we call the generate function.
        let result: Option<RawToken> = generate(rest, buffer);

        // Then: a token should be produced with the identifier "id".
        let expected_token: RawToken = RawToken::new(
            RawTokenType::Identifier("id".to_string()),
            Line::new(1).unwrap(),
            Position::new(1).unwrap(),
        );
        assert_eq!(result, Some(expected_token));
    }
}
