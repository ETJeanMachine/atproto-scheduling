mod utils;

use axum::{Router, extract::Query, routing::get};
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
    match utils::resolve_handle(handle.to_string()).await {
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

async fn oauth_callback() -> &'static str {
    "Callback executed!"
}
