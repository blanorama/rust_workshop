use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(Direction::Up),
            's' => Ok(Direction::Down),
            'd' => Ok(Direction::Right),
            'a' => Ok(Direction::Left),
            _ => Err("Can't convert".into()),
        }
    }
}
