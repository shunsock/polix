#[derive(Debug, PartialEq, Clone)]
pub struct PositiveInteger32 {
    pub value: u32,
}

#[derive(Debug)]
pub enum PositiveIntCreationError {
    Zero,
}

impl PositiveInteger32 {
    pub fn new(value: u32) -> Result<Self, PositiveIntCreationError> {
        match value {
            0 => Err(PositiveIntCreationError::Zero),
            _ => Ok(Self { value }),
        }
    }
}
