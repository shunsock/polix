use crate::identifier_extractor_error::IdentifierExtractorError;
use crate::parser::{parse_float, parse_integer, parse_string};
use crate::searcher::{search_keyword, search_number};
use core::source_code::{Line, Position};
use core::token::raw_token::{RawToken, RawTokenType};
use core::token::token_with_parsed_identifier::TokenWithParsedIdentifierType;

pub fn route(token: RawToken) -> Result<TokenWithParsedIdentifierType, IdentifierExtractorError> {
    match token.token_type {
        RawTokenType::Identifier(string) => route_identifier(string, token.line, token.position),
        _ => Ok(route_others(token.token_type)),
    }
}

pub fn route_identifier(
    str: String,
    line: Line,
    position: Position,
) -> Result<TokenWithParsedIdentifierType, IdentifierExtractorError> {
    if let Some(t) = search_keyword(str.clone()) {
        return Ok(t);
    }

    match search_number(str.clone()) {
        true => (),
        false => return parse_string(str, line, position),
    }

    match str.contains('.') {
        true => parse_float(str, line, position),
        false => parse_integer(str, line, position),
    }
}

fn route_others(token_type: RawTokenType) -> TokenWithParsedIdentifierType {
    match token_type {
        RawTokenType::AngleLeft => TokenWithParsedIdentifierType::DelimiterAngleLeft,
        RawTokenType::AngleRight => TokenWithParsedIdentifierType::DelimiterAngleRight,
        RawTokenType::BraceLeft => TokenWithParsedIdentifierType::DelimiterBraceLeft,
        RawTokenType::BraceRight => TokenWithParsedIdentifierType::DelimiterBraceRight,
        RawTokenType::BracketLeft => TokenWithParsedIdentifierType::DelimiterBracketLeft,
        RawTokenType::BracketRight => TokenWithParsedIdentifierType::DelimiterBracketRight,
        RawTokenType::ParenthesisLeft => TokenWithParsedIdentifierType::DelimiterParenthesisLeft,
        RawTokenType::ParenthesisRight => TokenWithParsedIdentifierType::DelimiterParenthesisRight,
        RawTokenType::Colon => TokenWithParsedIdentifierType::SeparatorColon,
        RawTokenType::Comma => TokenWithParsedIdentifierType::SeparatorComma,
        RawTokenType::Dot => TokenWithParsedIdentifierType::SeparatorDot,
        RawTokenType::Percent => TokenWithParsedIdentifierType::OperatorPercent,
        RawTokenType::Plus => TokenWithParsedIdentifierType::OperatorPlus,
        RawTokenType::Semicolon => TokenWithParsedIdentifierType::SeparatorSemicolon,
        _ => panic!("Invalid token type"),
    }
}

#[cfg(test)]
mod tests {
    use crate::identifier_extractor_error::IdentifierExtractorErrorKind;
    use super::*;
    #[test]
    /// test route identifier number
    fn test_route_identifier_number() {
        // Arrange
        let raw_token: RawToken = RawToken::new(
            RawTokenType::Identifier("123".to_string()),
            Line::new(1).unwrap(),
            Position::new(1).unwrap(),
        );

        // Act
        let actual: Result<TokenWithParsedIdentifierType, IdentifierExtractorError> =
            route(raw_token);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(
            actual.unwrap(),
            TokenWithParsedIdentifierType::LiteralInteger(123)
        );
    }
    #[test]
    /// test route identifier float
    fn test_route_identifier_float() {
        // Arrange
        let raw_token: RawToken = RawToken::new(
            RawTokenType::Identifier("123.45".to_string()),
            Line::new(1).unwrap(),
            Position::new(1).unwrap(),
        );

        // Act
        let actual: Result<TokenWithParsedIdentifierType, IdentifierExtractorError> =
            route(raw_token);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(
            actual.unwrap(),
            TokenWithParsedIdentifierType::LiteralFloat(123.45)
        );
    }
    #[test]
    /// test route identifier string
    fn test_route_identifier_string() {
        // Arrange
        let raw_token: RawToken = RawToken::new(
            RawTokenType::Identifier("abcABC_".to_string()),
            Line::new(1).unwrap(),
            Position::new(1).unwrap(),
        );

        // Act
        let actual: Result<TokenWithParsedIdentifierType, IdentifierExtractorError> =
            route(raw_token);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(
            actual.unwrap(),
            TokenWithParsedIdentifierType::LiteralString("abcABC_".to_string())
        );
    }

    #[test]
    /// test route identifier keyword
    fn test_route_identifier_keyword() {
        // Arrange
        let raw_token: RawToken = RawToken::new(
            RawTokenType::Identifier("if".to_string()),
            Line::new(1).unwrap(),
            Position::new(1).unwrap(),
        );

        // Act
        let actual: Result<TokenWithParsedIdentifierType, IdentifierExtractorError> =
            route(raw_token);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), TokenWithParsedIdentifierType::KeywordIf);
    }

    #[test]
    /// test route others
    fn test_route_others() {
        // Arrange
        let angle_left: RawToken = RawToken::new(
            RawTokenType::AngleLeft,
            Line::new(1).unwrap(),
            Position::new(1).unwrap(),
        );

        // Act
        let actual: Result<TokenWithParsedIdentifierType, IdentifierExtractorError> =
            route(angle_left);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(
            actual.unwrap(),
            TokenWithParsedIdentifierType::DelimiterAngleLeft
        );
    }

    #[test]
    /// test fail route (invalid number)
    fn test_fail_route_invalid_integer() {
        // Arrange
        let raw_token: RawToken = RawToken::new(
            RawTokenType::Identifier("123a".to_string()),
            Line::new(1).unwrap(),
            Position::new(1).unwrap(),
        );

        // Act
        let actual: Result<TokenWithParsedIdentifierType, IdentifierExtractorError> =
            route(raw_token);

        // Assert
        assert!(actual.is_err());
        assert_eq!(
            actual.unwrap_err().error_kind,
            IdentifierExtractorErrorKind::InvalidIntegerLiteralFound
        );
    }

    #[test]
    /// test fail route (invalid float)
    fn test_fail_route_invalid_float() {
        // Arrange
        let raw_token: RawToken = RawToken::new(
            RawTokenType::Identifier("123.45.6".to_string()),
            Line::new(1).unwrap(),
            Position::new(1).unwrap(),
        );

        // Act
        let actual: Result<TokenWithParsedIdentifierType, IdentifierExtractorError> =
            route(raw_token);

        // Assert
        assert!(actual.is_err());
        assert_eq!(
            actual.unwrap_err().error_kind,
            IdentifierExtractorErrorKind::InvalidFloatLiteralFound
        );
    }
}
