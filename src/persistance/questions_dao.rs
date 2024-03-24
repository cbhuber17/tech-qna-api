use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::{DBError, Question, QuestionDetail};

/// A trait representing data access operations for questions in the database.
#[async_trait]
pub trait QuestionsDao {
    /// Asynchronously creates a new question in the database.
    ///
    /// # Arguments
    ///
    /// * `question` - The question to be created.
    ///
    /// # Returns
    ///
    /// A `Result` containing the newly created question detail on success, or a `DBError` on failure.
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;

    /// Asynchronously deletes a question from the database.
    ///
    /// # Arguments
    ///
    /// * `question_uuid` - The unique identifier of the question to be deleted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure. An empty `Ok(())` is returned on success, otherwise, a `DBError` is returned.
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;

    /// Asynchronously retrieves all questions from the database.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of question details on success, or a `DBError` on failure.
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

/// Implementation of the `QuestionsDao` trait for PostgreSQL database.
pub struct QuestionsDaoImpl {
    db: PgPool,
}

/// Constructor
impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        QuestionsDaoImpl{db}
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {

    /// Asynchronously creates a new question in the database.
    ///
    /// # Arguments
    ///
    /// * `question` - The question to be created.
    ///
    /// # Returns
    ///
    /// A `Result` containing the newly created question detail on success, or a `DBError` on failure.
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {

        // Insert record into DB
        let record = sqlx::query!(
            r#"
                INSERT INTO questions ( title, description )
                VALUES ( $1, $2 )
                RETURNING *
            "#,
            question.title,
            question.description
        ).fetch_one(&self.db).await.map_err(|e| DBError::Other(Box::new(e)))?;

        // Return created record
        Ok(QuestionDetail {
            question_uuid: record.question_uuid.to_string(),
            title: record.title,
            description: record.description,
            created_at: record.created_at.to_string(),
        })
    }

    /// Asynchronously deletes a question from the database.
    ///
    /// # Arguments
    ///
    /// * `question_uuid` - The unique identifier of the question to be deleted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure. An empty `Ok(())` is returned on success, otherwise, a `DBError` is returned.
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {

        // Attempt to get the question UUID, make sure it is valid
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid).map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse question UUID: {}", question_uuid))
        })?;

        // Delete ID from DB
        sqlx::query!("DELETE FROM questions WHERE question_uuid = $1", uuid).execute(&self.db)
                                                                            .await
                                                                            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    /// Asynchronously retrieves all questions for a UUID from the database.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of question details on success, or a `DBError` on failure.
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {

        // Get all questions from DB
        let records = sqlx::query!("SELECT * FROM questions").fetch_all(&self.db)
                                                                          .await
                                                                          .map_err(|e| DBError::Other(Box::new(e)))?;

        // Put the records in an array of QuestionDetail
        let questions = records.iter().map(|r| QuestionDetail {
            question_uuid: r.question_uuid.to_string(),
            title: r.title.clone(),
            description: r.description.clone(),
            created_at: r.created_at.to_string(),
        }).collect();

        Ok(questions)
    }
}