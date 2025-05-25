use crate::tasks::TaskList;

pub struct App {
    pub task_list: TaskList,
    pub running: bool,
    pub selected: Option<usize>,
}

impl App {
    pub fn new(task_list: TaskList) -> Self {
        Self {
            task_list,
            running: true,
            selected: None,
        }
    }
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn select_next(&mut self) {
        let len = self.task_list.list().len();
        if len == 0 {
            return;
        }

        self.selected = Some(match self.selected {
            Some(i) => (i + 1) % len,
            None => 0,
        });
    }

    pub fn select_previous(&mut self) {
        let len = self.task_list.list().len();
        if len == 0 {
            return;
        }

        self.selected = Some(match self.selected {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        });
    }

    pub fn toggle_selected_task(&mut self) {
        if let Some(selected) = self.selected {
            if let Some(task) = self.task_list.list().get(selected) {
                let id = task.id;
                let _ = self.task_list.toggle(id);
            }
        }
    }
}
