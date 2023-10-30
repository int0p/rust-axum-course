#![allow(unused)]

pub use self::error::{Error, Result};

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{Router, middleware};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    //region: --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING ON {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    //endregion: --- Start Server
}

async fn main_response_mapper(res:Response)->Response{
    println!("->> {:<12} - main_response_mapper","RES_MAPPER");
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

//region: --- Handler Hello
/// Handler Hello
/// handler_hello(Query(params):Query<HelloParams>)->impl IntoResponse
/// 입력: 웹 쿼리를 HelloParams로 파싱한 값
/// 출력: HTTP응답
/// Query(params):Query<HelloParams>
///     Query: Extracter, 웹 요청으로부터 쿼리 파라미터 추출
///     Query<HelloParams>: HelloParams 구조체 형태로 쿼리 파라미터를 파싱
///     Query(params): 쿼리 파라미터가 성공적으로 HelloParams로 파싱될 경우 해당 값을 params 변수에 바인딩
/// IntoResponse: Trait for generating responses.
///     Types that implement IntoResponse can be returned from handlers.
///     impl IntoResponse를 반환한다는건, 핸들러가 (HTTP)response가 될 수 있는 값을 반환한다는 것 같다.
/// as_deref(): Option<String> 타입의 값을 Option<&str> 타입으로 변환하는 메서드입니다.
///     as_deref는 Option<T>에서 Option<&T::Target>으로 변환됩니다.
///     여기서 T::Target은 Deref 트레잇의 연관 타입입니다.
///     String에 대한 Deref 구현은 &str을 반환하므로, Option<String>에서 Option<&str>로 변환됩니다.
///     let name = params.name.as_deref().unwrap_or("World!"); 에서 name은 params.name이 가리키는 Stirng을 참조한다.
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g., '/hello?name=Jen'
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong> {name} </strong>"))
}

// e.g., '/hello2/Mike'
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong> {name} </strong>"))
}
//endregion: --- Handler Hello
