use std::sync::Arc;

use tokio::task::JoinHandle;

#[must_use]
#[derive(Debug, Clone)]
pub struct TaskGuard<T> {
    #[allow(unused)]
    inner: Arc<TaskGuardInner<T>>,
}

impl<T> From<JoinHandle<T>> for TaskGuard<T> {
    fn from(handle: JoinHandle<T>) -> Self {
        let inner = TaskGuardInner { handle };
        Self { inner: Arc::new(inner) }
    }
}

#[derive(Debug)]
pub struct TaskGuardInner<T> {
    handle: JoinHandle<T>,
}

impl<T> Drop for TaskGuardInner<T> {
    fn drop(&mut self) {
        self.handle.abort();
    }
}
