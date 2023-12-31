use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};

use crate::{Error, Result, web};

pub fn routes() -> Router{
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies:Cookies,payload:Json<LoginPayload>)->Result<Json<Value>>{
    println!("->> {:<12} - api_login","HANDLER");
    //TODO: Implement real db/auth logic
    if payload.username != "demo1" || payload.pwd != "welcome"{
        return Err(Error::LoginFail);
    }

    //FIXME: Implement real auth-token generation/signiture
    cookies.add(Cookie::new(web::AUTH_TOKEN,"user-1.exp.signature")); //format: user-[user-id].[expiration].[signature]

    // Create the success body
    let body = Json(json!({
        "result":{
            "success":true
        }
    }));
    
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}