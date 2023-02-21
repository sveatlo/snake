use std::str::FromStr;

use anyhow::Result;
use futures::{FutureExt, StreamExt};
use thiserror::Error;
use tokio::select;
use tokio::time::interval;

use crate::events::Event;
use crate::level::Level;
use crate::renderer::Renderer;
use crate::Direction;

pub struct Game {
    level: Level,
    renderer: Box<dyn Renderer>,
    // TODO: add score, gameplay duration, tick duration
}

static FIRST_LEVEL: &str = "\
########################################
#                                      #
#                                      #
#       h              f               #
#       b                              #
#                                      #
#                                      #
#                                      #
#                                      #
#                                      #
#                                      #
#                                      #
#                                      #
#                                      #
########################################";

impl Game {
    pub fn new() -> Result<Self, GameError> {
        Ok(Self {
            level: Level::from_str(FIRST_LEVEL)?,
            renderer: Box::new(crate::renderer::tui::TUIRenderer::new()),
        })
    }

    pub async fn run(&mut self) -> Result<(), GameError> {
        let mut events = self.renderer.events();

        let mut direction = Direction::Right;
        let mut tick = interval(std::time::Duration::from_millis(500));

        loop {
            select! {
                _ = tick.tick() => {},
                event = events.next().fuse() => {
                    direction = match event {
                        None => direction,
                        Some(Ok(Event::None)) => direction,
                        Some(Ok(Event::Quit)) => break,
                        Some(Ok(Event::Direction(new_direction))) => new_direction,
                        Some(Err(e)) => {
                            println!("Error receiving event: {:?}", e);
                            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                            break;
                        }
                    }
                }
            }

            match self.level.tick(direction) {
                Ok(snapshot) => {
                    self.renderer.render_level(&snapshot);
                }
                Err(_) => self.renderer.render_banner("You lost!"),
            }
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("quitting")]
    Quit,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
