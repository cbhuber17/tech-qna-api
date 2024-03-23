use crate::models::*;
use axum::{response::IntoResponse, Json};

// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    Json(QuestionDetail {
        question_uuid: "00000000-0000-0000-0000-000000000000".into(),
        title: "Lorem ipsum".into(),
        description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
            tempor incididunt ut labore et dolore magna aliqua. Auctor urna nunc id cursus. Non \
            odio euismod lacinia at quis. Augue lacus viverra vitae congue eu consequat ac felis. \
            Justo nec ultrices dui sapien."
            .into(),
        created_at: "Jan 1, 1970".into(),
    })
}

pub async fn read_questions() -> impl IntoResponse {
    Json(vec![
        QuestionDetail {
            question_uuid: "00000000-0000-0000-0000-000000000000".into(),
            title: "Lorem ipsum".into(),
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
                tempor incididunt ut labore et dolore magna aliqua. Est velit egestas dui id \
                ornare arcu odio. Suspendisse interdum consectetur libero id faucibus nisl. \
                Tincidunt vitae semper quis lectus. Proin gravida hendrerit lectus a."
                .into(),
            created_at: "Jan 1, 1970".into(),
        },
        QuestionDetail {
            question_uuid: "00000000-0000-0000-0000-000000000000".into(),
            title: "Lorem ipsum".into(),
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
                tempor incididunt ut labore et dolore magna aliqua. Ipsum faucibus vitae aliquet \
                nec ullamcorper sit amet risus nullam. Pulvinar elementum integer enim neque \
                volutpat ac tincidunt vitae semper. Scelerisque in dictum non consectetur a. Enim \
                nunc faucibus a pellentesque sit amet porttitor eget."
                .into(),
            created_at: "Jan 1, 1970".into(),
        },
    ])
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {
    ()
}

// ---- CRUD for Answers ----

// TODO: Create a POST route to /answer which accepts an `Answer` and returns `AnswerDetail` as JSON.
//       The handler function should be called `create_answer`.
//
//       hint: this function should look very similar to the create_question function above

pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    Json(AnswerDetail {
        answer_uuid: "00000000-0000-0000-0000-000000000000".into(),
        question_uuid: "00000000-0000-0000-0000-000000000000".into(),
        content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor \
            incididunt ut labore et dolore magna aliqua. In aliquam sem fringilla ut. Enim tortor \
            at auctor urna. Mattis vulputate enim nulla aliquet porttitor lacus. Interdum varius \
            sit amet mattis vulputate enim nulla."
            .into(),
        created_at: "Jan 1, 1970".into(),
    })
}

// TODO: Create a GET route to /answers which accepts an `QuestionId` and returns a vector of `AnswerDetail` as JSON.
//       The handler function should be called `read_answers`.
//
//       hint: this function should look very similar to the read_questions function above
pub async fn read_answers() -> impl IntoResponse {
    Json(vec![
        AnswerDetail {
            answer_uuid: "00000000-0000-0000-0000-000000000000".into(),
            question_uuid: "00000000-0000-0000-0000-000000000000".into(),
            content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
                tempor incididunt ut labore et dolore magna aliqua. Viverra aliquet eget sit amet \
                tellus cras. Est sit amet facilisis magna etiam. Odio facilisis mauris sit amet. \
                Diam sit amet nisl suscipit."
                .into(),
            created_at: "Jan 1, 1970".into(),
        },
        AnswerDetail {
            answer_uuid: "00000000-0000-0000-0000-000000000000".into(),
            question_uuid: "00000000-0000-0000-0000-000000000000".into(),
            content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
                tempor incididunt ut labore et dolore magna aliqua. Imperdiet nulla malesuada \
                pellentesque elit eget gravida cum sociis. Sed viverra tellus in hac habitasse \
                platea dictumst vestibulum rhoncus. Sed id semper risus in hendrerit gravida. \
                Vitae ultricies leo integer malesuada nunc vel."
                .into(),
            created_at: "Jan 1, 1970".into(),
        },
    ])
}

// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//
//       hint: this function should look very similar to the delete_question function above
pub async fn delete_answer(Json(answer_uuid): Json<AnswerId>) {
    ()
}