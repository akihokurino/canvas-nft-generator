pub mod contract;
pub mod token;
pub mod wallet;

use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(Debug, Ord, PartialOrd)]
pub struct Id<E> {
    id: String,
    _phantom: PhantomData<E>,
}

impl<E> PartialEq<Self> for Id<E> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<E> Eq for Id<E> {}

impl<E> Hash for Id<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<E> Clone for Id<E> {
    fn clone(&self) -> Self {
        self.id.clone().into()
    }
}

impl<E> From<String> for Id<E> {
    fn from(id: String) -> Self {
        Self::new(id)
    }
}

impl<E> Id<E> {
    pub fn new<I: Into<String>>(id: I) -> Self {
        Self {
            id: id.into(),
            _phantom: PhantomData,
        }
    }

    pub fn generate() -> Self {
        Self::new(uuid::Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        self.id.as_str()
    }

    pub fn to_string(&self) -> String {
        self.id.as_str().to_string()
    }
}

pub mod time {
    use chrono::{DateTime, Local};

    pub type LocalDateTime = DateTime<Local>;
}
