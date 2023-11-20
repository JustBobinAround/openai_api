use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use super::key::api_key;

#[derive(Debug)]
pub struct CompletionError {
    message: String,
}

impl CompletionError {
    fn from(msg: &str) -> CompletionError {
        CompletionError {
            message: String::from(msg)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    type_name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<CompletionMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

impl CompletionRequest {
    pub fn new35(messages: Vec<CompletionMessage>) -> CompletionRequest {
        CompletionRequest { 
            model: String::from("gpt-3.5-turbo"), 
            messages,
            response_format: None,
            max_tokens: None,
        }
    }

    pub fn get(&self) -> Result<CompletionResponse, CompletionError>{
        let client = Client::new();
        let url = "https://api.openai.com/v1/chat/completions";
        let response = client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", api_key() )
            .json(&self) // Serialize the JSON body
            .send();

        match response {
            Ok(response) => { 
                match response.json() {
                    Ok(response) => {Ok(response)},
                    Err(_) => { Err(CompletionError::from("Failed to get response")) }
                }
            },
            Err(_) => {
                Err(CompletionError::from("Failed to parse json payload"))
            }
        }
    }

    pub fn messages_to_str(&self) -> String {
        let mut all_msgs = String::new();
        for msg in &self.messages {
            all_msgs = format!("{}\n{}", all_msgs, msg.to_string());
        }

        all_msgs
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CompletionUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionMessage {
    role: String,
    content: String,
}
#[macro_export]
macro_rules! system {
    ($msg:expr) => {
        CompletionMessage::new("system", $msg) 
    };
}
#[macro_export]
macro_rules! assistant{
    ($msg:expr) => {
        CompletionMessage::new("assistant", $msg) 
    };
}
#[macro_export]
macro_rules! user {
    ($msg:expr) => {
        CompletionMessage::new("user",$msg) 
    };
}

impl CompletionMessage {

    pub fn to_string(&self) -> String {
        format!("{}:\n{}", self.role, self.content)
    }

    pub fn new<S: Into<String>>(role: String, content: S) -> CompletionMessage {
        CompletionMessage { 
            role, 
            content: content.into()
        }
    }

}
#[derive(Serialize, Deserialize, Debug)]
struct CompletionChoice {
    index: u32,
    message: CompletionMessage,
    finish_reason: String,
} 

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionResponse {
    choices: Vec<CompletionChoice>,
    created: i32,
    id: String,
    model: String,
    object: String,
    usage: CompletionUsage,
}
impl CompletionResponse {
    pub fn default_choice(&self) -> String {
        let first_choice = match &self.choices.get(0) {
            Some(choice) => {choice.message.to_string()}
            None => {String::from("No Response")}
        };
        first_choice
    }
    pub fn get_choice(&self, index: usize) -> Option<String> {
        let first_choice = match &self.choices.get(index) {
            Some(choice) => {Some(choice.message.to_string())}
            None => {None}
        };
        first_choice
    }
}
