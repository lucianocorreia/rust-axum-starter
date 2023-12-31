use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{web, Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login_handler))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn api_login_handler(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!(">> {:<12} - api_login_handler", "HANDLER");

    // TODO: Implement real db/auth logic.
    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }

    // FIXME: Implement real auth-token gneration/signature.
    cookies.add(Cookie::new(web::AUTH_TOKEN, "DDDuser-1.exp.sign"));

    // Create the success body
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}
