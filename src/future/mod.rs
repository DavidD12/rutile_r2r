pub mod node;
pub use node::*;

pub mod publisher;
pub use publisher::*;

pub mod client;
pub use client::*;

use std::sync::Arc;

pub use crate::Result;

pub type SyncMutex<T> = Arc<std::sync::Mutex<T>>;
pub type FutureMutex<T> = Arc<futures::lock::Mutex<T>>;

pub trait MutexInterface<T> {
    fn create(value: T) -> Self;
}

impl<T> MutexInterface<T> for SyncMutex<T> {
    fn create(value: T) -> Self {
        Arc::new(std::sync::Mutex::new(value))
    }
}

impl<T> MutexInterface<T> for FutureMutex<T> {
    fn create(value: T) -> Self {
        Arc::new(futures::lock::Mutex::new(value))
    }
}
