mod routes;
mod state;

use std::env;

#[tokio::main]
async fn main() {
    let state = state::AppState::new();
    //let _app = routes::app(state);

    let port = env::var("PORT").unwrap_or_else(|_| "3500".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Listening on http://localhost:{}", port);

    axum::serve(listener, routes::app(state)).await.unwrap()
}
