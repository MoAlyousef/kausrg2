use anyhow::Result;
use axum::routing::get;
use axum::Router;
use sqlx::sqlite::SqlitePool;
use tower_http::services::ServeDir;

mod data;
mod error;
mod routes;
mod utils;

use routes::*;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite:db/staff.db").await?;
    let s: Vec<data::Data> = sqlx::query_as(r#"SELECT * FROM staff"#)
        .fetch_all(&pool)
        .await?;
    data::STAFF.get_or_init(|| async { s }).await;

    let divisions_routes = Router::new()
        .route("/gs", get(gs))
        .route("/ns", get(ns))
        .route("/cs", get(cs))
        .route("/ts", get(ts))
        .route("/vs", get(vs))
        .route("/pls", get(pls))
        .route("/peds", get(peds));

    let default_routes = Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/mission", get(mission))
        .route("/privacy", get(privacy))
        .route("/contact", get(contact))
        .route("/divisions", get(divisions))
        .route("/faculty", get(faculty))
        .route("/faculty/{id}", get(staff));

    let app = Router::new()
        .merge(default_routes.clone())
        .nest("/ar/", default_routes.clone())
        .nest("/divisions/", divisions_routes.clone())
        .nest("/ar/divisions/", divisions_routes)
        .fallback_service(ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;
    println!("Listening on port 3000");
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
