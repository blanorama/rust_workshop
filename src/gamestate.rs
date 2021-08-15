use std::{collections::LinkedList, convert::TryInto, io::Stdout};

use termion::raw::RawTerminal;

use crate::{
    direction::Direction,
    point::{is_point_on_strait_line, Point2d},
};

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
    fn new(max_y: i32, max_x: i32) -> Self {
        let head_position = Point2d::new(10, 10);
        let current_direction = Direction::Right;
        let length: u32 = 10;
        let mut tail_position = head_position.clone();
        tail_position.move_by_direction(&current_direction, (length - 1) as i32, max_y, max_x);
        Snake {
            edges_position: LinkedList::new(),
            edges_directions: LinkedList::new(),
            length,
            current_direction,
            head_position,
            tail_position,
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
            .move_by_direction(&self.current_direction, 1, max_y, max_x);

        self.tail_position.move_by_direction(
            &self
                .edges_directions
                .back()
                .unwrap_or(&self.current_direction),
            1,
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

    fn draw(&self, x: i32, y: i32, terminal: &mut RawTerminal<Stdout>) -> bool {
        let point = &Point2d::new(x, y);
        if self.edges_directions.len() == 0 {
            if is_point_on_strait_line(&self.head_position, &self.tail_position, point) {
                use std::io::Write;
                write!(
                    terminal,
                    "{}\u{2B1B}",
                    termion::cursor::Goto((x + 1).try_into().unwrap(), (y + 1).try_into().unwrap())
                )
                .ok();
            }
        } else {
            let mut iter = self.edges_position.iter();
            let mut line_start_option = Some(&self.head_position);
            let mut line_end_option = iter.next();

            while let Some(line_end) = line_end_option {
                let line_start = line_start_option.unwrap();
                if is_point_on_strait_line(line_start, line_end, point) {
                    use std::io::Write;
                    write!(
                        terminal,
                        "{}\u{2B1B}",
                        termion::cursor::Goto(
                            (x + 1).try_into().unwrap(),
                            (y + 1).try_into().unwrap()
                        )
                    )
                    .ok();
                }

                line_start_option = line_end_option;
                line_end_option = iter.next();
            }
        }
        return false;
    }
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
            snake: Snake::new(max_y, max_x),
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

    pub fn draw(&mut self, x: i32, y: i32, terminal: &mut RawTerminal<Stdout>) -> bool {
        return self.snake.draw(x, y, terminal);
    }
}
