// Utility functions to simplify making requests to Supabase

use std::env;

use reqwest::{header::HeaderMap, RequestBuilder};

// Add two headers to authenticate requests to Supabase
fn build_header_map() -> HeaderMap {
    let mut headers = HeaderMap::new();

    let supabase_anon_key = env::var("SUPABASE_KEY").unwrap();

    headers.insert("apiKey", supabase_anon_key.parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", supabase_anon_key).parse().unwrap(),
    );

    headers
}

enum RequestTypes {
    GET,
    POST,
}

fn build_request_client(url: &str, request_type: RequestTypes) -> RequestBuilder {
    let client = reqwest::Client::new();

    let header_map = build_header_map();

    match request_type {
        RequestTypes::GET => client.get(url).headers(header_map),
        RequestTypes::POST => client.post(url).headers(header_map),
    }
}
