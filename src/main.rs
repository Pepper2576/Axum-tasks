use axum::{
    Router,
    extract::{Path, Query},
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Serialize, Clone)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
}

fn seed_tasks() -> Vec<Task> {
    vec![
        Task { id: 1, title: "Task 1".to_string(), done: false },
        Task { id: 2, title: "Task 2".to_string(), done: true },
    ]
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/{id}", get(get_task));

    let port = env::var("PORT").unwrap_or_else(|_| "3500".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Listening on http://localhost:{}", port);

    axum::serve(listener, app).await.unwrap()
}

async fn root() -> &'static str {
    "taskr is alive"
}

async fn health() -> &'static str {
    "ok"
}

async fn create_task() -> &'static str {
    "task created"
}

async fn list_tasks(Query(params): Query<HashMap<String, String>>) -> String {
    let filter = params
        .get("done")
        .map_or("everything".to_string(), |v| format!("done={}", v));
    format!("list of tasks filtered by {}", filter)
}

async fn get_task(Path(id): Path<u32>) -> String {
    format!("task with id {}", id)
}
