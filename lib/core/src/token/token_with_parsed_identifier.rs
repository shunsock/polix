use crate::source_code::Line;
use crate::source_code::Position;

#[derive(Debug, PartialEq, Clone)]
pub struct TokenWithParsedIdentifier {
    pub token_type: TokenWithParsedIdentifierType,
    pub line: Line,
    pub position: Position,
}

impl TokenWithParsedIdentifier {
    pub fn new(token_type: TokenWithParsedIdentifierType, line: Line, position: Position) -> TokenWithParsedIdentifier {
        TokenWithParsedIdentifier {
            token_type,
            line,
            position,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenWithParsedIdentifierType {
    DelimiterAngleLeft,
    DelimiterAngleRight,
    DelimiterBraceLeft,
    DelimiterBraceRight,
    DelimiterBracketLeft,
    DelimiterBracketRight,
    DelimiterParenthesisLeft,
    DelimiterParenthesisRight,
    DoubleEqual,
    DoubleSlash,
    Eof,
    KeywordAnd,
    KeywordAs,
    KeywordElse,
    KeywordIf,
    KeywordInterface,
    KeywordLet,
    KeywordMatch,
    KeywordMutable,
    KeywordNot,
    KeywordOperationBreak,
    KeywordOperationContinue,
    KeywordOperationFor,
    KeywordOperationLoop,
    KeywordOperationReturn,
    KeywordOr,
    KeywordTypeBoolean,
    KeywordTypeFloat,
    KeywordTypeFunction,
    KeywordTypeHash,
    KeywordTypeInteger,
    KeywordTypeList,
    KeywordTypeNone,
    KeywordTypeOption,
    KeywordTypeStruct,
    LiteralBoolean(bool),
    LiteralCharacter(char),
    LiteralFloat(f64),
    LiteralInteger(i64),
    LiteralNone,
    LiteralString(String),
    SeparatorColon,
    SeparatorComma,
    SeparatorDot,
    SeparatorSemicolon,
    SingleAsterisk,
    SingleEqual,
    SingleMinus,
    SinglePercent,
    SinglePlus,
    SingleSlash,
    Variable(String),
}
