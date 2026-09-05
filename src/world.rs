//! World class, all actions happens here
use std::{any::TypeId, collections::HashMap};

use super::{AnyStorage, ComponentConcept, Entity, EntityGen, EntityID, Storage};

/// World class, all actions happens here
#[derive(Default)]
pub struct World {
    free_ids: Vec<EntityID>,
    generations: Vec<EntityGen>,
    storages: HashMap<TypeId, Box<dyn AnyStorage>>,
}

impl World {
    /// Create new entity
    pub fn create_entity(&mut self) -> Entity {
        // If there's free id -> use it
        if let Some(id) = self.free_ids.pop() {
            Entity::new(id, self.generations[id.0 as usize])
        // Else create new
        } else {
            let id = EntityID(self.generations.len() as u32);
            self.generations.push(EntityGen(0));
            Entity::new(id, EntityGen(0))
        }
    }

    /// Destroy entity and add its id to free
    pub fn despawn_entity(&mut self, entity: Entity) {
        if !self.is_alive(entity) {
            return;
        }
        for storage in self.storages.values_mut() {
            storage.untyped_remove(entity.id());
        }
        self.generations[entity.id().0 as usize].0 += 1;
        self.free_ids.push(entity.id());
    }

    /// Add a component to the entity
    pub fn add_component<ComponentType: ComponentConcept>(
        &mut self,
        entity: Entity,
        component: ComponentType,
    ) {
        if !self.is_alive(entity) {
            return;
        }
        self.get_storage_mut::<ComponentType>()
            .add(entity.id(), component);
    }

    /// Get component from the entity
    pub fn get_component<ComponentType: ComponentConcept>(
        &self,
        entity: Entity,
    ) -> Option<&ComponentType> {
        if !self.is_alive(entity) {
            return None;
        }
        self.get_storage::<ComponentType>()?.get(entity.id())
    }

    /// Get mutable component from the entity
    pub fn get_component_mut<ComponentType: ComponentConcept>(
        &mut self,
        entity: Entity,
    ) -> Option<&mut ComponentType> {
        if !self.is_alive(entity) {
            return None;
        }
        self.get_storage_mut::<ComponentType>().get_mut(entity.id())
    }

    /// Remove component from the entity
    pub fn remove_component<ComponentType: ComponentConcept>(
        &mut self,
        entity: Entity,
    ) -> Option<ComponentType> {
        if !self.is_alive(entity) {
            return None;
        }
        self.get_storage_mut::<ComponentType>().remove(entity.id())
    }

    /// Get an iterator of all entities with the same components paired with component itself
    pub fn iter_components<ComponentType: ComponentConcept>(
        &self,
    ) -> impl Iterator<Item = (Entity, &ComponentType)> {
        self.get_storage::<ComponentType>()
            .into_iter()
            .flat_map(|s| s.iter())
            .map(|(id, c)| (Entity::new(id, self.generations[id.0 as usize]), c))
    }

    pub fn query2<A: ComponentConcept, B: ComponentConcept>(
        &self,
    ) -> impl Iterator<Item = (Entity, &A, &B)> {
        self.iter_components::<A>()
            .filter_map(move |(e, a)| self.get_component::<B>(e).map(|b| (e, a, b)))
    }

    /// Get storage if exist
    fn get_storage<ComponentType: ComponentConcept>(&self) -> Option<&Storage<ComponentType>> {
        self.storages
            .get(&TypeId::of::<ComponentType>())?
            .as_any()
            .downcast_ref::<Storage<ComponentType>>()
    }

    /// Get mutable storage or create
    fn get_storage_mut<ComponentType: ComponentConcept>(&mut self) -> &mut Storage<ComponentType> {
        self.storages
            .entry(TypeId::of::<ComponentType>())
            .or_insert_with(|| Box::new(Storage::<ComponentType>::new()))
            .as_any_mut()
            .downcast_mut()
            .unwrap()
    }

    ///Helper function to check if entity is available
    fn is_alive(&self, entity: Entity) -> bool {
        self.generations[entity.id().0 as usize].0 == entity.generation().0
    }
}
