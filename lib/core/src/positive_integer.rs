pub struct PositiveInteger64 {
    pub value: u64,
}

#[derive(Debug)]
pub enum PositiveIntCreationError {
    Zero,
}

impl PositiveInteger64 {
    pub fn new(value: u64) -> Result<Self, PositiveIntCreationError> {
        match value {
            0 => Err(PositiveIntCreationError::Zero),
            _ => Ok(Self { value }),
        }
    }
}