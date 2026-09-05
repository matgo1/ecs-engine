//! Storage
use std::any::Any;

use super::{ComponentConcept, EntityID, SparseSet};

// TODO: need to have a shared method of removing across a bunch of components

pub struct Storage<ComponentType>
where
    ComponentType: ComponentConcept,
{
    pub storage: SparseSet<ComponentType>,
}

impl<ComponentType: ComponentConcept> Storage<ComponentType> {
    pub fn new() -> Self {
        Self {
            storage: SparseSet::new(),
        }
    }

    pub fn add(&mut self, entity: EntityID, component: ComponentType) -> &ComponentType {
        self.storage.insert(entity, component)
    }

    pub fn get(&self, entity: EntityID) -> Option<&ComponentType> {
        self.storage.get(entity)
    }

    pub fn get_mut(&mut self, entity: EntityID) -> Option<&mut ComponentType> {
        self.storage.get_mut(entity)
    }

    pub fn remove(&mut self, entity: EntityID) -> Option<ComponentType> {
        self.storage.remove(entity)
    }

    /// Get an iterator of all entities with the same components paired with component itself
    pub fn iter(&self) -> impl Iterator<Item = (EntityID, &ComponentType)> {
        self.storage
            .dense_keys
            .iter()
            .zip(self.storage.dense.iter())
            .map(|(&k, c)| (EntityID(k as u32), c))
    }
}

pub trait AnyStorage {
    fn untyped_remove(&mut self, entity: EntityID);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<ComponentType: ComponentConcept> AnyStorage for Storage<ComponentType> {
    fn untyped_remove(&mut self, entity: EntityID) {
        self.remove(entity);
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
