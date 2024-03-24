#[macro_use]
extern crate log;

extern crate pretty_env_logger;

mod handlers;
mod models;
mod persistance;

use::std::sync::Arc;
use dotenvy::dotenv;
use handlers::*;
use sqlx::postgres::PgPoolOptions;
use axum::{
    routing::{delete, get, post},
    Router,
};
use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};

/// Represents the application state containing DAO instances for questions and answers.
#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

/// Main entry point of the application
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

    // Create DataAccessObject instances 
    let questions_dao = Arc::new(QuestionsDaoImpl::new(pool.clone()));
    let answers_dao = Arc::new(AnswersDaoImpl::new(pool));

    let app_state = AppState {questions_dao, answers_dao};

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("Running on 127.0.0.1:8080");
    
    axum::serve(listener, app).await.unwrap();
}