use crate::source_code::Line;
use crate::source_code::Position;

pub trait PolixErrorTrait {
    fn get_message(&self) -> String;
    fn get_line(&self) -> Line;
    fn get_position(&self) -> Position;
}