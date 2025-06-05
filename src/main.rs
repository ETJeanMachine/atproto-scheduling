use axum::{Json, Router, extract::Query, routing::get};
use regex::Regex;
use serde_json::{Value, json};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/oauth", get(oauth))
        .route("/oauth/callback", get(oauth_callback));
    let port = 8080;
    let bind = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(bind).await.unwrap();
    println!("Now running at http://localhost:{} !", port);
    println!(
        "Test link: http://localhost:{}/oauth?handle=jeanmachine.dev",
        port
    );
    println!(
        "Test link: http://localhost:{}/oauth?handle=etjeanmachine.bsky.social",
        port
    );
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Root route accessed!"
}

async fn oauth(Query(params): Query<HashMap<String, String>>) -> String {
    let handle = params.get("handle").unwrap();
    let re = Regex::new(r"^.+\.(.+\..{2,})$").unwrap();
    let pds_url = match re.captures(handle) {
        Some(capture) => Some(capture.get(1).unwrap().as_str()),
        None => None,
    };
    // checking the end of the handle if there's smth there to see if it's a valid PDS url
    if pds_url.is_some() {
        let client = reqwest::Client::new();
        let url = format!(
            "https://{}/xrpc/com.atproto.identity.resolveHandle",
            pds_url.unwrap()
        );
        let send = client.get(url).query(&[("handle", handle)]).send().await;
        if let Ok(response) = send {
            let json = response.json::<serde_json::Value>().await.unwrap();
            println!("{:?}", json)
        }
    }
    let message = format!("Auth route accessed! Username: {}", handle);
    message
}

async fn oauth_callback() -> &'static str {
    "Callback executed!"
}
