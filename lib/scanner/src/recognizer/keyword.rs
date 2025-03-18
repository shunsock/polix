use core::source_code::{Line, Position};
use core::token::Token;
use core::token::TokenKind;
use core::token::{BinaryOperation, Control, Declaration, Keyword, Literal, PrimitiveType, Type};

pub fn recognize_keyword(value: String, line: Line, position: Position) -> Option<Token> {
    match value.as_str() {
        "true" => Some(Token::new(
            TokenKind::Literal(Literal::Boolean(true)),
            line,
            position,
        )),
        "false" => Some(Token::new(
            TokenKind::Literal(Literal::Boolean(false)),
            line,
            position,
        )),
        "none" => Some(Token::new(
            TokenKind::Literal(Literal::None),
            line,
            position,
        )),
        "int" => Some(Token::new(
            TokenKind::Type(Type::Primitive(PrimitiveType::Integer)),
            line,
            position,
        )),
        "float" => Some(Token::new(
            TokenKind::Type(Type::Primitive(PrimitiveType::Float)),
            line,
            position,
        )),
        "bool" => Some(Token::new(
            TokenKind::Type(Type::Primitive(PrimitiveType::Boolean)),
            line,
            position,
        )),
        "string" => Some(Token::new(
            TokenKind::Type(Type::Primitive(PrimitiveType::String)),
            line,
            position,
        )),
        "Option" => Some(Token::new(TokenKind::Type(Type::Option), line, position)),
        "List" => Some(Token::new(TokenKind::Type(Type::List), line, position)),
        "if" => Some(Token::new(
            TokenKind::Keyword(Keyword::Control(Control::If)),
            line,
            position,
        )),
        "loop" => Some(Token::new(
            TokenKind::Keyword(Keyword::Control(Control::Loop)),
            line,
            position,
        )),
        "match" => Some(Token::new(
            TokenKind::Keyword(Keyword::Control(Control::Match)),
            line,
            position,
        )),
        "as" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::As)),
            line,
            position,
        )),
        "define" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::Define)),
            line,
            position,
        )),
        "fn" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::Function)),
            line,
            position,
        )),
        "let" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::Let)),
            line,
            position,
        )),
        "module" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::Module)),
            line,
            position,
        )),
        "new" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::New)),
            line,
            position,
        )),
        "return" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::Return)),
            line,
            position,
        )),
        "struct" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::Struct)),
            line,
            position,
        )),
        "use" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::Use)),
            line,
            position,
        )),
        "==" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::DoubleEqual)),
            line,
            position,
        )),
        "!=" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::NotEqual)),
            line,
            position,
        )),
        ">" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::GreaterThan)),
            line,
            position,
        )),
        ">=" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(
                BinaryOperation::GreaterThanOrEqual,
            )),
            line,
            position,
        )),
        "<" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::LessThan)),
            line,
            position,
        )),
        "<=" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::LessThanOrEqual)),
            line,
            position,
        )),
        "&&" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::And)),
            line,
            position,
        )),
        "||" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::Or)),
            line,
            position,
        )),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// # Test keyword recognition for boolean literals
    ///
    /// ## Test Case:
    /// - input: "true"
    /// - expected: Token with TokenKind::Literal(Literal::Boolean(true))
    #[test]
    fn test_recognize_boolean_true() {
        // Arrange
        let value = String::from("true");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_keyword(value, line, position);

        // Assert
        assert!(result.is_some());
        let token = result.unwrap();
        assert_eq!(token.kind, TokenKind::Literal(Literal::Boolean(true)));
        assert_eq!(token.line, line);
        assert_eq!(token.position, position);
    }

    /// # Test keyword recognition for boolean literals
    ///
    /// ## Test Case:
    /// - input: "false"
    /// - expected: Token with TokenKind::Literal(Literal::Boolean(false))
    #[test]
    fn test_recognize_boolean_false() {
        // Arrange
        let value = String::from("false");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_keyword(value, line, position);

        // Assert
        assert!(result.is_some());
        let token = result.unwrap();
        assert_eq!(token.kind, TokenKind::Literal(Literal::Boolean(false)));
        assert_eq!(token.line, line);
        assert_eq!(token.position, position);
    }

    /// # Test keyword recognition for primitive types
    ///
    /// ## Test Case:
    /// - input: "int"
    /// - expected: Token with TokenKind::Type(Type::Primitive(PrimitiveType::Integer))
    #[test]
    fn test_recognize_primitive_type() {
        // Arrange
        let value = String::from("int");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_keyword(value, line, position);

        // Assert
        assert!(result.is_some());
        let token = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Type(Type::Primitive(PrimitiveType::Integer))
        );
        assert_eq!(token.line, line);
        assert_eq!(token.position, position);
    }

    /// # Test keyword recognition for control keywords
    ///
    /// ## Test Case:
    /// - input: "if"
    /// - expected: Token with TokenKind::Keyword(Keyword::Control(Control::If))
    #[test]
    fn test_recognize_control_keyword() {
        // Arrange
        let value = String::from("if");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_keyword(value, line, position);

        // Assert
        assert!(result.is_some());
        let token = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Keyword(Keyword::Control(Control::If))
        );
        assert_eq!(token.line, line);
        assert_eq!(token.position, position);
    }

    /// # Test keyword recognition for binary operations
    ///
    /// ## Test Case:
    /// - input: "&&"
    /// - expected: Token with TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::And))
    #[test]
    fn test_recognize_binary_operation() {
        // Arrange
        let value = String::from("&&");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_keyword(value, line, position);

        // Assert
        assert!(result.is_some());
        let token = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::And))
        );
        assert_eq!(token.line, line);
        assert_eq!(token.position, position);
    }

    /// # Test keyword recognition for unrecognized words
    ///
    /// ## Test Case:
    /// - input: "unknown"
    /// - expected: None
    #[test]
    fn test_recognize_unrecognized_word() {
        // Arrange
        let value = String::from("unknown");
        let line = Line::new(1).unwrap();
        let position = Position::new(1).unwrap();

        // Act
        let result = recognize_keyword(value, line, position);

        // Assert
        assert!(result.is_none());
    }
}
