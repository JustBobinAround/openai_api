pub mod prelude {
    pub use crate::{gpt35, assistant, system, user};
    pub use crate::completions::{CompletionRequest, CompletionResponse, CompletionMessage};
    pub use crate::embeddings::{EmbeddingRequest, EmbeddingResponse, EmbeddingData};
    pub use crate::key::set_api_key;
}
pub mod completions;
pub mod embeddings;
pub mod key;
