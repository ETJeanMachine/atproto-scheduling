use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ResolveHandleResponse {
    did: String,
}

#[derive(Serialize, Deserialize)]
struct DnsResponse {
    #[serde(rename = "Answer")]
    answer: Option<Vec<DnsRecord>>,
}

#[derive(Serialize, Deserialize)]
struct DnsRecord {
    data: String,
}

pub async fn resolve_handle(handle: String) -> Result<String, Box<dyn std::error::Error>> {
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
        .json::<DnsResponse>()
        .await?;
    let did = response.answer.unwrap()[0]
        .data
        .split("=")
        .nth(1)
        .unwrap()
        .to_string();
    Ok(did)
}
