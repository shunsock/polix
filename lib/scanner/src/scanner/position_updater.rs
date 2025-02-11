use core::source_code::Position;

pub(super) fn update(c: char, p: Position) -> Position {
    match c {
        '\n' => Position::new(1).unwrap(),
        _ => p.increment(),
    }
}
