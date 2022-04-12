use super::Store;
use std::{ops::Deref, rc::Rc};

/// Context holding a reference to the store.
pub struct StoreContext<T> {
    pub(crate) store: Rc<super::Store<T>>,
}

impl<T> PartialEq for StoreContext<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.store, &other.store)
    }
}

impl<T> StoreContext<T> {
    /// Creates a new `StoreContext` with the given `initial_state`.
    pub fn new(initial_state: T) -> Self {
        Self {
            store: Rc::new(Store::new(initial_state)),
        }
    }
}

impl<T> Deref for StoreContext<T> {
    type Target = Rc<Store<T>>;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl<T> Clone for StoreContext<T> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
        }
    }
}
