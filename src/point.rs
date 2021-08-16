use crate::direction::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point2d {
    pub x: i32,
    pub y: i32,
}

impl Point2d {
    pub fn new(x: i32, y: i32) -> Self {
        Point2d { x, y }
    }
}

fn correct_boundary(value: i32, max: i32) -> i32 {
    if value < 0 {
        return max;
    }

    if value > max {
        return 0;
    }

    value
}

impl Point2d {
    pub fn move_by_direction(&mut self, direction: &Direction, max_y: i32, max_x: i32) {
        match direction {
            Direction::Left => {
                self.x = correct_boundary(self.x - 1, max_x);
            }
            Direction::Right => {
                self.x = correct_boundary(self.x + 1, max_x);
            }
            Direction::Up => {
                self.y = correct_boundary(self.y - 1, max_y);
            }
            Direction::Down => {
                self.y = correct_boundary(self.y + 1, max_y);
            }
        };
    }
}
