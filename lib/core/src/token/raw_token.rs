use crate::source_code::Line;
use crate::source_code::Position;

#[derive(Debug, PartialEq, Clone)]
pub struct RawToken {
    pub token_type: TokenType,
    pub line: Line,
    pub position: Position,
}

impl RawToken {
    pub fn new(token_type: TokenType, line: Line, position: Position) -> RawToken {
        RawToken {
            token_type,
            line,
            position,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
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
