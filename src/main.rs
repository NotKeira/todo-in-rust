mod tasks;
mod ui;

use crossterm::event::{KeyCode, KeyEventKind};
use tasks::{Task, TaskList};
use ui::{App, EventHandler, Tui};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut task_list = TaskList::new();
    task_list.add(Task::new(1, "Learn Rust structs"))?;
    task_list.add(Task::new(2, "Implement a task list"))?;

    let mut app = App::new(task_list);
    let events = EventHandler::new();
    let mut tui = Tui::new()?;

    while app.running {
        tui.draw(&app)?;

        match events.next()? {
            ui::Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.quit(),
                        KeyCode::Down => app.select_next(),
                        KeyCode::Up => app.select_previous(),
                        KeyCode::Enter => app.toggle_selected_task(),
                        _ => {}
                    }
                }
            }
            ui::Event::Tick => {}
        }
    }

    Ok(())
}
