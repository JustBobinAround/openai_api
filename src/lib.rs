pub mod prelude {
    pub use crate::assistant;
    pub use crate::system;
    pub use crate::user;
    pub use crate::completions::*;
    pub use crate::embeddings::*;
}
pub mod completions;
pub mod embeddings;
pub mod key;
