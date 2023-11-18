use std::env;

pub fn api_key() -> String {
    let api_key = env::var("OPENAI_API_KEY")
        .expect("Failed to get OPENAI_API_KEY env variable");

    format!("Bearer {}", api_key)
}
