use std::env;

use axum::{
    extract::Path,
    Router,
    http::StatusCode,
    routing::get,
};
    

use maud::{ html, Markup, };

#[tokio::main]
async fn main() {
    let site = Router::new()
        .route("/", get(front_page))
    //these routes mainly for debug (for now)
        .route("/create/:domain", get(nc_create))
        .route("/gethosts/:name/:tld", get(nc_get_hosts));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(site.into_make_service())
        .await
        .unwrap();
}

async fn front_page() -> (StatusCode, Markup) {

    ( StatusCode::OK, draw_index(&query_dns().await) )
}

fn draw_index(contents: &str) -> Markup {
    html!((contents))
}

//👽
//👽👽👽👽👽👽👽👽👽👽👽👽👽👽👽👽👽👽👽👽👽👽👽👽👽

//CODE SO SLOPPY
//TODO: REFACTOR 💋
async fn nc_create(
    Path(domain): Path<String>
) -> (StatusCode, Markup) {
    let api_user = env::var("APIUSER").unwrap();
    let api_key = env::var("APIKEY").unwrap();
    let user_name = env::var("USERNAME").unwrap();
    let client_ip = env::var("CLIENTIP").unwrap();

    //TODO: This is not correct, need to use the format specified at
    //https://www.namecheap.com/support/api/methods/domains/create/
    //there has got to be a better way to do this 💔
    let call = format!("https://api.sandbox.namecheap.com/xml.response?ApiUser={api_user}&ApiKey={api_key}&UserName={user_name}&ClientIp={client_ip}&Command=namecheap.domains.create&DomainName={domain}&Years=1&RegistrantFirstName=Hey&RegistrantLastName=Soteria&RegistrantAddress1=1815%20Central&RegistrantCity=Wichita&RegistrantStateProvince=Kansas&RegistrantPostalCode=67214&RegistrantCountry=US&RegistrantPhone=+1.3164487944&RegistrantEmailAddress=netadmin@heysoteria.com&TechFirstName=Hey&TechLastName=Soteria&TechAddress1=1815%20Central&TechCity=Wichita&TechStateProvince=Kansas&TechPostalCode=67214&TechCountry=US&TechPhone=+1.3164487944&TechEmailAddress=netadmin@heysoteria.com&AdminFirstName=Hey&AdminLastName=Soteria&AdminAddress1=1815%20Central&AdminCity=Wichita&AdminStateProvince=Kansas&AdminPostalCode=67214&AdminCountry=US&AdminPhone=+1.3164487944&AdminEmailAddress=netadmin@heysoteria.com&AuxBillingFirstName=Hey&AuxBillingLastName=Soteria&AuxBillingAddress1=1815%20Central&AuxBillingCity=Wichita&AuxBillingStateProvince=Kansas&AuxBillingPostalCode=67214&AuxBillingCountry=US&AuxBillingPhone=+1.3164487944&AuxBillingEmailAddress=netadmin@heysoteria.com");

    let resp = reqwest::get(&call)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    ( StatusCode::OK, draw_index(&resp) )
}

async fn nc_get_hosts(
        Path((name, tld)): Path<(String, String)>,
) -> (StatusCode, Markup) {
    let api_user = env::var("APIUSER").unwrap();
    let api_key = env::var("APIKEY").unwrap();
    let user_name = env::var("USERNAME").unwrap();
    let client_ip = env::var("CLIENTIP").unwrap();

    let call = format!("https://api.sandbox.namecheap.com/xml.response?ApiUser={api_user}&ApiKey={api_key}&UserName={user_name}&ClientIp={client_ip}&Command=namecheap.domains.dns.getHosts&SLD={name}&TLD={tld}");

    let resp = reqwest::get(&call)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    ( StatusCode::OK, draw_index(&resp) )
}

async fn query_dns() -> String {
    let api_user = env::var("APIUSER").unwrap();
    let api_key = env::var("APIKEY").unwrap();
    let user_name = env::var("USERNAME").unwrap();
    let client_ip = env::var("CLIENTIP").unwrap();

    let call = format!("https://api.sandbox.namecheap.com/xml.response?ApiUser={api_user}&ApiKey={api_key}&UserName={user_name}&ClientIp={client_ip}&Command=namecheap.domains.getList");

    let resp = reqwest::get(&call)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    resp
}
