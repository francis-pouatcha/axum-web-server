
use serde_json::{Value, json};
use axum::{Json, Router, routing::post};
use serde::Deserialize;
use tower_cookies::{Cookies, Cookie};

use crate::{error::{Error, Result}, web::AUTH_TOKEN};

pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: implement real db/auth logic
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFailed);
    }

    // add cookies
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    // Create the success body.
    let body = Json(json!({
        "result":{
            "success": true,
        }
    }));
    Ok(body)
}


#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}