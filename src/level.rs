use std::{collections::VecDeque, str::FromStr};

use anyhow::anyhow;
use rand::Rng;
use thiserror::Error;

use crate::{Direction, Point};

use super::snake::Snake;

#[derive(Clone)]
pub struct Level {
    // gameplan width
    width: usize,
    // gameplan height
    height: usize,

    // position of walls
    walls: Vec<Point>,
    // position of food
    food: Point,

    snake: Snake,
}

impl Level {
    pub fn tick(&mut self, direction: Direction) -> Result<Snapshot, LevelError> {
        self.snake.crawl(direction);

        if self.snake.is_head_on(self.food) {
            self.snake.eat();

            self.generate_food();
        }

        for wall in &self.walls {
            if self.snake.is_head_on(*wall) {
                return Err(LevelError::SnakeHitWall);
            }
        }

        for snake_part in self.snake.body.iter().skip(1) {
            if self.snake.is_head_on(*snake_part) {
                return Err(LevelError::SnakeHitSelf);
            }
        }

        Ok(self.snapshot())
    }

    fn generate_food(&mut self) {
        let mut rng = rand::thread_rng();
        let mut food = Point {
            x: rng.gen_range(0..self.width),
            y: rng.gen_range(0..self.height),
        };

        while self.snake.body.contains(&food) || self.walls.contains(&food) {
            food = Point {
                x: rng.gen_range(0..self.width),
                y: rng.gen_range(0..self.height),
            };
        }

        self.food = food;
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            width: self.width,
            height: self.height,
            walls: self.walls.clone(),
            food: self.food,
            body: self.snake.body.clone(),
        }
    }
}

impl FromStr for Level {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let h = s.lines().count();
        let w = s
            .lines()
            .next()
            .ok_or_else(|| anyhow!("cannot get first line"))?
            .len();

        let mut snake_head = None;
        let mut snake_body = VecDeque::new();
        let mut food = None;
        let mut walls = Vec::with_capacity(h as usize * w as usize);

        for (y, line) in s.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let point = Point { x, y };
                match char {
                    '#' => {
                        walls.push(point);
                    }
                    ' ' => {}
                    'h' => {
                        if snake_head.is_some() {
                            return Err(anyhow!("multiple snake heads found"));
                        }

                        snake_head = Some(point);
                        snake_body.push_back(point);
                    }
                    'b' => {
                        snake_body.push_back(point);
                    }
                    'f' => {
                        food = Some(point);
                    }
                    _ => return Err(anyhow!("Invalid char {} at {:?}", char, (x, y))),
                }
            }
        }

        if snake_head.is_none() {
            return Err(anyhow!("snake head not found"));
        }

        Ok(Level {
            width: w,
            height: h,
            walls,
            food: food.ok_or_else(|| anyhow!("food not found"))?,
            snake: Snake::new(snake_body)?,
        })
    }
}

#[derive(Clone)]
pub struct Snapshot {
    // gameplan width
    pub width: usize,
    // gameplan height
    pub height: usize,

    // position of walls
    pub walls: Vec<Point>,
    // position of food
    pub food: Point,

    // position of snake body
    pub body: VecDeque<Point>,
}

#[derive(Error, Debug)]
pub enum LevelError {
    // snake hit a wall
    #[error("snake hit a wall")]
    SnakeHitWall,
    // snake hit itself
    #[error("snake hit itself")]
    SnakeHitSelf,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_parse() {
        static FIRST_LEVEL: &str = "\
##########
#        #
#   h    #
#   b    #
#     f  #
##########";

        let level = Level::from_str(FIRST_LEVEL).unwrap();

        assert_eq!(level.width, 10);
        assert_eq!(level.height, 6);
        assert_eq!(
            level.snake.body,
            vec![Point { x: 4, y: 2 }, Point { x: 4, y: 3 }]
        );
    }
}
