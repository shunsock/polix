use core::source_code::{Line, Position};
use core::token::Token;
use core::token::TokenKind;
use regex::Regex;
use core::token::Identifier::{Struct, VariableOrFunction};


pub fn recognize_identifier(value: String, line: Line, position: Position) -> Option<Token> {
    let struct_regex = Regex::new(r"^[A-Z][a-zA-Z]+$").unwrap();
    let variable_of_function_regex = Regex::new(r"^[a-z][a-z0-9_]+$").unwrap();

    if struct_regex.is_match(&value) {
        Some(Token {
            kind: TokenKind::Identifier(Struct(value)),
            line,
            position,
        })
    } else if variable_of_function_regex.is_match(&value) {
        Some(Token {
            kind: TokenKind::Identifier(VariableOrFunction(value)),
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

    /// # Test identifier recognition for a struct identifier
    ///
    /// ## Test Case:
    /// - input: "Person"
    /// - expected: Token with TokenKind::Identifier(Struct("Person"))
    #[test]
    fn test_recognize_struct_identifier() {
        // Arrange
        let value = String::from("Person");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_identifier(value.clone(), line, position);

        // Assert
        assert!(result.is_some());
        let token = result.unwrap();
        assert_eq!(token.kind, TokenKind::Identifier(Struct(value)));
        assert_eq!(token.line, line);
        assert_eq!(token.position, position);
    }

    /// # Test identifier recognition for a variable or function identifier
    ///
    /// ## Test Case:
    /// - input: "user_name"
    /// - expected: Token with TokenKind::Identifier(VariableOrFunction("user_name"))
    #[test]
    fn test_recognize_variable_identifier() {
        // Arrange
        let value = String::from("user_name");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_identifier(value.clone(), line, position);

        // Assert
        assert!(result.is_some());
        let token = result.unwrap();
        assert_eq!(token.kind, TokenKind::Identifier(VariableOrFunction(value)));
        assert_eq!(token.line, line);
        assert_eq!(token.position, position);
    }

    /// # Test identifier recognition for an invalid identifier
    ///
    /// ## Test Case:
    /// - input: "123abc"
    /// - expected: None
    #[test]
    fn test_recognize_invalid_identifier_starting_with_number() {
        // Arrange
        let value = String::from("123abc");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_identifier(value, line, position);

        // Assert
        assert!(result.is_none());
    }

    /// # Test identifier recognition for an invalid identifier with special characters
    ///
    /// ## Test Case:
    /// - input: "user@name"
    /// - expected: None
    #[test]
    fn test_recognize_invalid_identifier_with_special_chars() {
        // Arrange
        let value = String::from("user@name");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_identifier(value, line, position);

        // Assert
        assert!(result.is_none());
    }
}
