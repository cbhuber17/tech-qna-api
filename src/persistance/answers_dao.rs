use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

/// A trait representing data access operations for questions in the database.
#[async_trait]
pub trait AnswersDao {

    /// Asynchronously creates a new answer in the database.
    ///
    /// # Arguments
    ///
    /// * `answer` - The answer to be created.
    ///
    /// # Returns
    ///
    /// A `Result` containing the newly created answer detail on success, or a `DBError` on failure.
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;

    /// Asynchronously deletes an answer from the database.
    ///
    /// # Arguments
    ///
    /// * `answer_uuid` - The unique identifier of the answer to be deleted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure. An empty `Ok(())` is returned on success, otherwise, a `DBError` is returned.
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;

    /// Asynchronously retrieves all answers from the database.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of answer details on success, or a `DBError` on failure.
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

/// Implementation of the `AnswersDao` trait for PostgreSQL database.
pub struct AnswersDaoImpl {
    db: PgPool,
}

/// Constructor
impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        AnswersDaoImpl {db} 
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {

    /// Asynchronously creates a new answer in the database.
    ///
    /// # Arguments
    ///
    /// * `answer` - The answer to be created.
    ///
    /// # Returns
    ///
    /// A `Result` containing the newly created answer detail on success, or a `DBError` on failure.
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {

        // Attempt to get question UUID (for the answer), make sure it is valid
        let uuid = sqlx::types::Uuid::parse_str(&answer.question_uuid).map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse answer UUID: {}", answer.question_uuid))
        })?;

        // If executing the query results in an error, check to see if
        // the error code matches `postgres_error_codes::FOREIGN_KEY_VIOLATION`.
        // If so early return the `DBError::InvalidUUID` error. Otherwise early return
        // the `DBError::Other` error.
        let record = sqlx::query!(
            r#"
                INSERT INTO answers ( question_uuid, content )
                VALUES ( $1, $2 )
                RETURNING *
            "#,
            uuid,
            answer.content
        ).fetch_one(&self.db)
         .await
         .map_err(|e: sqlx::Error| match e {
            sqlx::Error::Database(e) => {
                if let Some(code) = e.code() {
                    if code.eq(postgres_error_codes::FOREIGN_KEY_VIOLATION) {
                        return DBError::InvalidUUID(format!("Invalid question UUID: {}", answer.question_uuid));
                    }
                }
                DBError::Other(Box::new(e))
            }
            e => DBError::Other(Box::new(e)),
         })?;

        // Return created record
        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content,
            created_at: record.created_at.to_string(),
        })
    }

    /// Asynchronously deletes an answer from the database.
    ///
    /// # Arguments
    ///
    /// * `answer_uuid` - The unique identifier of the answer to be deleted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure. An empty `Ok(())` is returned on success, otherwise, a `DBError` is returned.
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {

        // Attempt to get the answer UUID, make sure it is valid
        let uuid = sqlx::types::Uuid::parse_str(&answer_uuid).map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse answer UUID: {}", answer_uuid))
        })?;

        // Delete from DB
        sqlx::query!("DELETE FROM answers WHERE answer_uuid = $1", uuid).execute(&self.db)
                                                                        .await
                                                                        .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    /// Asynchronously retrieves all answers for a UUID from the database.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of answer details on success, or a `DBError` on failure.
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {

        // Attempt to get question UUID (for the answer), make sure it is valid
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid).map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse question with UUID: {}", question_uuid))
        })?;

        // Get all answers from DB
        let records = sqlx::query!("SELECT * FROM answers WHERE question_uuid = $1", uuid).fetch_all(&self.db)
                                                                                                       .await
                                                                                                       .map_err(|e| DBError::Other(Box::new(e)))?;

        // Put the records in an array of AnswerDetail
        let answers = records.iter().map(|r| AnswerDetail {
            answer_uuid: r.answer_uuid.to_string(),
            question_uuid: r.question_uuid.to_string(),
            content: r.content.clone(),
            created_at: r.created_at.to_string(),
        }).collect();

        Ok(answers)
    }
}