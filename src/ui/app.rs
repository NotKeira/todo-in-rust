use crate::tasks::TaskList;

pub enum InputMode {
    Normal,
    Adding,
}

pub struct App {
    pub task_list: TaskList,
    pub running: bool,
    pub selected: Option<usize>,
    pub input_mode: InputMode,
    pub input: String,
}

impl App {
    pub fn new(task_list: TaskList) -> Self {
        Self {
            task_list,
            running: true,
            selected: None,
            input_mode: InputMode::Normal,
            input: String::new(),
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

    pub fn enter_add_mode(&mut self) {
        self.input_mode = InputMode::Adding;
        self.input.clear();
    }

    pub fn exit_add_mode(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input.clear();
    }

    pub fn add_char(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn delete_char(&mut self) {
        self.input.pop();
    }

    pub fn submit_task(&mut self) {
        if !self.input.is_empty() {
            let _ = self.task_list.add(self.input.clone());
            self.exit_add_mode();
        }
    }

    pub fn delete_selected_task(&mut self) {
        if let Some(selected) = self.selected {
            if let Some(task) = self.task_list.list().get(selected) {
                let id = task.id;
                let _ = self.task_list.remove(id);
                if selected >= self.task_list.list().len() && selected > 0 {
                    self.selected = Some(selected - 1);
                }
            }
        }
    }
}
impl Default for App {
    fn default() -> Self {
        Self::new(TaskList::default())
    }
}
