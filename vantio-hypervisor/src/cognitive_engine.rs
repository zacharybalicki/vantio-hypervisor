use reqwest::Client;
use serde_json::json;
use std::env;

pub async fn query_vector(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY")
        .expect("[ ∅ VANTIO ORACLE ] FATAL: GEMINI_API_KEY environment variable missing. Aborting cognitive link.");

    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent?key={}",
        api_key
    );

    let payload = json!({
        "contents": [{
            "parts": [{"text": prompt}]
        }]
    });

    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        let res_json: serde_json::Value = response.json().await?;
        if let Some(text) = res_json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
            Ok(text.to_string())
        } else {
            Err("Failed to parse cognitive output from the Phantom Dimension.".into())
        }
    } else {
        let error_text = response.text().await?;
        Err(format!("API Rejection: {}", error_text).into())
    }
}
