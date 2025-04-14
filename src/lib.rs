pub mod core;
pub mod future;
pub mod sync;

mod tokio;
pub use tokio::*;

pub type Result<T> = ::core::result::Result<T, Box<dyn std::error::Error>>;
