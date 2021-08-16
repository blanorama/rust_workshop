use std::{collections::LinkedList, convert::TryInto, io::Stdout, time::Duration};

use termion::raw::RawTerminal;

use crate::{direction::Direction, point::Point2d};

#[derive(Debug)]
pub struct Snake {
    path: LinkedList<Point2d>,

    length: u32,
    current_direction: Direction,
}

impl Snake {
    fn new(max_y: i32, max_x: i32) -> Self {
        let start_point = Point2d::new(max_x / 2, max_y / 2);

        let current_direction = Direction::Right;
        let length: u32 = 20;
        let mut path = LinkedList::new();
        path.push_front(start_point);
        for _ in 0..length - 1 {
            let mut point = path.front().unwrap().clone();
            point.move_by_direction(&current_direction, max_y, max_x);
            path.push_front(point);
        }

        Snake {
            path,

            length,
            current_direction,
        }
    }

    fn update(&mut self, new_direction: Option<Direction>, max_y: i32, max_x: i32) {
        if new_direction.is_some() {
            let new_direction = new_direction.unwrap();
            if self.current_direction.ne(&new_direction)
                && !self.current_direction.isOpposite(&new_direction)
            {
                self.current_direction = new_direction;
            }
        }
        let mut current_front = self.path.front().unwrap().clone();
        current_front.move_by_direction(&self.current_direction, max_y, max_x);
        self.path.push_front(current_front);
        self.path.pop_back();
    }

    fn draw(&self, terminal: &mut RawTerminal<Stdout>) {
        let mut last_y = -1;
        for ele in self.path.iter() {
            use std::io::Write;
            if last_y != ele.y {
                write!(
                    terminal,
                    "{}",
                    termion::cursor::Goto((ele.x).try_into().unwrap(), (ele.y).try_into().unwrap()),
                )
                .ok();
            }
            write!(terminal, "\u{2B1B}").ok();
            last_y = ele.y;
        }
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

    pub fn draw(&mut self, terminal: &mut RawTerminal<Stdout>) {
        self.snake.draw(terminal);
    }
}
