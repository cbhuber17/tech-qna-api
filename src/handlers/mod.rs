use axum::{
    extract::State as AxumState, http::StatusCode, response::IntoResponse, Json as JsonAxum,
};

use crate::{models::*, AppState};

mod handlers_inner;

impl IntoResponse for handlers_inner::HandlerError {
    /// Converts the `HandlerError` into an Axum response.
    ///
    /// # Returns
    ///
    /// An Axum response containing the appropriate status code and message based on the `HandlerError`.
    fn into_response(self) -> axum::response::Response {
        match self {
            handlers_inner::HandlerError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            handlers_inner::HandlerError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

// ---- CRUD for Questions ----

/// Asynchronously creates a new question using the provided `QuestionsDao`.
///
/// # Arguments
///
/// * `AxumState(AppState { questions_dao, .. })` - The application state containing the `QuestionsDao`.
/// * `JsonAxum(question)` - The JSON payload containing the details of the question to be created.
///
/// # Returns
///
/// A `Result` containing either a JSON response with the created question detail or an error response.
pub async fn create_question(
    // Example of how to add state to a route. Note that we are using ".." to ignore the other fields in AppState.
    AxumState(AppState { questions_dao, .. }): AxumState<AppState>,
    JsonAxum(question): JsonAxum<Question>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::create_question(question, questions_dao.as_ref())
        .await
        .map(JsonAxum)
}

/// Asynchronously retrieves all questions.
///
/// # Arguments
///
/// * `AxumState(AppState { questions_dao, .. })` - The application state containing the `QuestionsDao`.
///
/// # Returns
///
/// A `Result` containing either a JSON response with the retrieved questions or an error response.
pub async fn read_questions(
    AxumState(AppState { questions_dao, .. }): AxumState<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::read_questions(questions_dao.as_ref())
        .await
        .map(JsonAxum)
}

/// Asynchronously deletes a question.
///
/// # Arguments
///
/// * `AxumState(AppState { questions_dao, .. })` - The application state containing the `QuestionsDao`.
/// * `JsonAxum(question_uuid)` - The JSON payload containing the unique identifier of the question to be deleted.
///
/// # Returns
///
/// A `Result` containing either a successful response or an error response.
pub async fn delete_question(
    AxumState(AppState { questions_dao, .. }): AxumState<AppState>,
    JsonAxum(question_uuid): JsonAxum<QuestionId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::delete_question(question_uuid, questions_dao.as_ref()).await
}

// ---- CRUD for Answers ----

/// Asynchronously creates a new answer.
///
/// # Arguments
///
/// * `AxumState(AppState { answers_dao, .. })` - The application state containing the `AnswersDao`.
/// * `JsonAxum(answer)` - The JSON payload containing the details of the answer to be created.
///
/// # Returns
///
/// A `Result` containing either a JSON response with the created answer detail or an error response.
pub async fn create_answer(
    AxumState(AppState { answers_dao, .. }): AxumState<AppState>,
    JsonAxum(answer): JsonAxum<Answer>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::create_answer(answer, answers_dao.as_ref())
        .await
        .map(JsonAxum)
}

/// Asynchronously retrieves all answers for a given question.
///
/// # Arguments
///
/// * `AxumState(AppState { answers_dao, .. })` - The application state containing the `AnswersDao`.
/// * `JsonAxum(question_uuid)` - The JSON payload containing the unique identifier of the question for which answers are to be retrieved.
///
/// # Returns
///
/// A `Result` containing either a JSON response with the retrieved answers or an error response.
pub async fn read_answers(
    AxumState(AppState { answers_dao, .. }): AxumState<AppState>,
    JsonAxum(question_uuid): JsonAxum<QuestionId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::read_answers(question_uuid, answers_dao.as_ref())
        .await
        .map(JsonAxum)
}

/// Asynchronously deletes an answer.
///
/// # Arguments
///
/// * `AxumState(AppState { answers_dao, .. })` - The application state containing the `AnswersDao`.
/// * `JsonAxum(answer_uuid)` - The JSON payload containing the unique identifier of the answer to be deleted.
///
/// # Returns
///
/// A `Result` containing either a successful response or an error response.
pub async fn delete_answer(
    AxumState(AppState { answers_dao, .. }): AxumState<AppState>,
    JsonAxum(answer_uuid): JsonAxum<AnswerId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::delete_answer(answer_uuid, answers_dao.as_ref()).await
}