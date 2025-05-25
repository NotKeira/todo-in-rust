use crate::ui::App;
use crossterm::{
    ExecutableCommand,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::io::{self, Stdout};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    list_state: ratatui::widgets::ListState,
}

impl Tui {
    pub fn new() -> io::Result<Self> {
        let mut stdout = io::stdout();
        terminal::enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        let list_state = ratatui::widgets::ListState::default();
        Ok(Self {
            terminal,
            list_state,
        })
    }

    pub fn draw(&mut self, app: &App) -> io::Result<()> {
        self.terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3), // Header
                        Constraint::Min(3),    // Tasks
                        Constraint::Length(5), // Footer
                    ]
                    .as_ref(),
                )
                .split(frame.size());

            // Header
            let header = Paragraph::new("WhatTodo - A Task List done right.")
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center);
            frame.render_widget(header, chunks[0]);

            // Tasks
            let tasks: Vec<ListItem> = app
                .task_list
                .list()
                .iter()
                .map(|t| {
                    let style = if t.done {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(t.to_string()).style(style)
                })
                .collect();

            let tasks = List::new(tasks)
                .block(Block::default().title("Tasks").borders(Borders::ALL))
                .highlight_style(
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                );
            if let Some(selected) = app.selected {
                self.list_state.select(Some(selected));
            }
            frame.render_stateful_widget(tasks, chunks[1], &mut self.list_state);

            // Footer
            let controls = ["↑/↓: Navigate", "Enter: Toggle completion", "q: Quit"].join("\n");

            let footer = Paragraph::new(controls)
                .style(Style::default().fg(Color::Gray))
                .block(Block::default().borders(Borders::ALL).title("Controls"));
            frame.render_widget(footer, chunks[2]);
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
