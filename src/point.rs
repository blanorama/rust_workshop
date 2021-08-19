use crate::direction::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point2d {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
}

impl Point2d {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Point2d { x, y, direction }
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
    pub fn move_by_direction(&mut self, max_y: i32, max_x: i32) {
        match self.direction {
            Direction::Left => {
                self.x = correct_boundary(self.x - 2, max_x);
            }
            Direction::Right => {
                self.x = correct_boundary(self.x + 2, max_x);
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
