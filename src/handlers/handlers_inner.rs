use crate::{
    models::{Answer, AnswerDetail, AnswerId, DBError, Question, QuestionDetail, QuestionId},
    persistance::{answers_dao::AnswersDao, questions_dao::QuestionsDao},
};

/// Represents errors that can occur within request handlers.
#[derive(Debug, PartialEq)]
pub enum HandlerError {
    BadRequest(String),
    InternalError(String),
}

impl HandlerError {

    /// Constructs a default internal error.
    ///
    /// This method creates an instance of `HandlerError` representing a generic internal error message.
    ///
    /// # Returns
    ///
    /// A `HandlerError` instance representing a default internal error message.
    pub fn default_internal_error() -> Self {
        HandlerError::InternalError("Something went wrong! Please try again.".to_owned())
    }
}

pub async fn create_question(
    question: Question,
    // Using a trait object here so that inner handlers do not depend on concrete DAO implementations
    questions_dao: &(dyn QuestionsDao + Sync + Send),
) -> Result<QuestionDetail, HandlerError> {

    let question = questions_dao.create_question(question).await;

    match question {
        Ok(question) => Ok(question),
        Err(err) => {
            error!("{:?}", err);
            Err(HandlerError::default_internal_error())
        }
    }
}

/// Asynchronously retrieves all questions using the provided `QuestionsDao`.
///
/// # Arguments
///
/// * `questions_dao` - A reference to an object implementing the `QuestionsDao` trait along with `Sync` and `Send` traits.
///
/// # Returns
///
/// A `Result` containing a vector of question details on success, or a `HandlerError` on failure.
pub async fn read_questions(
    questions_dao: &(dyn QuestionsDao + Sync + Send),
) -> Result<Vec<QuestionDetail>, HandlerError> {
    let questions = questions_dao.get_questions().await;

    match questions {
        Ok(questions) => Ok(questions),
        Err(err) => {
            error!("{:?}", err);
            Err(HandlerError::default_internal_error())
        }
    }
}

/// Asynchronously deletes a question identified by the given `QuestionId` using the provided `QuestionsDao`.
///
/// # Arguments
///
/// * `question_id` - The unique identifier of the question to be deleted.
/// * `questions_dao` - A reference to an object implementing the `QuestionsDao` trait along with `Sync` and `Send` traits.
///
/// # Returns
///
/// A `Result` indicating success or failure. An empty `Ok(())` is returned on success, otherwise, a `HandlerError` is returned.
pub async fn delete_question(
    question_id: QuestionId,
    questions_dao: &(dyn QuestionsDao + Sync + Send),
) -> Result<(), HandlerError> {
    let result = questions_dao.delete_question(question_id.question_uuid).await;

    if result.is_err() {
        return Err(HandlerError::default_internal_error());
    }

    Ok(())
}

/// Asynchronously creates an answer using the provided `AnswersDao`.
///
/// # Arguments
///
/// * `answer` - The answer to be created.
/// * `answers_dao` - A reference to an object implementing the `AnswersDao` trait along with `Send` and `Sync` traits.
///
/// # Returns
///
/// A `Result` containing the created answer detail on success, or a `HandlerError` on failure.
pub async fn create_answer(
    answer: Answer,
    answers_dao: &(dyn AnswersDao + Send + Sync),
) -> Result<AnswerDetail, HandlerError> {
    let answer = answers_dao.create_answer(answer).await;

    match answer {
        Ok(answer) => Ok(answer), // return answer
        Err(err) => {
            error!("{:?}", err);

            match err {
                DBError::InvalidUUID(s) => Err(HandlerError::BadRequest(s)),
                _ => Err(HandlerError::default_internal_error()),
            }
        }
    }
}

/// Asynchronously retrieves answers associated with the given question ID using the provided `AnswersDao`.
///
/// # Arguments
///
/// * `question_id` - The unique identifier of the question whose answers are to be retrieved.
/// * `answers_dao` - A reference to an object implementing the `AnswersDao` trait along with `Send` and `Sync` traits.
///
/// # Returns
///
/// A `Result` containing a vector of answer details on success, or a `HandlerError` on failure.
pub async fn read_answers(
    question_id: QuestionId,
    answers_dao: &(dyn AnswersDao + Send + Sync),
) -> Result<Vec<AnswerDetail>, HandlerError> {
    let answers = answers_dao.get_answers(question_id.question_uuid).await;

    match answers {
        Ok(answers) => Ok(answers),
        Err(e) => {
            error!("{:?}", e);
            Err(HandlerError::default_internal_error())
        }
    }
}

