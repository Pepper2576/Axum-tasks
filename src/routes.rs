use axum::{Router, routing::get};

use crate::handlers::tasks;
use crate::state::AppState;

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(tasks::root))
        .route("/health", get(tasks::health))
        .route("/tasks", get(tasks::list_tasks).post(tasks::create))
        .route(
            "/tasks/{id}",
            get(tasks::get_one)
                .delete(tasks::delete)
                .patch(tasks::update),
        )
        .with_state(state)
}
