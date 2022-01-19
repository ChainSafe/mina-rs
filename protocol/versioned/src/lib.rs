
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Versioned<T, const V: usize> {
    version: usize,
    t: T,
}

impl<T, const V: usize> Versioned<T, V> {
    pub fn new(t: T) -> Self {
        Self {
            version: V,
            t,
        }
    }

    pub fn inner(self) -> T {
        self.t
    }

    pub fn version(&self) -> usize {
        self.version
    }

}

impl<T, const V: usize> From<T> for Versioned<T, V> {
    fn from(t: T) -> Versioned<T, V> {
        Self::new(t)
    }
}
