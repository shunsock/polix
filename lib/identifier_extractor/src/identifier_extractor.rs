use core::token::raw_token::RawToken;
use core::token::token_with_parsed_identifier::TokenWithParsedIdentifier;

pub struct IdentifierExtractor {
    pub tokens: Vec<RawToken>,
    pub tokens_with_parsed_identifier: Vec<TokenWithParsedIdentifier>,
}
