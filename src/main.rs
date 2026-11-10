use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
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

async fn create_task(Json(payload): Json<CreateTask>) -> (StatusCode, Json<Task>) {
    let new_task = Task {
        id: seed_tasks().len() as u32 + 1,
        title: payload.title,
        done: false,
    };
    // Here you would normally save the new task to a database or in-memory store
    (StatusCode::CREATED, Json(new_task))
}

async fn list_tasks() -> Json<Vec<Task>> {
    Json(seed_tasks())
}

async fn get_task(Path(id): Path<u32>) -> Result<Json<Task>, StatusCode> {
    let tasks = seed_tasks();
    tasks
        .into_iter()
        .find(|task| task.id == id)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
