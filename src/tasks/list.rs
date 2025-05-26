use super::{Task, TaskError};

#[derive(Debug, Default)]
pub struct TaskList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, description: impl Into<String>) -> Result<(), TaskError> {
        let task = Task::new(self.next_id, description);
        self.next_id += 1;
        self.tasks.push(task);
        Ok(())
    }

    pub fn toggle(&mut self, id: usize) -> Result<(), TaskError> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.done = !task.done;
            Ok(())
        } else {
            Err(TaskError::NotFound(id))
        }
    }

    pub fn remove(&mut self, id: usize) -> Result<(), TaskError> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            Ok(())
        } else {
            Err(TaskError::NotFound(id))
        }
    }

    pub fn list(&self) -> &[Task] {
        &self.tasks
    }
}
