use core::source_code::Line;
use core::source_code::Position;
use core::polix_error_trait::PolixErrorTrait;

pub struct TransformerError {
    pub message: String,
    pub line: Line,
    pub position: Position,
}

impl PolixErrorTrait for TransformerError {
    fn get_message(&self) -> String {
        self.message.clone()
    }

    fn get_line(&self) -> Line {
        self.line.clone()
    }

    fn get_position(&self) -> Position {
        self.position.clone()
    }
}