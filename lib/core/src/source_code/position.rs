use crate::positive_integer::PositiveInteger64;

pub struct Position {
    pub number: PositiveInteger64,
}

impl Position {
    pub fn new(number: PositiveInteger64) -> Self {
        Self { number }
    }
}
