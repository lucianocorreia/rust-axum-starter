#![allow(unused)]
use std::net::SocketAddr;

use crate::model::ModelController;

pub use self::error::{Error, Result};

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod context;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelController
    let mc = ModelController::new().await?;

    let routes_api = web::routes_tasks::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region:    --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!(">> Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!(">> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();

    res
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region:    --- Handlers

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!(">> {:<12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("<h1>Hello, <strong>{name}</strong>!</h1>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!(">> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("<h1>Hello2, <strong>{name}</strong>!</h1>"))
}

// endregion: --- Handlers
