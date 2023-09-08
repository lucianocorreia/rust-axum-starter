use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    tasks_store: Arc<Mutex<Vec<Option<Task>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tasks_store: Arc::default(),
        })
    }
}

impl ModelController {
    pub async fn create_task(&self, task_fc: TaskForCreate) -> Result<Task> {
        let mut store = self.tasks_store.lock().unwrap();

        let id = store.len() as u64;
        let task = Task {
            id: id as u64,
            title: task_fc.title,
            done: false,
        };

        store.push(Some(task.clone()));

        Ok(task)
    }

    pub async fn list_tasks(&self) -> Result<Vec<Task>> {
        let store = self.tasks_store.lock().unwrap();

        let tasks = store.iter().filter_map(|task| task.clone()).collect();

        Ok(tasks)
    }

    pub async fn delete_task(&self, id: u64) -> Result<Task> {
        let mut store = self.tasks_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TaskDeleteFailIdNotFound { id })
    }
}
