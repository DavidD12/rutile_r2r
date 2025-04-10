pub mod node;
pub use node::*;

pub mod publisher;
pub use publisher::*;

pub mod client;
pub use client::*;

use std::sync::Arc;

pub use crate::Result;

pub type SMutex<T> = Arc<std::sync::Mutex<T>>;
pub type TMutex<T> = Arc<tokio::sync::Mutex<T>>;

pub trait MutexInterface<T> {
    fn create(value: T) -> Self;
}

impl<T> MutexInterface<T> for SMutex<T> {
    fn create(value: T) -> Self {
        Arc::new(std::sync::Mutex::new(value))
    }
}

impl<T> MutexInterface<T> for TMutex<T> {
    fn create(value: T) -> Self {
        Arc::new(tokio::sync::Mutex::new(value))
    }
}
