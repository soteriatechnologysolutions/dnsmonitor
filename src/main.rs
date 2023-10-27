use std::env;
use std::net::{Ipv4Addr, Ipv6Addr};

use quick_xml::events::Event;
use quick_xml::reader::Reader;

use axum::{
    extract::Path,
    Router,
    http::StatusCode,
    routing::get,
};

use maud::{ html, Markup, PreEscaped, DOCTYPE };

//TODO: CAA records, SRV records
//I think I'll need to do NS records too
enum DnsRecord {
    A { host: String, value: Ipv4Addr },
    AAAA { host: String, value: Ipv6Addr },
    MX { host: String, priority: u32, value: String },
    CNAME { host: String, value: String },
    TXT { host: String, value: String },
}

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
    html! { 
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "DNS Monitor Tool" } 
            }
            body {
                pre {
                    (PreEscaped(contents))
                }
            }
        }
    }
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

    // let resp = env::var("DEBUG_RESPONSE").unwrap();

    let mut reader = Reader::from_str(&resp);
    let mut buf = Vec::new();
    let mut skip_buf = Vec::new();
    let mut domain_list = String::new();

    //TODO: Using if let will make this much more readable
    //I hate xml
    //TODO: I think the recursion might not actually be unnecessary?
    //look into it later
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"DomainGetListResult" => {
                        loop {
                            skip_buf.clear();
                            match reader.read_event_into(&mut skip_buf) {
                                //empty is for tags formatted like < attribute="xyz" />
                                Ok(Event::Empty(f)) => {
                                    match f.name().as_ref() {
                                        b"Domain" => {
                                            for i in f.attributes()
                                            {
                                                let att = i.unwrap();
                                                if att.key.local_name().as_ref() == b"Name"
                                                {
                                                    let domain = att.unescape_value().unwrap();
                                                    println!("{}", domain);
                                                    domain_list.push_str(domain.as_ref());
                                                    domain_list.push_str("\n");
                                                }
                                            }
                                        },
                                        _ => (),
                                    }
                                },
                                Ok(Event::End(e)) => {
                                    if e.name().as_ref() == b"DomainGetListResult" {
                                        break;
                                    }
                                }
                                Err(e) => panic!("Error at position {}: {:?}",
                                                  reader.buffer_position(), e),
                                _ => (),
                            }
                        }
                    },
                    _ => (),
                }
            },
            _ => (),
        }
    }

    buf.clear();
    domain_list
}
