use axum::{Router, extract::Query, routing::get};
use regex::Regex;
use serde::{Deserialize, Serialize};
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
    println!("Test link: http://localhost:{}/oauth?handle=nat.vg", port);
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Root route accessed!"
}

async fn oauth(Query(params): Query<HashMap<String, String>>) -> String {
    let handle = params.get("handle").unwrap();
    match resolve_handle(handle.to_string()).await {
        Ok(did) => {
            format!("Auth route accessed! Username: {}, DID: {}", handle, did)
        }
        Err(_) => {
            format!(
                "Auth route accessed! Username: {} (failed to resolve)",
                handle
            )
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ResolveHandleResponse {
    did: String,
}

async fn resolve_handle(handle: String) -> Result<String, Box<dyn std::error::Error>> {
    // regex to fetch the last part of the handle to check if that's the pds.
    let re = Regex::new(r"^.+\.(.+\..{2,})$").unwrap();
    let pds_url = match re.captures(handle.as_str()) {
        Some(capture) => Some(capture.get(1).unwrap().as_str()),
        None => None,
    };
    // first check (PDS is in handle)
    let client = reqwest::Client::new();
    if pds_url.is_some() {
        let url = format!(
            "https://{}/xrpc/com.atproto.identity.resolveHandle",
            pds_url.unwrap()
        );
        let did = client
            .get(url)
            .query(&[("handle", handle.clone())])
            .send()
            .await?
            .json::<ResolveHandleResponse>()
            .await?
            .did;
        return Ok(did);
    }
    // second check (set by DNS)
    let dns = format!("_atproto.{}", handle);
    let url = format!("https://cloudflare-dns.com/dns-query?name={}&type=TXT", dns);
    let response = client
        .get(&url)
        .header("Accept", "application/dns-json")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{:?}", response);
    let data = response["Answer"][0]["data"].as_str().unwrap().to_string();
    let did = data.split("=").nth(1).unwrap().to_string();
    Ok(did)
}

async fn oauth_callback() -> &'static str {
    "Callback executed!"
}
