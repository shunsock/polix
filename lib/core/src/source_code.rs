use crate::positive_integer::{PositiveIntCreationError, PositiveInteger32};

pub struct Line {
    pub number: PositiveInteger32,
}

impl Line {
    pub fn new(number: u32) -> Result<Self, PositiveIntCreationError> {
        Ok(Self {
            number: PositiveInteger32::new(number)?,
        })
    }
}

pub struct Position {
    pub number: PositiveInteger32,
}

impl Position {
    pub fn new(number: u32) -> Result<Self, PositiveIntCreationError> {
        Ok(Self {
            number: PositiveInteger32::new(number)?,
        })
    }
}
