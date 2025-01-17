use core::panic;
use std::hash::Hash;

use rustc_hash::FxHashMap;

/// A registry is just like a map but supposed to be
/// used with an enum of IDs. It panics in many cases
/// to make sure items are created and deleted in a planned manner.
/// You can also think of it as a cache.
///
/// Use-cases: Registering and caching bindgroups, shaders, audio, etc..
///
/// The user is heavily adviced to do something like:
/// ```rust,no_run
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// enum ExampleId {
///     A,
///     B,
///     C,
/// }
/// type ExampleRegistry = Registry<ExampleId, BindGroup>;
/// ```
pub struct Registry<ID: Hash + Eq, T> {
    items: FxHashMap<ID, T>,
}

impl<ID: Hash + Eq, T> Default for Registry<ID, T> {
    fn default() -> Self {
        Self {
            items: FxHashMap::default(),
        }
    }
}

impl<ID: Hash + Eq, T> Registry<ID, T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Panics if the item already exists.
    pub fn insert(&mut self, id: ID, item: T) {
        if self.items.insert(id, item).is_some() {
            panic!("Item collision in registry.");
        }
    }

    /// Panics if the item does not yet exist.
    pub fn overwrite(&mut self, id: ID, item: T) {
        if self.items.insert(id, item).is_none() {
            panic!("Item not found in registry but overwrite was called.");
        }
    }

    /// Does not panic.
    pub fn insert_or_overwrite(&mut self, id: ID, item: T) {
        self.items.insert(id, item);
    }

    /// Panics if the item does not exist.
    pub fn remove(&mut self, id: &ID) -> T {
        self.items.remove(id).expect("Item not found in registry.")
    }

    /// Does not panic.
    pub fn try_remove(&mut self, id: &ID) -> Option<T> {
        self.items.remove(id)
    }

    /// Panics if the item does not exist.
    pub fn get(&self, id: &ID) -> &T {
        self.items.get(id).expect("Item not found in registry.")
    }

    /// Does not panic.
    pub fn try_get(&self, id: &ID) -> Option<&T> {
        self.items.get(id)
    }

    /// Does not panic
    pub fn exists(&self, id: &ID) -> bool {
        self.items.contains_key(id)
    }
}
