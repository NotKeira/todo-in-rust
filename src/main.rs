mod tasks;
mod ui;

use crossterm::event::{KeyCode, KeyEventKind};
use tasks::TaskList;
use ui::{App, EventHandler, InputMode, Tui};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut task_list = TaskList::new();
    task_list.add("Learn Rust structs")?;
    task_list.add("Implement a task list")?;

    let mut app = App::new(task_list);
    let events = EventHandler::new();
    let mut tui = Tui::new()?;

    while app.running {
        tui.draw(&app)?;

        match events.next()? {
            ui::Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    match app.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('q') => app.quit(),
                            KeyCode::Down => app.select_next(),
                            KeyCode::Up => app.select_previous(),
                            KeyCode::Enter => app.toggle_selected_task(),
                            KeyCode::Char('a') => app.enter_add_mode(),
                            KeyCode::Char('d') => app.delete_selected_task(),
                            _ => {}
                        },
                        InputMode::Adding => match key.code {
                            KeyCode::Enter => app.submit_task(),
                            KeyCode::Esc => app.exit_add_mode(),
                            KeyCode::Backspace => app.delete_char(),
                            KeyCode::Char(c) => app.add_char(c),
                            _ => {}
                        },
                    }
                }
            }
            ui::Event::Tick => {}
        }
    }

    Ok(())
}
