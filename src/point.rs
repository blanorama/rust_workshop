use crate::direction::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point2d {
    pub x: i32,
    pub y: i32,
}

impl Point2d {
    pub fn new(x: u16, y: u16) -> Self {
        Point2d {
            x: (x - (x % 2)).into(),
            y: y.into(),
        }
    }

    pub fn new_random_point(max_y: u16, max_x: u16) -> Point2d {
        Point2d::new(rand::random::<u16>() % max_x, rand::random::<u16>() % max_y)
    }

    pub fn move_by_direction(
        &mut self,
        max_y: u16,
        max_x: u16,
        direction: &Direction,
    ) -> Result<(), ()> {
        match direction {
            Direction::Left => {
                self.x = correct_boundary(self.x - 2, max_x)?;
            }
            Direction::Right => {
                self.x = correct_boundary(self.x + 2, max_x)?;
            }
            Direction::Up => {
                self.y = correct_boundary(self.y - 1, max_y)?;
            }
            Direction::Down => {
                self.y = correct_boundary(self.y + 1, max_y)?;
            }
        };
        Ok(())
    }
}

fn correct_boundary(value: i32, max: u16) -> Result<i32, ()> {
    if value <= 0 {
        return Err(());
    }

    if value >= max.into() {
        return Err(());
    }

    Ok(value)
}
