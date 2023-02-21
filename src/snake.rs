use std::collections::VecDeque;

use anyhow::Result;

use crate::{Direction, Point};

#[derive(Clone)]
pub struct Snake {
    // body position
    pub body: VecDeque<Point>,
    // current direction
    direction: Direction,
}

impl Snake {
    pub fn new(body: VecDeque<Point>) -> Result<Self> {
        Ok(Self {
            body,
            direction: Direction::Right,
        })
    }

    pub fn is_head_on(&self, point: Point) -> bool {
        *self.body.front().unwrap() == point
    }

    pub fn crawl(&mut self, direction: Direction) {
        let mut direction = direction;
        if !direction.is_compatible_with(self.direction) {
            direction = self.direction;
        }

        self.direction = direction;

        let head = *self.body.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Point {
                x: head.x,
                y: head.y - 1,
            },
            Direction::Down => Point {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Left => Point {
                x: head.x - 1,
                y: head.y,
            },
            Direction::Right => Point {
                x: head.x + 1,
                y: head.y,
            },
        };

        self.body.pop_back();
        self.body.push_front(new_head);
    }

    pub fn eat(&mut self) {
        let tail = *self.body.back().unwrap();
        self.body.push_back(tail);
    }
}
