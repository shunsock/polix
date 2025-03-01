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

pub(crate) fn parse_string_literal(
    s: String,
    line: Line,
    position: Position,
) -> Result<TokenWithParsedIdentifierType, IdentifierExtractorError> {
    let mut chars = s.chars();
    let first_char = chars.next();
    if first_char != Some('"') {
        return Err(IdentifierExtractorError::new(
            IdentifierExtractorErrorKind::InvalidStringLiteralFound,
            line,
            position,
        ));
    }
    let last_char = chars.last();
    if last_char != Some('"') {
        return Err(IdentifierExtractorError::new(
            IdentifierExtractorErrorKind::InvalidStringLiteralFound,
            line,
            position,
        ));
    }
    // remove the first and last character
    let s = s[1..s.len() - 1].to_string();
    Ok(TokenWithParsedIdentifierType::LiteralString(s))
}

pub(crate) fn parse_variable(
    s: String,
    line: Line,
    position: Position,
) -> Result<TokenWithParsedIdentifierType, IdentifierExtractorError> {
    match s.chars().all(|c| c.is_alphanumeric() || c == '_') {
        true => Ok(TokenWithParsedIdentifierType::Variable(s)),
        false => Err(IdentifierExtractorError::new(
            IdentifierExtractorErrorKind::InvalidStringLiteralFound,
            line,
            position,
        )),
    }
}
