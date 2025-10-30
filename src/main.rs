mod handlers;

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::cors::CorsLayer;

// #[derive(Clone)]
// pub struct AppState {
//     pub repo: OutsiderRepository,
// }

#[derive(Clone)]
pub struct AppState {
    pub count: u32,
}

async fn health_check() -> &'static str {
    "Ping from Outsider API"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // sqlx::migrate!("./migrations")
    //     .run(&pool)
    //     .await
    //     .expect("Failed to run migrations");

    // let repo = OutsiderRepository::new(pool);
    // let state = AppState { repo };
    
    let state = AppState { count: 0 };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/lobby", post(handlers::create_lobby))
        .route("/api/lobby/{id}", get(handlers::get_lobby))
        .layer(CorsLayer::permissive())
        .with_state(state);

    Ok(app.into())
}
