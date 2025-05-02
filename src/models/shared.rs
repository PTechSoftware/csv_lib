use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};

/// ## Shared struct
/// Encapsula un `Arc<Mutex<T>>` con acceso simplificado.
pub struct Shared<T>
where
    T: Default,
{
    guard: Arc<Mutex<T>>,
}

impl<T> Shared<T>
where
    T: Default,
{
    #[inline(always)]
    pub fn new(element: T) -> Self {
        Self {
            guard: Arc::new(Mutex::new(element)),
        }
    }

    #[inline(always)]
    pub fn default() -> Self {
        Self::new(T::default())
    }

    #[inline(always)]
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.guard.lock().unwrap()
    }

    #[inline(always)]
    pub fn arc(&self) -> Arc<Mutex<T>> {
        Arc::clone(&self.guard)
    }
}

impl<T> Deref for Shared<T>
where
    T: Default,
{
    type Target = Arc<Mutex<T>>;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}
