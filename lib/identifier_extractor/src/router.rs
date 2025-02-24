use crate::identifier_extractor_error::IdentifierExtractorError;
use crate::parser::{parse_float, parse_integer, parse_string};
use crate::searcher::{search_keyword, search_number};
use core::source_code::{Line, Position};
use core::token::token_with_parsed_identifier::TokenWithParsedIdentifierType;

pub fn route(
    s: String,
    l: Line,
    p: Position,
) -> Result<TokenWithParsedIdentifierType, IdentifierExtractorError> {
    match search_keyword(s.clone()) {
        Some(t) => return Ok(t),
        None => (),
    }

    match search_number(s.clone()) {
        true => {
            return if s.contains('.') {
                parse_float(s, l, p)
            } else {
                parse_integer(s, l, p)
            }
        }
        false => (),
    }

    parse_string(s, l, p)
}
