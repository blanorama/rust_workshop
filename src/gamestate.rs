use std::{
    collections::LinkedList,
    convert::{TryFrom, TryInto},
    io::Stdout,
};

use termion::raw::RawTerminal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point2d {
    x: i32,
    y: i32,
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
    fn move_by_direction(&mut self, direction: &Direction, max_y: i32, max_x: i32) {
        match direction {
            Direction::Left => {
                self.x = correct_boundary(self.x - 1, max_x);
            }
            Direction::Right => {
                self.x = correct_boundary(self.x + 1, max_x);
            }
            Direction::Up => {
                self.y = correct_boundary(self.y + 1, max_y);
            }
            Direction::Down => {
                self.y = correct_boundary(self.y - 1, max_y);
            }
        };
    }
}

impl Point2d {
    pub fn path_length(&self, other: Point2d) -> u32 {
        f32::sqrt((self.x + other.x + self.y + other.y) as f32).round() as u32
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
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

#[derive(Debug)]
pub struct Snake {
    edges_position: LinkedList<Point2d>,
    edges_directions: LinkedList<Direction>,
    length: u32,
    current_direction: Direction,
    head_position: Point2d,
    tail_position: Point2d,
}

impl Snake {
    fn new() -> Self {
        Snake {
            edges_position: LinkedList::new(),
            edges_directions: LinkedList::new(),
            length: 1,
            current_direction: Direction::Right,
            head_position: Point2d::new(0, 0),
            tail_position: Point2d::new(0, 0),
        }
    }

    fn update(&mut self, new_direction: Option<Direction>, max_y: i32, max_x: i32) {
        if new_direction.is_some() {
            let new_direction = new_direction.unwrap();
            if self.current_direction.ne(&new_direction) {
                if self.length > 2 {
                    self.edges_position.push_front(self.head_position.clone());
                    self.edges_directions
                        .push_front(self.current_direction.clone());
                }
                self.current_direction = new_direction;
            }
        }

        self.head_position
            .move_by_direction(&self.current_direction, max_y, max_x);

        self.tail_position.move_by_direction(
            &self
                .edges_directions
                .back()
                .unwrap_or(&self.current_direction),
            max_y,
            max_x,
        );

        if self.edges_position.len() > 0
            && self.tail_position.eq(self.edges_position.back().unwrap())
        {
            self.edges_position.pop_back();
            self.edges_directions.pop_back();
        }
    }

    fn draw(&self, x: i32, y: i32) -> bool {
        let point = &Point2d::new(x, y);
        if self.edges_directions.len() == 0 {
            if is_point_on_strait_line(&self.head_position, &self.tail_position, point) {
                return true;
            }
        } else {
            let mut iter = self.edges_position.iter();
            let mut line_start_option = Some(&self.head_position);
            let mut line_end_option = iter.next();

            while let Some(line_end) = line_end_option {
                let line_start = line_start_option.unwrap();
                if is_point_on_strait_line(line_start, line_end, point) {
                    return true;
                }

                line_start_option = line_end_option;
                line_end_option = iter.next();
            }
        }
        return false;
    }
}

fn is_point_on_strait_line(line_start: &Point2d, line_end: &Point2d, point: &Point2d) -> bool {
    ((point.x >= line_start.x && point.x <= line_end.x
        || point.x <= line_start.x && point.x >= line_end.x)
        && point.y == line_start.y)
        || ((point.y >= line_start.y && point.y <= line_end.y
            || point.y <= line_start.y && point.y >= line_end.y)
            && point.x == line_start.x)
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

#[derive(Debug)]
pub struct GameState {
    snake: Snake,
    max_y: i32,
    max_x: i32,
}

impl GameState {
    pub fn new(max_y: i32, max_x: i32) -> Self {
        GameState {
            snake: Snake::new(),
            max_x,
            max_y,
        }
    }
}

impl GameState {
    pub fn update_gamestate(&mut self, pressed_key: char) {
        let new_direction: Option<Direction> = pressed_key.try_into().ok();

        self.snake.update(new_direction, self.max_y, self.max_x)
    }

    pub fn draw(&mut self, x: i32, y: i32) -> bool {
        return self.snake.draw(x, y);
    }
}
