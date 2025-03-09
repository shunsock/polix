use std::num::NonZero;

#[derive(Debug, PartialEq, Clone)]
pub struct PositiveIntCreationError {
    message: String,
}

impl PositiveIntCreationError {
    fn new() -> Self {
        Self {
            message: "Value must be greater than 0".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Line {
    pub number: NonZero<u32>,
}

impl Line {
    pub fn new(number: u32) -> Result<Self, PositiveIntCreationError> {
        match NonZero::new(number) {
            None => Err(PositiveIntCreationError::new()),
            Some(n) => Ok(Self { number: n }),
        }
    }

    pub fn increment(&self) -> Line {
        Line::new(self.number.get() + 1).unwrap()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
    pub number: NonZero<u32>,
}

impl Position {
    pub fn new(number: u32) -> Result<Self, PositiveIntCreationError> {
        match NonZero::new(number) {
            None => Err(PositiveIntCreationError::new()),
            Some(n) => Ok(Self { number: n }),
        }
    }

    pub fn increment(&self) -> Position {
        Position::new(self.number.get() + 1).unwrap()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SourceCodeCharacter {
    pub character: char,
    pub line: Line,
    pub position: Position,
}

impl SourceCodeCharacter {
    pub fn new(character: char, line: Line, position: Position) -> Self {
        Self {
            character,
            line,
            position,
        }
    }
}