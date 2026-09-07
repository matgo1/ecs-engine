//! Own Sparse-Set based ECS engine for game development
mod component;
mod entity;
mod sparse_set;
mod storage;
mod world;

pub use component::ComponentConcept;
pub use ecs_engine_derive::Component;
pub use entity::Entity;
pub(crate) use entity::{EntityGen, EntityID};
pub(crate) use sparse_set::SparseSet;
pub(crate) use storage::{AnyStorage, Storage};
pub use world::World;
