use std::pin::Pin;

use futures::Stream;

use crate::{events::Event, level::Snapshot};

pub mod tui;

pub trait Renderer {
    fn events(&mut self) -> Pin<Box<dyn Stream<Item = Result<Event, std::io::Error>>>>;

    fn render_level(&mut self, snapshot: &Snapshot);
    fn render_banner(&mut self, message: &str);
}
