use super::{Task, TaskError};

#[derive(Debug, Default)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add(&mut self, task: Task) -> Result<(), TaskError> {
        if self.tasks.iter().any(|t| t.id == task.id) {
            return Err(TaskError::DuplicateId(task.id));
        }
        self.tasks.push(task);
        Ok(())
    }

    // pub fn complete(&mut self, id: usize) -> Result<(), TaskError> {
    //     self.tasks
    //         .iter_mut()
    //         .find(|t| t.id == id)
    //         .ok_or(TaskError::NotFound(id))
    //         .map(|task| {
    //             task.done = true;
    //             Ok(())
    //         })
    //         .unwrap_or_else(|_| Err(TaskError::NotFound(id)))
    // }

    pub fn toggle(&mut self, id: usize) -> Result<(), TaskError> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.done = !task.done;
            Ok(())
        } else {
            Err(TaskError::NotFound(id))
        }
    }

    pub fn list(&self) -> &[Task] {
        &self.tasks
    }
}
