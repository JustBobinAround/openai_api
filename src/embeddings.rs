use rayon::prelude::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use super::key::api_key;


#[derive(Debug)]
pub struct EmbeddingError {
    pub message: String,
}

impl EmbeddingError {
    fn from(msg: &str) -> EmbeddingError {
        EmbeddingError {
            message: String::from(msg)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddingRequest {
    input: String,
    model: String,
}

impl EmbeddingRequest {
    pub fn new<S: Into<String>>(input: S) -> EmbeddingRequest {
        EmbeddingRequest { 
            input: input.into(),
            model: String::from("text-embedding-ada-002"),
        }
    }

    pub fn get(&self) -> Result<EmbeddingResponse, EmbeddingError> {
        let client = Client::new();
        let url = "https://api.openai.com/v1/embeddings";
        let response = client
          .post(url)
          .header("Content-Type", "application/json")
          .header("Authorization", api_key() )
          .json(&self) // Serialize the JSON body
          .send();

        match response {
            Ok(response) => {
                match response.json() {
                    Ok(response) => {
                        Ok(response)
                    },
                    Err(_) => {
                        Err(EmbeddingError::from("Failed to parse json payload"))
                    }
                }
            },
            Err(_) => {
                Err(EmbeddingError::from("Failed to get response"))
            }
        }
    }
} 

#[derive(Serialize, Deserialize, Debug)]
struct EmbeddingUsage {
    prompt_tokens: u32,
    total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddingData {
    object: String,
    index: u32,
    embedding: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddingResponse {
    object: String,
    data: Vec<EmbeddingData>,
    model: String,
    usage: EmbeddingUsage,
}

impl EmbeddingResponse {
    pub fn get_embeddings(&self) -> Option<&Vec<f64>> {
        match &self.data.get(0) {
            Some(data) => {Some(&data.embedding)},
            None => {None}
        }
    }

    pub fn get_distance(&self,points: Option<&Vec<f64>>) -> f64{
        match points {
            Some(points_b) => {
                match self.get_embeddings() {
                    Some(points_a) => { 
                        EmbeddingResponse::calc_distances(points_a, points_b)
                    },
                    None => {f64::MAX}
                }
            },
            None => {f64::MAX}
        }
    }

    fn calc_distances(points_a: &Vec<f64>, points_b: &Vec<f64>) -> f64 {
        points_a
            .par_iter()
            .zip(points_b.par_iter())
            .map(|(i, j)| (i-j).abs())
            .sum()
    }
}
