use crate::positive_integer::{PositiveIntCreationError, PositiveInteger32};

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    pub number: PositiveInteger32,
}

impl Line {
    pub fn new(number: u32) -> Result<Self, PositiveIntCreationError> {
        Ok(Self {
            number: PositiveInteger32::new(number)?,
        })
    }

    pub fn increment(&self) -> Line {
        Line::new(self.number.value + 1).unwrap()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub number: PositiveInteger32,
}

impl Position {
    pub fn new(number: u32) -> Result<Self, PositiveIntCreationError> {
        Ok(Self {
            number: PositiveInteger32::new(number)?,
        })
    }

    pub fn increment(&self) -> Position {
        Position::new(self.number.value + 1).unwrap()
    }
}
