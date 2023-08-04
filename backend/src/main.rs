#![allow(unused)]

pub use self::error::{Error, Result};
use std::net::SocketAddr;

use axum::{Router, Json, response::IntoResponse,middleware::{MapResponse, self}, response::Html, response::Response,routing::{post, get,get_service}, http::StatusCode, ServiceExt, extract::{Path, Query}};
//use reqwest::header::AUTHORIZATION;
use serde_json::json;
use serde::{Deserialize, Serialize};
//use tower_http::trace::TraceLayer;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;
mod ctx;

#[tokio::main]
async fn main() {
  println!("Hello, world!");

  let mc = model::ModelController::new().await;

  let routes_apis = web::routes_tickets::routes(mc.clone().unwrap())
    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

  let routes_all = Router::new()
  .merge(routes_hello())
  .merge(web::routes_login::routes())
  .nest("/api", routes_apis)
  .layer(middleware::map_response(main_response_mapper))
  .layer(CookieManagerLayer::new())
  .fallback_service(routes_static());

   // .layer(TraceLayer::new_for_http());

   let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
   println!("Listening on {}", addr);
   axum::Server::bind(&addr)
       .serve(routes_all.into_make_service())
       .await
       .unwrap();
}

fn routes_hello() -> Router{
    Router::new()
    .route("/hello",get(handler_hello))
    .route("/hello2/:name",get(handler_hello2))
}
fn routes_static() -> Router{
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
async fn main_response_mapper(res: Response) -> Response{
  println!("-->> {:<12} - main_response_mapper", "RES_MAPPER");

  println!();
  res
}
//region: ---Routes Hello
#[derive(Debug,Deserialize)]
struct HelloParams {
    name: Option<String>,
}
//e.g. `/hello?name=Ruud`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello - {params:?}", "HANDLER"); 
    let name = params.name.as_deref().unwrap_or( "World");
  Html( format!("Hello, <strong>{name}!</strong>"))
}
//e.g. `/hello2/Ruud`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello - {name:?}", "HANDLER"); 
  Html( format!("Hello, <strong>{name}!</strong>"))
}