/// Asynchronously deletes an answer identified by the given `AnswerId` using the provided `AnswersDao`.
///
/// # Arguments
///
/// * `answer_id` - The unique identifier of the answer to be deleted.
/// * `answers_dao` - A reference to an object implementing the `AnswersDao` trait along with `Send` and `Sync` traits.
///
/// # Returns
///
/// A `Result` indicating success or failure. An empty `Ok(())` is returned on success, otherwise, a `HandlerError` is returned.
pub async fn delete_answer(
    answer_id: AnswerId,
    answers_dao: &(dyn AnswersDao + Send + Sync),
) -> Result<(), HandlerError> {
    let result = answers_dao.delete_answer(answer_id.answer_uuid).await;

    if result.is_err() {
        return Err(HandlerError::default_internal_error());
    }

    Ok(())
}

// ***********************************************************
//                           Tests
// ***********************************************************

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use tokio::sync::Mutex;

    struct QuestionsDaoMock {
        create_question_response: Mutex<Option<Result<QuestionDetail, DBError>>>,
        delete_question_response: Mutex<Option<Result<(), DBError>>>,
        get_questions_response: Mutex<Option<Result<Vec<QuestionDetail>, DBError>>>,
    }

    impl QuestionsDaoMock {
        pub fn new() -> Self {
            QuestionsDaoMock {
                create_question_response: Mutex::new(None),
                delete_question_response: Mutex::new(None),
                get_questions_response: Mutex::new(None),
            }
        }
        pub fn mock_create_question(&mut self, response: Result<QuestionDetail, DBError>) {
            self.create_question_response = Mutex::new(Some(response));
        }
        pub fn mock_delete_question(&mut self, response: Result<(), DBError>) {
            self.delete_question_response = Mutex::new(Some(response));
        }
        pub fn mock_get_questions(&mut self, response: Result<Vec<QuestionDetail>, DBError>) {
            self.get_questions_response = Mutex::new(Some(response));
        }
    }

    #[async_trait]
    impl QuestionsDao for QuestionsDaoMock {
        async fn create_question(&self, _: Question) -> Result<QuestionDetail, DBError> {
            self.create_question_response
                .lock()
                .await
                .take()
                .expect("create_question_response should not be None.")
        }
        async fn delete_question(&self, _: String) -> Result<(), DBError> {
            self.delete_question_response
                .lock()
                .await
                .take()
                .expect("delete_question_response should not be None.")
        }
        async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
            self.get_questions_response
                .lock()
                .await
                .take()
                .expect("get_questions_response should not be None.")
        }
    }

    struct AnswersDaoMock {
        create_answer_response: Mutex<Option<Result<AnswerDetail, DBError>>>,
        delete_answer_response: Mutex<Option<Result<(), DBError>>>,
        get_answers_response: Mutex<Option<Result<Vec<AnswerDetail>, DBError>>>,
    }

    impl AnswersDaoMock {
        pub fn new() -> Self {
            AnswersDaoMock {
                create_answer_response: Mutex::new(None),
                delete_answer_response: Mutex::new(None),
                get_answers_response: Mutex::new(None),
            }
        }
        pub fn mock_create_answer(&mut self, response: Result<AnswerDetail, DBError>) {
            self.create_answer_response = Mutex::new(Some(response));
        }
        pub fn mock_delete_answer(&mut self, response: Result<(), DBError>) {
            self.delete_answer_response = Mutex::new(Some(response));
        }
        pub fn mock_get_answers(&mut self, response: Result<Vec<AnswerDetail>, DBError>) {
            self.get_answers_response = Mutex::new(Some(response));
        }
    }

    #[async_trait]
    impl AnswersDao for AnswersDaoMock {
        async fn create_answer(&self, _: Answer) -> Result<AnswerDetail, DBError> {
            self.create_answer_response
                .lock()
                .await
                .take()
                .expect("create_answer_response should not be None.")
        }
        async fn delete_answer(&self, _: String) -> Result<(), DBError> {
            self.delete_answer_response
                .lock()
                .await
                .take()
                .expect("delete_answer_response should not be None.")
        }
        async fn get_answers(&self, _: String) -> Result<Vec<AnswerDetail>, DBError> {
            self.get_answers_response
                .lock()
                .await
                .take()
                .expect("get_answers_response should not be None.")
        }
    }

    #[tokio::test]
    async fn create_question_should_return_question() {
        let question = Question {
            title: "test title".to_owned(),
            description: "test description".to_owned(),
        };

        let question_detail = QuestionDetail {
            question_uuid: "123".to_owned(),
            title: question.title.clone(),
            description: question.description.clone(),
            created_at: "now".to_owned(),
        };

        let mut questions_dao = QuestionsDaoMock::new();

        questions_dao.mock_create_question(Ok(question_detail.clone()));

        let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(questions_dao);

        let result = create_question(question, questions_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), question_detail);
    }

    #[tokio::test]
    async fn create_question_should_return_error() {
        let question = Question {
            title: "test title".to_owned(),
            description: "test description".to_owned(),
        };

        let mut questions_dao = QuestionsDaoMock::new();

        questions_dao.mock_create_question(Err(DBError::InvalidUUID("test".to_owned())));

        let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(questions_dao);

        let result = create_question(question, questions_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }

    #[tokio::test]
    async fn read_questions_should_return_questions() {
        let question_detail = QuestionDetail {
            question_uuid: "123".to_owned(),
            title: "test title".to_owned(),
            description: "test description".to_owned(),
            created_at: "now".to_owned(),
        };

        let mut questions_dao = QuestionsDaoMock::new();

        questions_dao.mock_get_questions(Ok(vec![question_detail.clone()]));

        let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(questions_dao);

        let result = read_questions(questions_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![question_detail]);
    }

    #[tokio::test]
    async fn read_questions_should_return_error() {
        let mut questions_dao = QuestionsDaoMock::new();

        questions_dao.mock_get_questions(Err(DBError::InvalidUUID("test".to_owned())));

        let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(questions_dao);

        let result = read_questions(questions_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }

    #[tokio::test]
    async fn delete_question_should_succeed() {
        let question_id = QuestionId {
            question_uuid: "123".to_owned(),
        };

        let mut questions_dao = QuestionsDaoMock::new();

        questions_dao.mock_delete_question(Ok(()));

        let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(questions_dao);

        let result = delete_question(question_id, questions_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[tokio::test]
    async fn delete_question_should_return_error() {
        let question_id = QuestionId {
            question_uuid: "123".to_owned(),
        };

        let mut questions_dao = QuestionsDaoMock::new();

        questions_dao.mock_delete_question(Err(DBError::InvalidUUID("test".to_owned())));

        let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(questions_dao);

        let result = delete_question(question_id, questions_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }

    #[tokio::test]
    async fn create_answer_should_return_answer() {
        let answer = Answer {
            question_uuid: "123".to_owned(),
            content: "test content".to_owned(),
        };

        let answer_detail = AnswerDetail {
            answer_uuid: "456".to_owned(),
            question_uuid: answer.question_uuid.clone(),
            content: answer.content.clone(),
            created_at: "now".to_owned(),
        };

        let mut answers_dao = AnswersDaoMock::new();

        answers_dao.mock_create_answer(Ok(answer_detail.clone()));

        let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(answers_dao);

        let result = create_answer(answer, answers_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), answer_detail);
    }

    #[tokio::test]
    async fn create_answer_should_return_bad_request_error() {
        let answer = Answer {
            question_uuid: "123".to_owned(),
            content: "test content".to_owned(),
        };

        let mut answers_dao = AnswersDaoMock::new();

        answers_dao.mock_create_answer(Err(DBError::InvalidUUID("test".to_owned())));

        let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(answers_dao);

        let result = create_answer(answer, answers_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::BadRequest("".to_owned()))
        );
    }

    #[tokio::test]
    async fn create_answer_should_return_internal_error() {
        let answer = Answer {
            question_uuid: "123".to_owned(),
            content: "test content".to_owned(),
        };

        let mut answers_dao = AnswersDaoMock::new();

        answers_dao.mock_create_answer(Err(DBError::Other(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "oh no!",
        )))));

        let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(answers_dao);

        let result = create_answer(answer, answers_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }

    #[tokio::test]
    async fn read_answers_should_return_answers() {
        let answer_detail = AnswerDetail {
            answer_uuid: "456".to_owned(),
            question_uuid: "123".to_owned(),
            content: "test content".to_owned(),
            created_at: "now".to_owned(),
        };

        let question_id = QuestionId {
            question_uuid: "123".to_owned(),
        };

        let mut answers_dao = AnswersDaoMock::new();

        answers_dao.mock_get_answers(Ok(vec![answer_detail.clone()]));

        let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(answers_dao);

        let result = read_answers(question_id, answers_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![answer_detail]);
    }

    #[tokio::test]
    async fn read_answers_should_return_error() {
        let question_id = QuestionId {
            question_uuid: "123".to_owned(),
        };

        let mut answers_dao = AnswersDaoMock::new();

        answers_dao.mock_get_answers(Err(DBError::InvalidUUID("test".to_owned())));

        let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(answers_dao);

        let result = read_answers(question_id, answers_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }

    #[tokio::test]
    async fn delete_answer_should_succeed() {
        let answer_id = AnswerId {
            answer_uuid: "123".to_owned(),
        };

        let mut answers_dao = AnswersDaoMock::new();

        answers_dao.mock_delete_answer(Ok(()));

        let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(answers_dao);

        let result = delete_answer(answer_id, answers_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[tokio::test]
    async fn delete_answer_should_return_error() {
        let answer_id = AnswerId {
            answer_uuid: "123".to_owned(),
        };

        let mut answers_dao = AnswersDaoMock::new();

        answers_dao.mock_delete_answer(Err(DBError::InvalidUUID("test".to_owned())));

        let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(answers_dao);

        let result = delete_answer(answer_id, answers_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }
}