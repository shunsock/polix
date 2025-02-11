use crate::source_code::Line;
use crate::source_code::Position;

#[derive(Debug, PartialEq, Clone)]
pub struct TokenPrepared {
    pub token_type: PreparedTokenType,
    pub line: Line,
    pub position: Position,
}

impl TokenPrepared {
    pub fn new(token_type: PreparedTokenType, line: Line, position: Position) -> TokenPrepared {
        TokenPrepared {
            token_type,
            line,
            position,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PreparedTokenType {
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
    Identifier(String),
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
    LiteralInteger(i32),
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
}
