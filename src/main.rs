use anyhow::Result;
use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;
use rinja::Template;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "../templates/layout.html")]
struct LayoutTemplate<'a> {
    entry: &'a str,
}

#[derive(Template)]
#[template(path = "../templates/index.html")]
struct IndexTemplate {
}

#[derive(Template)]
#[template(path = "../templates/about.html")]
struct AboutTemplate {
}

#[tokio::main]
async fn main() -> Result<()> {
	let app = Router::new()
		.route("/", get(index))
        .route("/index", get(index))
        .route("/about", get(about))
		.nest_service("/static", ServeDir::new("static"));

	let listener = tokio::net::TcpListener::bind("localhost:8000").await?;
    println!("Listening on port 8000");
	axum::serve(listener, app.into_make_service()).await?;
	Ok(())
}

async fn index() -> Response {
    let index = IndexTemplate {};
    let layout = LayoutTemplate { entry: &index.render().unwrap() };
    Html(layout.render().unwrap()).into_response()
}

async fn about() -> Response {
    let about = AboutTemplate {};
    let layout = LayoutTemplate { entry: &about.render().unwrap() };
    Html(layout.render().unwrap()).into_response()
}
