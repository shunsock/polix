use crate::source_code::{Line, Position};

struct Token {
    pub kind: TokenKind,
    pub line: Line,
    pub position: Position,
}

enum TokenKind {
    Control,
    Unrecognized(String),
    Literal(Literal),
    Type(Type),
    Keyword(Keyword),
}

enum Type {
    Primitive(PrimitiveType),
    Option,
    List,
}

enum PrimitiveType {
    Integer,
    Float,
    Boolean,
    String,
    Function,
    Struct,
}

enum Literal {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    String(String),
    None,
}

enum Keyword {
    Control(Control),
    Declaration(Declaration),
    BinaryOperation(BinaryOperation),
    Colon(Colon),
}

enum Control {
    If,
    Loop,
    Match,
}

enum Declaration {
    As,
    Bind,
    Define,
    Let,
    Module,
    New,
    Rebind,
    Use,
}

enum Colon {
    Colon,
    DoubleColon,
    SemiColon,
}

enum Paren {
    AngleBracketLeft,
    AngleBracketRight,
    BraceLeft,
    BraceRight,
    ParenLeft,
    ParenRight,
    SquareBracketLeft,
    SquareBracketRight,
}

enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    And,
    Or,
    Not,
}
