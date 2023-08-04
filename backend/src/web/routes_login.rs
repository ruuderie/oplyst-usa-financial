use crate::{Error,Result, web};
use serde::Deserialize;
use serde_json::{Value, json};
use axum::{Json, body, extract::Extension, Router, routing::post};
use tower_cookies::{Cookies, Cookie};

pub fn routes() ->Router {
    Router::new()
        .route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies,payload: Json<LoginPayload>) -> Result<Json<Value>>{
    println!("-->> {:<12} - api_login", "HANDLER"); 
    
    // TODO: Implement real db/auth logic.
    if payload.username != "admin" || payload.password != "admin" {
        return Err(Error::LoginFailed);
    }
    //FIXME: Implement real auth-token generation/signature.
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
    // create the success body
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));
    Ok(body)
}


#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

struct LoginResponse {
    username: String,
    password: String,
}