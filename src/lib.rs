use std::path::PathBuf;

use actix_web::{
    error, get,
    web::{self, Json, ServiceConfig},
};
use serde::{Deserialize, Serialize};
use shuttle_service::{error::CustomError, ShuttleActixWeb};
use sqlx::{Executor, FromRow, PgPool};
use tracing::instrument;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
struct Todo {
    pub id: i32,
    pub note: String,
}

#[instrument(skip(db))]
#[get("/todos/{note}")]
async fn add_todo(note: web::Path<String>, db: web::Data<PgPool>) -> actix_web::Result<Json<Todo>> {
    let db = db.as_ref();

    let todo = sqlx::query_as::<_, Todo>("INSERT INTO todos (note) VALUES ($1) RETURNING id, note")
        .bind(note.as_str())
        .fetch_one(db)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(todo))
}

#[instrument(skip(db))]
#[get("/todos")]
async fn todos(db: web::Data<PgPool>) -> actix_web::Result<Json<Vec<Todo>>> {
    let db = db.as_ref();
    let todos = sqlx::query_as::<_, Todo>("Select * from todos ")
        .fetch_all(db)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(todos))
}

#[shuttle_service::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
    pool.execute(include_str!("../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    tracing::info!("Database initialized");

    let db = web::Data::new(pool);

    Ok(move |cfg: &mut ServiceConfig| {
        cfg.app_data(db).service(todos).service(add_todo).service(
            actix_files::Files::new("/", static_folder)
                .show_files_listing()
                .index_file("index.html"),
        );
    })
}
