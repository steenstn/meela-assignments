use std::env;

use log::info;
use poem::{
    EndpointExt, Route, Server,
    error::ResponseError,
    get, handler,
    http::StatusCode,
    listener::TcpListener,
    web::{Data, Json, Path},
};
use serde::Serialize;
use sqlx::SqlitePool;

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
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    Ok(pool)
}

#[derive(Serialize)]
struct HelloResponse {
    hello: String,
}

#[handler]
async fn hello(
    Data(pool): Data<&SqlitePool>,
    Path(name): Path<String>,
) -> Result<Json<HelloResponse>, Error> {
    let r = sqlx::query!("select concat('Hello ', $1) as hello", name)
        .fetch_one(pool)
        .await?;
    let Some(hello) = r.hello else {
        Err(Error::QueryFailed)?
    };

    Ok(Json(HelloResponse { hello }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv()?;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Initialize db pool");
    let pool = init_pool().await?;
    let app = Route::new().at("hello/:name", get(hello)).data(pool);
    Server::new(TcpListener::bind("0.0.0.0:3005")).run(app).await?;

    Ok(())
}
