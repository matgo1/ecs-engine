//! SparseSet system for containg enteties and their comonents
//! At least 6 times faster than HashMap. Thanks Salar (YT)
use super::{ComponentConcept, EntityID};

/// Value that won't be reached
const INVALID: usize = usize::MAX; // Verstappen

pub struct SparseSet<ComponentType>
where
    ComponentType: ComponentConcept,
{
    pub sparse: Vec<usize>,
    pub dense_keys: Vec<usize>,
    pub dense: Vec<ComponentType>,
}

impl<ComponentType: ComponentConcept> SparseSet<ComponentType> {
    /// Constructor
    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense_keys: Vec::new(),
            dense: Vec::new(),
        }
    }

    /// Add new comonent
    pub fn insert(&mut self, key: EntityID, component: ComponentType) -> &ComponentType {
        self.ensure_sparse_size(key);

        let idx: usize = key.0 as usize;

        // If first time for entity -> write in new slot
        if self.sparse[idx] == INVALID {
            self.sparse[idx] = self.dense.len();
            self.dense_keys.push(idx);
            self.dense.push(component);
        } else {
            // if entity has a component -> rewrite it
            self.dense[self.sparse[idx]] = component;
        }
        &self.dense[self.sparse[idx]]
    }

    pub fn get(&self, key: EntityID) -> Option<&ComponentType> {
        let idx: usize = key.0 as usize;

        if !self.is_valid_data(idx) {
            None
        } else {
            Some(&self.dense[self.sparse[idx]])
        }
    }

    pub fn get_mut(&mut self, key: EntityID) -> Option<&mut ComponentType> {
        let idx: usize = key.0 as usize;

        if !self.is_valid_data(idx) {
            None
        } else {
            Some(&mut self.dense[self.sparse[idx]])
        }
    }

    /// Return a component, return if exist
    pub fn remove(&mut self, key: EntityID) -> Option<ComponentType> {
        let idx = key.0 as usize;
        if !self.is_valid_data(idx) {
            None
        } else {
            let dense_idx = self.sparse[idx];
            let last_dense_idx = self.dense.len() - 1;

            // Overwrite Component on index into last component
            let removed = self.dense.swap_remove(dense_idx);
            self.dense_keys.swap_remove(dense_idx);

            self.sparse[idx] = INVALID; // Mark cleared entity

            // Fix if broken
            if dense_idx != last_dense_idx {
                let moved_entity_idx = self.dense_keys[dense_idx];
                self.sparse[moved_entity_idx] = dense_idx;
            }

            Some(removed) // Return Removed
        }
    }

    /// Resize sparse Vector if needed
    fn ensure_sparse_size(&mut self, key: EntityID) {
        let idx: usize = key.0 as usize;
        if idx >= self.sparse.len() {
            self.sparse.resize(idx + 1, INVALID);
        }
    }

    fn is_valid_data(&self, idx: usize) -> bool {
        idx < self.sparse.len() && self.sparse[idx] != INVALID
    }
}
