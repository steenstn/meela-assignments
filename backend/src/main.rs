use chrono::Utc;
use log::info;
use poem::middleware::Cors;
use poem::{
    endpoint::{StaticFileEndpoint, StaticFilesEndpoint}, error::ResponseError, get,
    handler,
    http::StatusCode,
    listener::TcpListener, post,
    web::{Data, Json, Path},
    EndpointExt,
    Route,
    Server,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite, SqlitePool};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Var(#[from] std::env::VarError),
    #[error(transparent)]
    Dotenv(#[from] dotenv::Error),
    #[error("Query failed")]
    QueryFailed,
}

impl ResponseError for Error {
    fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn init_pool() -> Result<SqlitePool, Error> {
    let pool = SqlitePool::connect("sqlite://data.db").await?;
    Ok(pool)
}

#[derive(Serialize)]
struct FormResponse {
    questions: Vec<Question>,
}

#[derive(sqlx::FromRow)]
struct AnswerRow {
    id: String,
    person_id: String,
    question: String,
    answer: String,
    created_at: String,
}

#[handler]
async fn get_answers(
    Data(pool): Data<&SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<HashMap<String, Vec<String>>>, Error> {
    let rows = sqlx::query_as_unchecked!(
        AnswerRow,
        "select id, person_id, question, answer, created_at from answers where person_id = ?",
        id
    )
        .fetch_all(pool)
        .await?;
    let mut answers: HashMap<String, Vec<String>> = HashMap::new();
    for row in rows {
        answers
            .entry(row.question)
            .or_insert_with(Vec::new)
            .push(row.answer);
    }

    Ok(Json(answers))
}

#[handler]
async fn post_answers(
    Data(pool): Data<&SqlitePool>,
    Path((id, question)): Path<(String, String)>,
    Json(payload): Json<AnswersJson>,
) -> Result<Json<serde_json::Value>, Error> {
    let person_id = Uuid::parse_str(id.as_str()).unwrap().to_string();
    sqlx::query!(
        "delete from answers where person_id = ? and question = ?",
        person_id,
        question
    )
        .execute(pool)
        .await?;

    for answer in payload.answers {
        let _id = Uuid::new_v4().to_string();
        let now = Utc::now().to_string();
        sqlx::query!(
            "insert into answers(id, person_id, question, answer, created_at) values (?, ?, ?, ?, ?)",
            _id,
            person_id,
            question,
            answer,
            now
        )
            .execute(pool)
            .await?;
    }
    Ok(Json(serde_json::json!({"success": true})))
}

#[handler]
async fn new_id() -> Result<Json<serde_json::Value>, Error> {
    Ok(Json(serde_json::json!({"id": Uuid::new_v4().to_string()})))
}

#[handler]
async fn get_questions() -> Result<Json<FormResponse>, Error> {
    let questions = vec![
        Question {
            name: "which_areas".into(),
            pretty_name: "Vad behöver du hjälp med?".into(),
            options: vec![
                Option {
                    name: "adhd".into(),
                    pretty_name: "ADHD/ADD".into(),
                },
                Option {
                    name: "adoption".into(),
                    pretty_name: "Adoption".into(),
                },
                Option {
                    name: "work".into(),
                    pretty_name: "Arbete".into(),
                },
                Option {
                    name: "disease".into(),
                    pretty_name: "Allvarlig sjukdom".into(),
                },
                Option {
                    name: "autism".into(),
                    pretty_name: "Autism".into(),
                },
            ],
        },
        Question {
            name: "which_skills".into(),
            pretty_name: "Vad vill du lära dig eller utveckla i terapi?".into(),
            options: vec![
                Option {
                    name: "acceptance".into(),
                    pretty_name: "Acceptans".into(),
                },
                Option {
                    name: "childhood".into(),
                    pretty_name: "Bearbeta barndom".into(),
                },
                Option {
                    name: "self_confidence".into(),
                    pretty_name: "Bygga upp självkänsla".into(),
                },
                Option {
                    name: "relationships".into(),
                    pretty_name: "Förbättra relationer".into(),
                },
            ],
        },
        Question {
            name: "therapist-minority-competence".into(),
            pretty_name: "Önskar du att terapeuten har kunskap inom några av dessa områden?".into(),
            options: vec![
                Option {
                    name: "hbtq".into(),
                    pretty_name: "HBTQ+".into(),
                },
                Option {
                    name: "trans".into(),
                    pretty_name: "Kunskap om transfrågor".into(),
                },
                Option {
                    name: "minority_stress".into(),
                    pretty_name: "Minoritetsstress".into(),
                },
                Option {
                    name: "neurodivergence".into(),
                    pretty_name: "Neurodivergens".into(),
                },
            ],
        },
    ];

    Ok(Json(FormResponse { questions }))
}

#[derive(Deserialize)]
struct AnswersJson {
    answers: Vec<String>,
}

#[derive(Serialize)]
struct Question {
    name: String,
    pretty_name: String,
    options: Vec<Option>,
}

#[derive(Serialize)]
struct Option {
    name: String,
    pretty_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv()?;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Initialize db pool");
    let pool = init_pool().await?;
    let app = Route::new()
        .at("/api/get-questions/", get(get_questions))
        .at("/api/new-id/", get(new_id))
        .at("/api/get-answers/:id/", get(get_answers))
        .at("/api/post-answers/:id/:question/", post(post_answers))
        .at("/favicon.ico", StaticFileEndpoint::new("www/favicon.ico"))
        .nest("/static/", StaticFilesEndpoint::new("www"))
        .data(pool)
        .with(Cors::new());
    Server::new(TcpListener::bind("0.0.0.0:3005"))
        .run(app)
        .await?;

    Ok(())
}
