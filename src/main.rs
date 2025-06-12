mod utils;

use axum::{Router, extract::Query, response::Redirect, routing::get};
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

async fn oauth(Query(params): Query<HashMap<String, String>>) -> Redirect {
    let handle = params.get("handle").unwrap();
    let oauth_url = match utils::fetch_pds(handle.clone()).await {
        Ok(pds_endpoint) => format!("{}/oauth", pds_endpoint),
        Err(_) => return Redirect::to("/"),
    };
    let client = reqwest::Client::new();
    let client_id_url = format!("{}/client", oauth_url);
    let client_id = client
        .post(client_id_url)
        .query(&[("name", "atproto-scheduling")])
        .send()
        .await
        .unwrap();
    println!("{:?}", client_id);
    Redirect::to(&oauth_url)
}

async fn oauth_callback() -> &'static str {
    "Callback executed!"
}
