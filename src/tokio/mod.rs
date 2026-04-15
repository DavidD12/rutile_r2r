pub mod node;
pub use node::*;

pub mod publisher;
pub use publisher::*;

pub mod client;
pub use client::*;

//------------------------- tokio::sync::Mutex -------------------------
use std::sync::Arc;

pub type TMutex<T> = Arc<tokio::sync::Mutex<T>>;

impl<T> crate::MutexCreate<T> for TMutex<T> {
    fn create(value: T) -> Self {
        Arc::new(tokio::sync::Mutex::new(value))
    }
}
