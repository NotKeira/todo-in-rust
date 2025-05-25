use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::time::Duration;

pub enum Event {
    Key(KeyEvent),
    Tick,
}

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn next(&self) -> Result<Event, Box<dyn std::error::Error>> {
        if event::poll(Duration::from_millis(250))? {
            if let CrosstermEvent::Key(key) = event::read()? {
                return Ok(Event::Key(key));
            }
        }
        Ok(Event::Tick)
    }
}
