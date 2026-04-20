pub mod api;
pub mod future;
pub mod future_mono;
mod macros;
pub mod mono;
pub mod tokio;
pub mod tokio_mono;

pub type Result<T> = ::core::result::Result<T, Box<dyn std::error::Error>>;

pub trait MutexCreate<T> {
    fn create(value: T) -> Self;
}

pub trait MutexLockErr<T> {
    fn lock_err(&self, name: &'static str) -> crate::Result<std::sync::MutexGuard<'_, T>>;
}

pub trait MutexLockOrLog<T> {
    fn lock_or_log(&self, name: &'static str) -> std::sync::MutexGuard<'_, T>;
}

//------------------------- std::sync::Mutex -------------------------
use std::sync::Arc;

pub type SMutex<T> = Arc<std::sync::Mutex<T>>;

impl<T> crate::MutexCreate<T> for SMutex<T> {
    fn create(value: T) -> Self {
        Arc::new(std::sync::Mutex::new(value))
    }
}
impl<T> MutexLockErr<T> for SMutex<T> {
    fn lock_err(&self, name: &'static str) -> crate::Result<std::sync::MutexGuard<'_, T>> {
        self.lock()
            .map_err(|e| std::io::Error::other(format!("{name} poisoned: {e}")).into())
    }
}

impl<T> MutexLockOrLog<T> for SMutex<T> {
    fn lock_or_log(&self, name: &'static str) -> std::sync::MutexGuard<'_, T> {
        self.lock().unwrap_or_else(|e| {
            eprintln!("[WARN] mutex '{name}' poisoned: {e}");
            e.into_inner()
        })
    }
}
