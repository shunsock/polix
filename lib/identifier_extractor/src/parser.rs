use crate::identifier_extractor_error::{IdentifierExtractorError, IdentifierExtractorErrorKind};
use core::source_code::{Line, Position};
use core::token::token_with_parsed_identifier::TokenWithParsedIdentifierType;

pub(crate) fn parse_float(
    s: String,
    line: Line,
    position: Position,
) -> Result<TokenWithParsedIdentifierType, IdentifierExtractorError> {
    match s.parse::<f64>() {
        Ok(parsed) => Ok(TokenWithParsedIdentifierType::LiteralFloat(parsed)),
        Err(_) => Err(IdentifierExtractorError::new(
            IdentifierExtractorErrorKind::InvalidFloatLiteralFound,
            line,
            position,
        )),
    }
}

pub(crate) fn parse_integer(
    s: String,
    line: Line,
    position: Position,
) -> Result<TokenWithParsedIdentifierType, IdentifierExtractorError> {
    match s.parse::<i64>() {
        Ok(parsed) => Ok(TokenWithParsedIdentifierType::LiteralInteger(parsed)),
        Err(_) => Err(IdentifierExtractorError::new(
            IdentifierExtractorErrorKind::InvalidIntegerLiteralFound,
            line,
            position,
        )),
    }
}

pub(crate) fn parse_string(
    s: String,
    line: Line,
    position: Position,
) -> Result<TokenWithParsedIdentifierType, IdentifierExtractorError> {
    match s.chars().all(|c| c.is_alphanumeric() || c == '_') {
        true => Ok(TokenWithParsedIdentifierType::LiteralString(s)),
        false => Err(IdentifierExtractorError::new(
            IdentifierExtractorErrorKind::InvalidStringLiteralFound,
            line,
            position,
        )),
    }
}
