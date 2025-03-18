use core::source_code::{Line, Position};
use core::token::Literal::Text;
use core::token::Token;
use core::token::TokenKind;

pub fn recognize_literal_string(value: String, line: Line, position: Position) -> Option<Token> {
    let chars: Vec<char> = value.chars().collect();

    if chars.len() >= 2 && chars[0] == '"' && chars[chars.len() - 1] == '"' {
        let inner_content: String = chars[1..chars.len() - 1].iter().collect();

        Some(Token {
            kind: TokenKind::Literal(Text(inner_content)),
            line,
            position,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// # Test string literal recognition for a valid string
    ///
    /// ## Test Case:
    /// - input: "\"hello\""
    /// - expected: Token with TokenKind::Literal(Text("hello"))
    #[test]
    fn test_recognize_valid_string() {
        // Arrange
        let value = String::from("\"hello\"");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_literal_string(value, line, position);

        // Assert
        assert!(result.is_some());
        let token = result.unwrap();
        assert_eq!(token.kind, TokenKind::Literal(Text(String::from("hello"))));
        assert_eq!(token.line, line);
        assert_eq!(token.position, position);
    }

    /// # Test string literal recognition for an empty string
    ///
    /// ## Test Case:
    /// - input: "\"\""
    /// - expected: Token with TokenKind::Literal(Text(""))
    #[test]
    fn test_recognize_empty_string() {
        // Arrange
        let value = String::from("\"\"");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_literal_string(value, line, position);

        // Assert
        assert!(result.is_some());
        let token = result.unwrap();
        assert_eq!(token.kind, TokenKind::Literal(Text(String::from(""))));
        assert_eq!(token.line, line);
        assert_eq!(token.position, position);
    }

    /// # Test string literal recognition for a string with escaped quotes
    ///
    /// ## Test Case:
    /// - input: "\"hello\\\"world\""
    /// - expected: Token with TokenKind::Literal(Text("hello\\\"world"))
    #[test]
    fn test_recognize_string_with_escaped_quotes() {
        // Arrange
        let value = String::from("\"hello\\\"world\"");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_literal_string(value, line, position);

        // Assert
        assert!(result.is_some());
        let token = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Literal(Text(String::from("hello\\\"world")))
        );
        assert_eq!(token.line, line);
        assert_eq!(token.position, position);
    }

    /// # Test string literal recognition for an invalid string (no closing quote)
    ///
    /// ## Test Case:
    /// - input: "\"hello"
    /// - expected: None
    #[test]
    fn test_recognize_invalid_string_no_closing_quote() {
        // Arrange
        let value = String::from("\"hello");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_literal_string(value, line, position);

        // Assert
        assert!(result.is_none());
    }

    /// # Test string literal recognition for an invalid string (no opening quote)
    ///
    /// ## Test Case:
    /// - input: "hello\""
    /// - expected: None
    #[test]
    fn test_recognize_invalid_string_no_opening_quote() {
        // Arrange
        let value = String::from("hello\"");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_literal_string(value, line, position);

        // Assert
        assert!(result.is_none());
    }
}
