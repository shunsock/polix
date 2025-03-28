use crate::source_code::{Line, Position};
use std::collections::HashMap;

pub enum Constant {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    None,
}

pub enum Operator {
    // Boolean
    And,
    Equal,
    Not,
    NotEqual,
    Or,
    // Arithmetic
    Addition,
    Division,
    Modulus,
    Multiplication,
    Power,
    Subtraction,
}

pub struct GenericParameter {
    pub name: String,
}

pub enum PolixType {
    Integer,
    Float,
    String,
    Boolean,
    Void,
    List {
        content_type: Box<PolixType>,
        length: Option<usize>,
    },
    Struct(String),
    Option(Box<PolixType>),
    Result {
        ok: Box<PolixType>,
        error: Box<PolixType>,
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

pub struct Block {
    pub statements: Vec<Ast>,
    pub line: Line,
    pub position: Position,
}

pub enum Statement {
    Assign {
        assign_name: String,
        mutable: bool,
        global: bool,
        kind: PolixType,
        value: Box<Expression>,
        line: Line,
        position: Position,
    },
    Block(Block),
    DefineFunction {
        name: String,
        generics: Option<Vec<GenericParameter>>,
        arguments: HashMap<String, PolixType>,
        return_type: PolixType,
        global: bool,
        body: Block,
        line: Line,
        position: Position,
    },
    DefineStruct {
        name: String,
        generics: Option<Vec<GenericParameter>>,
        global: bool,
        mutable: bool,
        fields: Vec<(String, PolixType)>,
        line: Line,
        position: Position,
    },
    If {
        condition: Box<Expression>,
        true_branch: Block,
        false_branch: Option<Block>,
        line: Line,
        position: Position,
    },
    While {
        condition: Box<Expression>,
        body: Vec<Block>,
        line: Line,
        position: Position,
    },
    Loop {
        condition: Box<Expression>,
        body: Vec<Block>,
        line: Line,
        position: Position,
    },
    Switch {
        expression: Box<Expression>,
        cases: Vec<(Constant, Block)>,
        default: Option<Block>,
        line: Line,
        position: Position,
    },
}

pub enum Ast {
    Expression(Box<Expression>),
    Statement(Box<Statement>),
}
