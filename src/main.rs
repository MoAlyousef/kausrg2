use anyhow::Result;
use axum::routing::{get, post};
use axum::Router;
use sqlx::sqlite::SqlitePool;
use tower_http::services::ServeDir;

use std::env;

mod data;
mod error;
mod routes;
mod utils;

use routes::*;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let staffs = sqlx::query_as!(data::Record, r#"SELECT * FROM staff"#)
        .fetch_all(&pool)
        .await?;
    data::STAFF.get_or_init(|| async { staffs }).await;

    let divisions_routes = Router::new()
        .route("/gs", get(gs))
        .route("/ns", get(ns))
        .route("/cs", get(cs))
        .route("/ts", get(ts))
        .route("/vs", get(vs))
        .route("/pls", get(pls))
        .route("/peds", get(peds));

    let app = Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/mission", get(mission))
        .route("/privacy", get(privacy))
        .route("/contact", get(contact))
        .route("/faculty", get(faculty))
        .route("/faculty/{id}", get(staff))
        .route("/search", post(search))
        .nest("/divisions", divisions_routes)
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("localhost:8000").await?;
    println!("Listening on port 8000");
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
