pub mod node;
pub use node::*;

pub mod publisher;
pub use publisher::*;

pub mod client;
pub use client::*;

//------------------------- futures::lock::Mutex -------------------------
use std::sync::Arc;

pub type FMutex<T> = Arc<futures::lock::Mutex<T>>;

impl<T> crate::MutexCreate<T> for FMutex<T> {
    fn create(value: T) -> Self {
        Arc::new(futures::lock::Mutex::new(value))
    }
}
