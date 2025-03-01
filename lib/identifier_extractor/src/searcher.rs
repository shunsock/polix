use core::token::token_with_parsed_identifier::TokenWithParsedIdentifierType;

pub(crate) fn search_keyword(s: String) -> Option<TokenWithParsedIdentifierType> {
    match s.as_str() {
        "and" => Some(TokenWithParsedIdentifierType::KeywordAnd),
        "as" => Some(TokenWithParsedIdentifierType::KeywordAs),
        "else" => Some(TokenWithParsedIdentifierType::KeywordElse),
        "if" => Some(TokenWithParsedIdentifierType::KeywordIf),
        "interface" => Some(TokenWithParsedIdentifierType::KeywordInterface),
        "let" => Some(TokenWithParsedIdentifierType::KeywordLet),
        "match" => Some(TokenWithParsedIdentifierType::KeywordMatch),
        "mut" => Some(TokenWithParsedIdentifierType::KeywordMutable),
        "not" => Some(TokenWithParsedIdentifierType::KeywordNot),
        "break" => Some(TokenWithParsedIdentifierType::KeywordOperationBreak),
        "continue" => Some(TokenWithParsedIdentifierType::KeywordOperationContinue),
        "for" => Some(TokenWithParsedIdentifierType::KeywordOperationFor),
        "loop" => Some(TokenWithParsedIdentifierType::KeywordOperationLoop),
        "return" => Some(TokenWithParsedIdentifierType::KeywordOperationReturn),
        "or" => Some(TokenWithParsedIdentifierType::KeywordOr),
        "bool" => Some(TokenWithParsedIdentifierType::KeywordTypeBoolean),
        "float" => Some(TokenWithParsedIdentifierType::KeywordTypeFloat),
        "fn" => Some(TokenWithParsedIdentifierType::KeywordTypeFunction),
        "hash" => Some(TokenWithParsedIdentifierType::KeywordTypeHash),
        "int" => Some(TokenWithParsedIdentifierType::KeywordTypeInteger),
        "list" => Some(TokenWithParsedIdentifierType::KeywordTypeList),
        "void" => Some(TokenWithParsedIdentifierType::KeywordTypeNone),
        "option" => Some(TokenWithParsedIdentifierType::KeywordTypeOption),
        "struct" => Some(TokenWithParsedIdentifierType::KeywordTypeStruct),
        "none" => Some(TokenWithParsedIdentifierType::LiteralNone),
        "true" => Some(TokenWithParsedIdentifierType::LiteralBoolean(true)),
        "false" => Some(TokenWithParsedIdentifierType::LiteralBoolean(false)),
        _ => None,
    }
}

pub(crate) fn search_number_literal(s: String) -> bool {
    s.chars().next().map_or(false, |c| c.is_ascii_digit())
}

pub(crate) fn search_string_literal(s: String) -> bool {
    // read the first character
    let mut chars = s.chars();
    let first_char = chars.next();
    // check if the first character is a double quote
    if first_char != Some('"') {
        return false;
    }
    // check if the last character is a double quote
    let last_char = chars.last();
    if last_char != Some('"') {
        return false;
    }

    true
}
