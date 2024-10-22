// Utility functions to simplify making requests to Supabase

use std::env;

use reqwest::header::HeaderMap;

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
