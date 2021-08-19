use std::{cmp::min, collections::LinkedList, convert::TryInto, io::Stdout, time::Duration};

use termion::raw::RawTerminal;

use crate::{direction::Direction, point::Point2d};

#[derive(Debug)]
pub struct Snake {
    path: Vec<Point2d>,
}

impl Snake {
    fn new(max_y: i32, max_x: i32) -> Self {
        let mut path = vec![];
        path.push(Point2d::new(max_x / 2, max_y / 2, Direction::Left));
        path.push(Point2d::new(max_x / 2, max_y / 2 - 1, Direction::Left));

        Snake { path }
    }

    fn update(&mut self, new_direction: Option<Direction>, max_y: i32, max_x: i32) {
        for i in (0..self.path.len()).rev() {
            if i != 0 {
                self.path[i].direction = self.path[i - 1].direction.clone();
                self.path[i].x = self.path[i - 1].x.clone();
                self.path[i].y = self.path[i - 1].y.clone();
            }
        }

        let mut head = &mut self.path[0];
        if new_direction.is_some() {
            let new_direction = new_direction.unwrap();
            if head.direction.ne(&new_direction) && !head.direction.isOpposite(&new_direction) {
                head.direction = new_direction;
            }
        }
        head.move_by_direction(max_y, max_x);
    }

    fn draw(&self, terminal: &mut RawTerminal<Stdout>) {
        for ele in self.path.iter() {
            use std::io::Write;
            write!(
                terminal,
                "{}{}",
                termion::cursor::Goto((ele.x).try_into().unwrap(), (ele.y).try_into().unwrap()),
                "\u{2B1B}",
            )
            .ok();
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
