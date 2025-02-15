use core::token::token_with_parsed_identifier::TokenWithParsedIdentifierType;
use core::source_code::{Line, Position};
use crate::transformer_error::{TransformerError, TransformerErrorKind};

pub(crate) fn parse_float(s: String, line: Line, position: Position) -> Result<TokenWithParsedIdentifierType, TransformerError> {
    match s.parse::<f64>() {
        Ok(parsed) => Ok(TokenWithParsedIdentifierType::LiteralFloat(parsed)),
        Err(_) => Err(
            TransformerError::new(
                TransformerErrorKind::InvalidFloatLiteralFound,
                line,
                position
            )
        ),
    }
}

pub(crate) fn parse_integer(s: String, line: Line, position: Position) -> Result<TokenWithParsedIdentifierType, TransformerError> {
    match s.parse::<i64>() {
        Ok(parsed) => Ok(TokenWithParsedIdentifierType::LiteralInteger(parsed)),
        Err(_) => Err(
            TransformerError::new(
                TransformerErrorKind::InvalidIntegerLiteralFound,
                line,
                position
            )
        ),
    }
}

pub(crate) fn parse_string(s: String, line: Line, position: Position) -> Result<TokenWithParsedIdentifierType, TransformerError> {
    match s.chars().all(|c| c.is_alphanumeric() || c == '_') {
        true => Ok(TokenWithParsedIdentifierType::LiteralString(s)),
        false => Err(
            TransformerError::new(
                TransformerErrorKind::InvalidStringLiteralFound,
                line,
                position
            )
        ),
    }
}