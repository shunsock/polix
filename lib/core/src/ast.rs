use crate::source_code::{Line, Position};
use std::collections::HashMap;

pub enum Ast {
    Expression(Box<Expression>),
    Statement(Box<Statement>),
}

pub enum Expression {
    BinaryOperation {
        operator: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
        line: Line,
        position: Position,
    },
    Primary {
        value: Box<Primary>,
        line: Line,
        position: Position,
    },
    TypeCast {
        value: Box<Expression>,
        target_type: PolixType,
        line: Line,
        position: Position,
    },
    Return {
        value: Box<Expression>,
        line: Line,
        position: Position,
    },
    Match {
        expression: Box<Expression>,
        cases: Vec<(Constant, Vec<Expression>)>,
        default: Option<Vec<Expression>>,
        line: Line,
        position: Position,
    },
}

pub enum Primary {
    Constant {
        value: Constant,
        line: Line,
        position: Position,
    },
    FunctionCall {
        name: String,
        arguments: HashMap<String, PolixType>,
        generics: Option<Vec<GenericParameter>>,
        line: Line,
        position: Position,
    },
    VariableCall {
        name: String,
        generics: Option<Vec<GenericParameter>>,
        line: Line,
        position: Position,
    },
}

pub enum Operator {
    Add,
    And,
    Or,
    Not,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Power,
}

pub enum Constant {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    None,
}

pub enum Statement {
    LetVariable {
        name: String,
        mutable: bool,
        kind: PolixType,
        value: Box<Expression>,
        line: Line,
        position: Position,
    },
    DefineFunction {
        name: String,
        generics: Option<Vec<GenericParameter>>,
        arguments: HashMap<String, PolixType>,
        return_type: PolixType,
        body: Vec<Ast>,
        line: Line,
        position: Position,
    },
    DefineStruct {
        name: String,
        generics: Option<Vec<GenericParameter>>,
        mutable: bool,
        fields: Vec<(String, PolixType)>,
        line: Line,
        position: Position,
    },
    If {
        condition: Box<Ast>,
        true_branch: Vec<Ast>,
        false_branch: Option<Vec<Ast>>,
        line: Line,
        position: Position,
    },
    While {
        condition: Box<Ast>,
        body: Vec<Ast>,
        line: Line,
        position: Position,
    },
    Loop {
        condition: Box<Ast>,
        body: Vec<Ast>,
        line: Line,
        position: Position,
    },
}

pub enum PolixType {
    Integer,
    Float,
    String,
    Boolean,
    Void,
    List {
        content_type: Box<PolixType>,
        contents: Vec<PolixType>,
        length: Option<usize>,
    },
    Struct(String),
    Option(Box<PolixType>),
    Result {
        ok: Box<PolixType>,
        error: Box<PolixType>,
    },
}

pub struct GenericParameter {
    pub name: String,
}
