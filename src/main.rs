#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use axum::{
    routing::{delete, get, post},
    Router,
};

use dotenvy::dotenv;

mod handlers;
mod models;
mod persistance;

use handlers::*;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    const MAX_CONNECTIONS: u32 = 5;

    pretty_env_logger::init();
    dotenv().ok();

    // Create a new PgPoolOptions instance
    let pool = PgPoolOptions::new().max_connections(MAX_CONNECTIONS)
                                                   .connect(&std::env::var("DATABASE_URL")
                                                   .expect("DATABASE_URL must be set."))
                                                   .await
                                                   .expect("Failed to create Postgres connection pool!");

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}