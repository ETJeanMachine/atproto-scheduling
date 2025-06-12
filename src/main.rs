mod utils;

use axum::{Router, extract::Query, response::Redirect, routing::get};
use std::{clone, collections::HashMap};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/client-metadata.json", get(client_metadata))
        .route("/oauth", get(oauth))
        .route("/oauth/callback", get(oauth_callback));
    let port = 8080;
    let bind = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(bind).await.unwrap();
    println!("Now running at http://full-gobbler-whole.ngrok-free.app !",);
    println!("Test link: http://full-gobbler-whole.ngrok-free.app/oauth?handle=jeanmachine.dev",);
    println!(
        "Test link: http://full-gobbler-whole.ngrok-free.app/oauth?handle=etjeanmachine.bsky.social",
    );
    println!("Test link: http://full-gobbler-whole.ngrok-free.app/oauth?handle=nat.vg");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Root route accessed!"
}

async fn oauth(Query(params): Query<HashMap<String, String>>) -> Redirect {
    let handle = params.get("handle").unwrap();
    let client_id = "https://full-gobbler-whole.ngrok-free.app/client-metadata.json";
    let oauth_url = match utils::fetch_pds(handle.clone()).await {
        Ok(pds_endpoint) => format!(
            "{}/oauth/authorize?client_id={}&redirect_uri={}&response_type=code&scope=atproto",
            pds_endpoint, client_id, "https://full-gobbler-whole.ngrok-free.app/oauth/callback"
        ),
        Err(_) => return Redirect::to("/"),
    };
    Redirect::to(&oauth_url)
}

async fn oauth_callback() -> &'static str {
    "Callback executed!"
}

async fn client_metadata() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "client_name": "atproto-scheduling",
        "client_uri": "https://atproto-scheduling.com",
        "redirect_uri": "https://atproto-scheduling.com/oauth/callback",
        "response_types": ["code"],
        "grant_types": ["authorization_code"],
        "scope": "atproto"
    }))
}
