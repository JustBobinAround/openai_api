# OpenAI Rust API Client

This is a Rust library for interacting with the OpenAI API, specifically
designed for language model completions and text embeddings. This library
also has some helper tools for dealing with embeddings distance.

## Features

### Language Model Completions

The `CompletionRequest` struct allows you to create requests for language model
completions using OpenAI's API. You can easily construct
messages in a conversation format and receive responses. Additionally, macros
have been introduced for convenient message creation.

#### Example Usage:

```rust
use openai_rust::completion::{CompletionRequest, system, assistant, user};

let msg1 = user!("You are a helpful assistant");
let msg2 = assistant!("The meaning of life is...");
let msg3 = user!("what is the meaning of life?");

let request = CompletionRequest::new35(vec![msg1, msg2, msg3]);
let response = request.get();

match response {
    Ok(response) => {
        println!("Model response: {}", response.default_choice());
    },
    Err(error) => {
        eprintln!("Error: {}", error.message);
    },
}
```

### Text Embeddings

The `EmbeddingRequest` struct allows you to obtain text embeddings using
OpenAI's text-embedding-ada-002 model. You can provide a text input, and the
library will return the corresponding embeddings.

#### Example Usage:

```rust
use openai_rust::embedding::EmbeddingRequest;

let input_text = String::from("Hello, how are you?");
let request = EmbeddingRequest::new(input_text);
let response = request.get();

match response {
    Ok(response) => {
        if let Some(embeddings) = response.get_embeddings() {
            println!("Embeddings: {:?}", embeddings);
        }
    },
    Err(error) => {
        eprintln!("Error: {}", error.message);
    },
}
```

## Dependencies

- `reqwest`: A simple HTTP client for Rust.
- `serde`: A framework for serializing and deserializing Rust data structures efficiently.
- rayon: This is used for parallel processing of the embedding vectors when comparing distances.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
reqwest = "0.11"
serde = "1.0"
rayon = "1.5"
```

## Usage

1. Import the library into your Rust project:

```rust
use openai_rust::{completion::*, embedding::*};
```

2. Create requests using the provided structs and methods.

3. Handle responses and errors accordingly.

## License

This library is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Special thanks to the contributors to the used
crates (`reqwest`, `serde`, `rayon`) for their valuable work.

