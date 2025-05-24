use std::fmt;

struct Task {
    id: usize,
    description: String,
    done: bool,
}

struct TaskList {
    tasks: Vec<Task>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.done { "[X]" } else { "[ ]" };
        write!(f, "{} {}", status, self.description)
    }
}

impl TaskList {
    fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn show(&self) {
        for task in &self.tasks {
            println!("{task}");
        }
    }
}

impl Task {
    /// makea a new taskie
    fn new(id: usize, desc: impl Into<String>) -> Self {
        Self {
            id,
            description: desc.into(),
            done: false,
        }
    }

    /// mark a de taskie as de completeh
    fn complete(&mut self) {
        self.done = true;
    }

    /// send de task to de consolio
    fn print(&self) {
        println!("{self}");
    }
}

fn main() {
    let mut todo = vec![
        Task::new(1, "Learn Rust structs"),
        Task::new(1, "Implement a task list"),
    ];

    todo[0].complete();

    for task in &todo {
        task.print();
    }
}
