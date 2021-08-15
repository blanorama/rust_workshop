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
    pub fn move_by_direction(
        &mut self,
        direction: &Direction,
        amount: i32,
        max_y: i32,
        max_x: i32,
    ) {
        match direction {
            Direction::Left => {
                self.x = correct_boundary(self.x - amount, max_x);
            }
            Direction::Right => {
                self.x = correct_boundary(self.x + amount, max_x);
            }
            Direction::Up => {
                self.y = correct_boundary(self.y + amount, max_y);
            }
            Direction::Down => {
                self.y = correct_boundary(self.y - amount, max_y);
            }
        };
    }
}

impl Point2d {
    pub fn path_length(&self, other: &Point2d) -> u32 {
        f32::sqrt(((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32).round() as u32
    }
}

pub fn is_point_on_strait_line(line_start: &Point2d, line_end: &Point2d, point: &Point2d) -> bool {
    println!(
        "{}",
        line_start.path_length(point) + line_end.path_length(point)
    );
    println!("{}", line_start.path_length(line_end));
    line_start.path_length(point) + line_end.path_length(point) == line_start.path_length(line_end)
}

#[test]
fn test_is_point_on_strait_line() {
    assert_eq!(
        is_point_on_strait_line(
            &Point2d::new(10, 10),
            &Point2d::new(20, 10),
            &Point2d::new(15, 10)
        ),
        true
    );
    assert_eq!(
        is_point_on_strait_line(
            &Point2d::new(20, 10),
            &Point2d::new(10, 10),
            &Point2d::new(15, 10)
        ),
        true
    );
    assert_eq!(
        is_point_on_strait_line(
            &Point2d::new(10, 10),
            &Point2d::new(10, 20),
            &Point2d::new(10, 15)
        ),
        true
    );
    assert_eq!(
        is_point_on_strait_line(
            &Point2d::new(10, 20),
            &Point2d::new(10, 10),
            &Point2d::new(10, 15)
        ),
        true
    );
    assert_eq!(
        is_point_on_strait_line(
            &Point2d::new(10, 10),
            &Point2d::new(20, 10),
            &Point2d::new(21, 10)
        ),
        false
    );
    assert_eq!(
        is_point_on_strait_line(
            &Point2d::new(20, 10),
            &Point2d::new(10, 10),
            &Point2d::new(9, 10)
        ),
        false
    );
    assert_eq!(
        is_point_on_strait_line(
            &Point2d::new(10, 10),
            &Point2d::new(20, 10),
            &Point2d::new(15, 9)
        ),
        false
    );
    assert_eq!(
        is_point_on_strait_line(
            &Point2d::new(20, 10),
            &Point2d::new(10, 10),
            &Point2d::new(15, 9)
        ),
        false
    );
}
