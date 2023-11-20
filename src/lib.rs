pub mod prelude {
    pub use crate::{assistant, system, user};
    pub use crate::completions::{CompletionRequest, CompletionResponse};
    pub use crate::embeddings::{EmbeddingRequest, EmbeddingResponse};
}
pub mod completions;
pub mod embeddings;
pub mod key;
