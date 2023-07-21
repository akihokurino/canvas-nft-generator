pub use once_cell::sync::Lazy;
use std::fmt;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::{Mutex, OnceCell};

pub struct LoadOnce<T> {
    cell: OnceCell<T>,
    init: Mutex<Option<Pin<Box<dyn Future<Output = T> + Send>>>>,
}

impl<T: Debug> Debug for LoadOnce<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Lazy")
            .field("cell", &self.cell)
            .field("init", &"..")
            .finish()
    }
}

impl<T> LoadOnce<T> {
    pub fn lazy(init: impl Future<Output = T> + Send + 'static) -> Self {
        LoadOnce {
            cell: OnceCell::const_new(),
            init: Mutex::new(Some(Box::pin(init))),
        }
    }

    pub fn eager(value: T) -> Self {
        LoadOnce {
            cell: OnceCell::new_with(Some(value)),
            init: Mutex::new(None),
        }
    }

    pub async fn get(&self) -> &T {
        self.cell
            .get_or_init(|| async { self.init.lock().await.take().unwrap().await })
            .await
    }
}

impl<T, E: Clone> LoadOnce<Result<T, E>> {
    pub async fn try_get(&self) -> Result<&T, E> {
        self.get().await.as_ref().map_err(Clone::clone)
    }
}
