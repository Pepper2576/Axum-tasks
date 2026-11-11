use crate::models::Task;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub tasks: Arc<Mutex<Vec<Task>>>,
    pub next_id: Arc<Mutex<u32>>,
}

impl AppState {
    pub fn new() -> Self {
        let seed = vec![
            Task {
                id: 1,
                title: "Task 1".to_string(),
                done: false,
            },
            Task {
                id: 2,
                title: "Task 2".to_string(),
                done: true,
            },
            Task {
                id: 3,
                title: "Task 3".to_string(),
                done: false,
            },
            Task {
                id: 4,
                title: "Task 4".to_string(),
                done: true,
            },
        ];
        Self {
            tasks: Arc::new(Mutex::new(seed)),
            next_id: Arc::new(Mutex::new(5)),
        }
    }
}
