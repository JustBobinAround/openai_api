use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

lazy_static!(
    static ref API_KEY: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
);
pub fn set_api_key(new_key: String) {
    if let Ok(mut api_key) = API_KEY.lock() {
        *api_key = new_key.clone(); 
    }
}
pub fn api_key() -> String {
    let api_key = API_KEY.lock().expect("Failed to get API_KEY mutex lock");
    format!("Bearer {}", api_key)
}
