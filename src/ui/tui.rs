use crate::ui::App;
use crossterm::{
    ExecutableCommand,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};
use std::io::{self, Stdout};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn new() -> io::Result<Self> {
        let mut stdout = io::stdout();
        terminal::enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn draw(&mut self, app: &mut App) -> io::Result<()> {
        self.terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(frame.size());

            let tasks: Vec<ListItem> = app
                .task_list
                .list()
                .iter()
                .map(|t| ListItem::new(t.to_string()))
                .collect();

            let tasks = List::new(tasks)
                .block(Block::default().title("Tasks").borders(Borders::ALL))
                .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
                .highlight_symbol("> ");

            frame.render_stateful_widget(tasks, chunks[0], &mut app.list_state);
        })?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
        self.terminal
            .backend_mut()
            .execute(LeaveAlternateScreen)
            .unwrap();
    }
}
