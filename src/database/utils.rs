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

// Helper function to create `reqwest` client to simplify making requests to Supabase
fn build_request_client(url: &str, request_type: RequestTypes) -> RequestBuilder {
    let client = reqwest::Client::new();

    let header_map = build_header_map();

    match request_type {
        RequestTypes::GET => client.get(url).headers(header_map),
        RequestTypes::POST => client.post(url).headers(header_map),
    }
}

// Three helper functions to easily build URLs
fn build_url_from_base(url: String) -> String {
    let project_ref = env::var("PROJECT_REF").unwrap();

    format!("https://{}.supabase.co/{}", project_ref, url)
}

fn build_auth_url(url: String) -> String {
    build_url_from_base(format!("/auth/v1/{}", url))
}

fn build_rest_url(url: String) -> String {
    build_url_from_base(format!("/rest/v1/{}", url))
}
