use std::fmt;

#[derive(Debug)]
pub struct Task {
    pub(crate) id: usize,
    pub(crate) description: String,
    pub(crate) done: bool,
}

impl Task {
    pub fn new(id: usize, desc: impl Into<String>) -> Self {
        Self {
            id,
            description: desc.into(),
            done: false,
        }
    }

    pub fn complete(&mut self) {
        self.done = true;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.done { "[X]" } else { "[ ]" };
        write!(f, "{} - {} {}", self.id, status, self.description)
    }
}
