use std::env;

use axum::{
    Router,
    http::StatusCode,
    routing::get,
};
    

use maud::{ html, Markup, };

#[tokio::main]
async fn main() {
    let site = Router::new()
        .route("/", get(front_page));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(site.into_make_service())
        .await
        .unwrap();
}

async fn front_page() -> (StatusCode, Markup) {

    ( StatusCode::OK, draw_index(&querydns().await))
}

fn draw_index(contents: &str) -> Markup {
    html!((contents))
}

async fn querydns() -> String {
    let api_user = env::var("APIUSER").unwrap();
    let api_key = env::var("APIKEY").unwrap();
    let user_name = env::var("USERNAME").unwrap();
    let client_ip = env::var("CLIENTIP").unwrap();

    let call = format!("https://api.sandbox.namecheap.com/xml.response?ApiUser={api_user}&ApiKey={api_key}&UserName={user_name}&ClientIp={client_ip}&Command=namecheap.domains.check&DomainList=heysoteria.com");

    let resp = reqwest::get(&call)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    resp
}
