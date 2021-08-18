use std::{cmp::min, collections::LinkedList, convert::TryInto, io::Stdout, time::Duration};

use termion::raw::RawTerminal;

use crate::{direction::Direction, point::Point2d};

#[derive(Debug)]
pub struct Snake {
    path: Vec<Point2d>,
}

impl Snake {
    fn new(max_y: i32, max_x: i32) -> Self {
        let start_point = Point2d::new(max_x / 2, max_y / 2, Direction::Left);

        let length: u32 = 20;
        let mut path = vec![];
        path.push(start_point.clone());
        for x in 0..length - 1 {
            let mut point = start_point.clone();
            point.x = start_point.x - 1;
            path.push(point);
        }

        Snake { path }
    }

    fn update(&mut self, new_direction: Option<Direction>, max_y: i32, max_x: i32) {
        let mut skip_next = false;
        if new_direction.is_some() {
            let new_direction = new_direction.unwrap();
            let mut head = &mut self.path[0];
            if head.direction.ne(&new_direction) && !head.direction.isOpposite(&new_direction) {
                head.direction = new_direction;
            }
        }

        for i in 0..self.path.len() {
            self.path[i].move_by_direction(max_y, max_x);
        }

        for i in 1..self.path.len() {
            if skip_next {
                skip_next = false;
                continue;
            }

            let parent = &self.path[i - 1].clone();
            if parent.direction != self.path[i].direction {
                skip_next = true;
            }
            self.path[i].update_direction_by_parent(parent);
        }
    }

    fn draw(&self, terminal: &mut RawTerminal<Stdout>) {
        let mut buffer: Vec<(u16, u16, u16)> = vec![];
        for ele in self.path.iter() {
            let bele = buffer.iter_mut().find(|x| x.1 == ele.y.try_into().unwrap());

            match bele {
                Some(x) => {
                    x.2 = x.2 + 1;
                    x.0 = min(x.0, ele.x.try_into().unwrap())
                }
                None => buffer.push((ele.x.try_into().unwrap(), ele.y.try_into().unwrap(), 1)),
            }
        }

        for ele in buffer {
            use std::io::Write;
            write!(
                terminal,
                "{}",
                termion::cursor::Goto((ele.0).try_into().unwrap(), (ele.1).try_into().unwrap()),
            )
            .ok();
            for i in 0..ele.2 {
                write!(terminal, "\u{2B1B}").ok();
            }
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
