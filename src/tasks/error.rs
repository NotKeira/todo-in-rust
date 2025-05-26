use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TaskError {
    NotFound(usize),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::NotFound(id) => write!(f, "Task with id {} not found", id),
        }
    }
}

impl Error for TaskError {}
