use super::Renderer;
use std::{io, pin::Pin};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event as TermEvent, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{Stream, TryStreamExt};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction as LayoutDirection, Layout, Rect},
    style::Style,
    text::Span,
    widgets::{Block, Borders, Clear, Paragraph},
    Terminal,
};

use crate::{events::Event, level::Snapshot, Direction};

pub struct TUIRenderer {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    last_snapshot: Option<Snapshot>,
}

impl TUIRenderer {
    pub fn new() -> Self {
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        Self {
            terminal,
            last_snapshot: None,
        }
    }
}

impl Renderer for TUIRenderer {
    fn events(&mut self) -> Pin<Box<dyn Stream<Item = Result<Event, std::io::Error>>>> {
        let reader = EventStream::new().map_ok(|event| match event {
            TermEvent::Key(event) => match event.code {
                KeyCode::Char('q') => Event::Quit,
                KeyCode::Char('h') | KeyCode::Left => Event::Direction(Direction::Left),
                KeyCode::Char('j') | KeyCode::Down => Event::Direction(Direction::Down),
                KeyCode::Char('k') | KeyCode::Up => Event::Direction(Direction::Up),
                KeyCode::Char('l') | KeyCode::Right => Event::Direction(Direction::Right),
                _ => Event::None,
            },
            _ => Event::None,
        });

        Box::pin(reader)
    }

    fn render_level(&mut self, snapshot: &Snapshot) {
        self.terminal
            .draw(|frame| {
                let terminal = frame.size();

                let level_rect =
                    centered_rect(snapshot.width as u16, snapshot.height as u16, terminal);

                // render level rectangle
                frame.render_widget(Block::default().borders(Borders::ALL), level_rect);

                if let Some(last_snapshot) = &self.last_snapshot {
                    // render last body
                    for part in last_snapshot.body.iter() {
                        print_at(
                            ' ',
                            level_rect.x + part.x as u16,
                            level_rect.y + part.y as u16,
                        )
                        .unwrap();
                    }
                }

                // // render walls
                // for part in snapshot.walls.iter() {
                //     print_at(
                //         '█',
                //         level_rect.x + part.x as u16,
                //         level_rect.y + part.y as u16,
                //     )
                //     .unwrap();
                // }

                // render food
                print_at(
                    '🍎',
                    level_rect.x + snapshot.food.x as u16,
                    level_rect.y + snapshot.food.y as u16,
                )
                .unwrap();

                // render body
                for part in snapshot.body.iter() {
                    print_at(
                        '█',
                        level_rect.x + part.x as u16,
                        level_rect.y + part.y as u16,
                    )
                    .unwrap();
                }
            })
            .unwrap();

        self.last_snapshot = Some(snapshot.clone());
    }

    fn render_banner(&mut self, message: &str) {
        self.terminal
            .draw(|frame| {
                let terminal = frame.size();

                let block = Block::default().title("Oh no!").borders(Borders::ALL);
                let paragraph = Paragraph::new(Span::styled(message, Style::default()))
                    .alignment(Alignment::Center)
                    .block(block);

                let area = centered_rect(20, 10, terminal);
                frame.render_widget(Clear, area); //this clears out the background
                frame.render_widget(paragraph, area);
            })
            .unwrap();
    }
}

impl Drop for TUIRenderer {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();

        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        self.terminal.show_cursor().unwrap();
    }
}

fn print_at(c: char, x: u16, y: u16) -> Result<(), io::Error> {
    execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(x, y),
        crossterm::style::Print(c)
    )
}

fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let Rect {
        width: grid_width,
        height: grid_height,
        ..
    } = r;

    let popup_layout = Layout::default()
        .direction(LayoutDirection::Vertical)
        .constraints(
            [
                Constraint::Length(grid_height / 2 - height / 2),
                Constraint::Length(height),
                Constraint::Length(grid_height / 2 - height / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(LayoutDirection::Horizontal)
        .constraints(
            [
                Constraint::Length(grid_width / 2 - width / 2),
                Constraint::Length(width),
                Constraint::Length(grid_width / 2 - width / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
