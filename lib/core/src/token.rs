use crate::source_code::{Line, Position};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: Line,
    pub position: Position,
}

impl Token {
    pub fn new(kind: TokenKind, line: Line, position: Position) -> Token {
        Token {
            kind,
            line,
            position,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Control,
    Unrecognized(String),
    Literal(Literal),
    Type(Type),
    Keyword(Keyword),
    Identifier(Identifier),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primitive(PrimitiveType),
    Option,
    List,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveType {
    Integer,
    Float,
    Boolean,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Text(String),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Control(Control),
    Declaration(Declaration),
    BinaryOperation(BinaryOperation),
    Colon(Colon),
    Paren(Paren),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Control {
    If,
    Loop,
    Match,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    As,
    Define,
    Function,
    Let,
    Module,
    New,
    Return,
    Struct,
    Use,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Colon {
    Colon,
    DoubleColon,
    SemiColon,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Paren {
    AngleBracketLeft,
    AngleBracketRight,
    BraceLeft,
    BraceRight,
    ParenLeft,
    ParenRight,
    SquareBracketLeft,
    SquareBracketRight,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    DoubleEqual,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Identifier {
    // [A-Z][a-zA-Z]+
    Struct(String),
    // [a-z][a-z0-9_]+
    VariableOrFunction(String),
}
