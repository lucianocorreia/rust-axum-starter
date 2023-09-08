use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::model::{ModelController, Task, TaskForCreate};
use crate::Result;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tasks", post(create_task).get(list_tasks))
        .route("/tasks/:id", delete(delete_task))
        .with_state(mc)
}

async fn create_task(
    State(mc): State<ModelController>,
    Json(task_fc): Json<TaskForCreate>,
) -> Result<Json<Task>> {
    println!(">> {:<12} - create_task", "HANDLER");

    let task = mc.create_task(task_fc).await?;

    Ok(Json(task))
}

async fn list_tasks(State(mc): State<ModelController>) -> Result<Json<Vec<Task>>> {
    println!(">> {:<12} - list_tasks", "HANDLER");

    let tasks = mc.list_tasks().await?;

    Ok(Json(tasks))
}

async fn delete_task(State(mc): State<ModelController>, Path(id): Path<u64>) -> Result<Json<Task>> {
    println!(">> {:<12} - delete_task", "HANDLER");

    let task = mc.delete_task(id).await?;

    Ok(Json(task))
}
