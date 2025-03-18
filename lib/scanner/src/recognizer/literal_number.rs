use core::source_code::{Line, Position};
use core::token::Token;
use core::token::TokenKind;

use core::token::Literal::{
    Integer,
    Float,
};

pub fn recognize_literal_number(value: String, line: Line, position: Position) -> Option<Token> {
    if let Ok(int_value) = value.parse::<i64>() {
        Some(Token {
            kind: TokenKind::Literal(Integer(int_value)),
            line,
            position,
        })
    } else if let Ok(float_value) = value.parse::<f64>() {
        Some(Token {
            kind: TokenKind::Literal(Float(float_value)),
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
    use core::token::Literal;  // Added this import for the tests

    /// # Test number literal recognition for an integer
    ///
    /// ## Test Case:
    /// - input: "42"
    /// - expected: Token with TokenKind::Literal(Integer(42))
    #[test]
    fn test_recognize_integer() {
        // Arrange
        let value = String::from("42");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_literal_number(value, line, position);

        // Assert
        assert!(result.is_some());
        let token = result.unwrap();
        assert_eq!(token.kind, TokenKind::Literal(Integer(42)));
        assert_eq!(token.line, line);
        assert_eq!(token.position, position);
    }

    /// # Test number literal recognition for a negative integer
    ///
    /// ## Test Case:
    /// - input: "-42"
    /// - expected: Token with TokenKind::Literal(Integer(-42))
    #[test]
    fn test_recognize_negative_integer() {
        // Arrange
        let value = String::from("-42");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_literal_number(value, line, position);

        // Assert
        assert!(result.is_some());
        let token = result.unwrap();
        assert_eq!(token.kind, TokenKind::Literal(Integer(-42)));
        assert_eq!(token.line, line);
        assert_eq!(token.position, position);
    }

    /// # Test number literal recognition for zero
    ///
    /// ## Test Case:
    /// - input: "0"
    /// - expected: Token with TokenKind::Literal(Integer(0))
    #[test]
    fn test_recognize_zero() {
        // Arrange
        let value = String::from("0");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_literal_number(value, line, position);

        // Assert
        assert!(result.is_some());
        let token = result.unwrap();
        assert_eq!(token.kind, TokenKind::Literal(Integer(0)));
        assert_eq!(token.line, line);
        assert_eq!(token.position, position);
    }

    /// # Test number literal recognition for a float
    ///
    /// ## Test Case:
    /// - input: "3.14"
    /// - expected: Token with TokenKind::Literal(Float(3.14))
    #[test]
    fn test_recognize_float() {
        // Arrange
        let value = String::from("3.14");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_literal_number(value, line, position);

        // Assert
        assert!(result.is_some());
        let token = result.unwrap();
        if let TokenKind::Literal(Literal::Float(f)) = token.kind {
            assert!((f - 3.14).abs() < f64::EPSILON);
        } else {
            panic!("Expected Float, got {:?}", token.kind);
        }
        assert_eq!(token.line, line);
        assert_eq!(token.position, position);
    }

    /// # Test number literal recognition for a negative float
    ///
    /// ## Test Case:
    /// - input: "-3.14"
    /// - expected: Token with TokenKind::Literal(Float(-3.14))
    #[test]
    fn test_recognize_negative_float() {
        // Arrange
        let value = String::from("-3.14");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_literal_number(value, line, position);

        // Assert
        assert!(result.is_some());
        let token = result.unwrap();
        if let TokenKind::Literal(Literal::Float(f)) = token.kind {
            assert!((f - (-3.14)).abs() < f64::EPSILON);
        } else {
            panic!("Expected Float, got {:?}", token.kind);
        }
        assert_eq!(token.line, line);
        assert_eq!(token.position, position);
    }

    /// # Test number literal recognition for a non-number
    ///
    /// ## Test Case:
    /// - input: "abc"
    /// - expected: None
    #[test]
    fn test_recognize_non_number() {
        // Arrange
        let value = String::from("abc");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_literal_number(value, line, position);

        // Assert
        assert!(result.is_none());
    }
}
