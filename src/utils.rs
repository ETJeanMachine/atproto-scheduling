use std::collections::HashMap;

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
    let response = client
        .get("https://cloudflare-dns.com/dns-query")
        .query(&[("name", dns.as_str()), ("type", "TXT")])
        .header("Accept", "application/dns-json")
        .send()
        .await?
        .json::<DnsResponse>()
        .await?;
    let did = response.answer.unwrap()[0]
        .data
        .trim_matches('"')
        .split("=")
        .nth(1)
        .unwrap()
        .to_string();
    Ok(did)
}

#[derive(Serialize, Deserialize)]
struct DidDocument {
    id: String,
    #[serde(rename = "alsoKnownAs")]
    also_known_as: Vec<String>,
    service: Vec<Service>,
}

#[derive(Serialize, Deserialize)]
struct Service {
    id: String,
    #[serde(rename = "serviceEndpoint")]
    service_endpoint: String,
}

pub async fn fetch_pds(handle: String) -> Result<String, Box<dyn std::error::Error>> {
    // fetching the DID for the handle
    let did = resolve_handle(handle.clone()).await?;
    println!("{}", did);
    // matching the identity type to find the DID document
    let identity_type = did.split(":").nth(1).unwrap();
    let url = match identity_type {
        "plc" => format!("https://plc.directory/{}", did),
        "web" => format!("https://{}/.well-known", handle),
        _ => unreachable!(), /* Unsupported identity type */
    };
    let client = reqwest::Client::new();
    // fetching the DID document
    let did_document = client.get(url).send().await?.json::<DidDocument>().await?;
    let service_map: HashMap<String, String> = did_document
        .service
        .into_iter()
        .map(|s| (s.id, s.service_endpoint))
        .collect();
    let pds = service_map.get("#atproto_pds").unwrap();
    Ok(pds.clone())
}
