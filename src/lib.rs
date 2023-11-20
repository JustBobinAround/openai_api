pub mod prelude {
    pub use crate::{newGPT35, assistant, system, user};
    pub use crate::completions::{CompletionRequest, CompletionResponse, CompletionMessage};
    pub use crate::embeddings::{EmbeddingRequest, EmbeddingResponse, EmbeddingData};
}
pub mod completions;
pub mod embeddings;
pub mod key;
