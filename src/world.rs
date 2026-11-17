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

    /// Get a mutable iterator of all entities with the component paired with the component itself
    pub fn iter_components_mut<ComponentType: ComponentConcept>(
        &mut self,
    ) -> impl Iterator<Item = (Entity, &mut ComponentType)> {
        let generations = &self.generations;
        Self::storage_mut::<ComponentType>(&mut self.storages)
            .into_iter()
            .flat_map(|s| s.iter_mut())
            .map(move |(id, c)| (Entity::new(id, generations[id.0 as usize]), c))
    }

    /// Get an iterator of all entities that have 2 given components
    pub fn query2<A: ComponentConcept, B: ComponentConcept>(
        &self,
    ) -> impl Iterator<Item = (Entity, &A, &B)> {
        assert_ne!(
            TypeId::of::<A>(),
            TypeId::of::<B>(),
            "Components can't be the same"
        );
        self.iter_components::<A>()
            .filter_map(move |(e, a)| self.get_component::<B>(e).map(|b| (e, a, b)))
    }

    /// Get a mutable iterator of all entities that have 2 given components
    pub fn query2_mut<A: ComponentConcept, B: ComponentConcept>(
        &mut self,
    ) -> impl Iterator<Item = (Entity, &mut A, &mut B)> {
        assert_ne!(
            // Check if valid call
            TypeId::of::<A>(),
            TypeId::of::<B>(),
            "Components can't be the same"
        );

        // Get 2 &mut simulantaly
        let [a, b] = self
            .storages
            .get_disjoint_mut([&TypeId::of::<A>(), &TypeId::of::<B>()]);

        let a: Option<&mut Storage<A>> =
            a.and_then(|s| s.as_any_mut().downcast_mut::<Storage<A>>());
        let b: Option<&mut Storage<B>> =
            b.and_then(|s| s.as_any_mut().downcast_mut::<Storage<B>>());

        let generations = &self.generations;
        a.zip(b).into_iter().flat_map(move |(a, b)| {
            let b_components = b.storage.dense.as_mut_ptr();
            let b_set = &b.storage;
            a.iter_mut().filter_map(move |(id, a)| {
                let idx = b_set.dense_index(id)?;
                // SAFETY:
                // 1. the index is valid since dense_index return positions of live components
                // 2. There are never 2 &mut to one B
                // 3. A and B are different storages
                // 4. The pointer can't go stale
                let b = unsafe { &mut *b_components.add(idx) };
                Some((Entity::new(id, generations[id.0 as usize]), a, b))
            })
        })
    }
}

impl World {
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

impl World {
    /// Get mutable storage (not &mut self)
    fn storage_mut<ComponentType: ComponentConcept>(
        storages: &mut HashMap<TypeId, Box<dyn AnyStorage>>,
    ) -> Option<&mut Storage<ComponentType>> {
        storages
            .get_mut(&TypeId::of::<ComponentType>())?
            .as_any_mut()
            .downcast_mut()
    }
}
