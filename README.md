# ecs-engine

A small, dependency-free Entity-Component-System library for Rust, built around
sparse-set storage and generational entity IDs.

There's no scheduler, no plugin system -- just a `World` you poke entities and
components into, and iterate over. It's meant to be read in an afternoon and
extended to fit whatever you're building.

## Features

- **Generational entities** -- a despawned entity's ID can be recycled without a
  stale `Entity` handle from before the despawn silently resolving to the new
  occupant.
- **Sparse-set component storage** -- each component type lives in its own
  `SparseSet<T>`, giving O(1) insert/get/remove and dense, cache-friendly
  iteration over existing components.
- **Type-erased storage map** -- `World` keeps one `Storage<T>` per component
  type behind a `TypeId`-keyed map, so component types don't need to be
  registered or listed up front.
- **`#[derive(Component)]`** -- generates the marker `impl ComponentConcept`
  for a type, so you don't have to write it by hand.
- **Zero runtime dependencies** -- only `std` (the `ecs-engine-derive` proc-macro
  crate pulls in `syn`/`quote`, but only at compile time).

## Installation

Not published on crates.io yet -- pull it in as a path or git dependency:

```toml
[dependencies]
ecs-engine = { git = "https://github.com/matgo1/ecs-engine" }
```

or, if you're working in the same workspace:

```toml
[dependencies]
ecs-engine = { path = "../ecs-engine" }
```

## Quick start

```rust
use ecs_engine::{Component, World};

#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct Velocity { dx: f32, dy: f32 }

fn main() {
    let mut world = World::default();

    let player = world.create_entity();
    world.add_component(player, Position { x: 0.0, y: 0.0 });
    world.add_component(player, Velocity { dx: 1.0, dy: 0.5 });

    // Iterate every entity that has both a Position and a Velocity.
    for (entity, pos, vel) in world.query2::<Position, Velocity>() {
        println!("{:?} at ({}, {}) moving by ({}, {})", entity.id(), pos.x, pos.y, vel.dx, vel.dy);
    }

    if let Some(pos) = world.get_component_mut::<Position>(player) {
        pos.x += 1.0;
    }

    world.remove_component::<Velocity>(player);
    world.despawn_entity(player);
}
```

## Core concepts

### Entities

An `Entity` is just an `EntityID` (a recycled slot index) paired with an
`EntityGen` (a generation counter). `World` hands them out with
`create_entity()` and reclaims them with `despawn_entity()`:

- On despawn, the entity's components are removed from every storage, its
  generation counter is bumped, and its ID is pushed onto a free list.
- On the next `create_entity()`, that ID is reused with the bumped generation.
- Any `Entity` handle created before the despawn now carries a stale
  generation, so `World` treats it as dead (`get_component`, `add_component`,
  etc. become no-ops / return `None`) instead of silently pointing at whoever
  now holds that ID.

### Components

A component is any `'static` type that implements the marker trait
`ComponentConcept`. The easiest way to get there is `#[derive(Component)]`:

```rust
#[derive(Component)]
struct Position { x: f32, y: f32 }
```

which expands to:

```rust
impl ComponentConcept for Position {}
```

You can still write that `impl` by hand if you'd rather not pull in the
derive macro.

### Storage & SparseSet

Each component type gets its own `Storage<T>`, backed by a `SparseSet<T>`:

- `sparse` maps an entity's raw ID to an index into the dense arrays (or
  `usize::MAX` if the entity has no component of that type).
- `dense` / `dense_keys` hold the actual component values and their owning
  entity IDs, packed contiguously so iteration never touches empty slots.
- `remove` is a swap-remove, so it's O(1) at the cost of not preserving
  iteration order.

`World` stores one `Box<dyn AnyStorage>` per component type in a
`HashMap<TypeId, _>`, and downcasts back to the concrete `Storage<T>` on
access. This is what lets you call `add_component::<Position>(...)` for any
type without registering it anywhere first.

### World

`World` is the single entry point for everything above:

| Method | Description |
| --- | --- |
| `create_entity()` | Allocate a new entity (reusing a freed ID if one exists). |
| `despawn_entity(entity)` | Remove all of an entity's components and free its ID. |
| `add_component::<T>(entity, component)` | Attach (or overwrite) a component on an entity. |
| `get_component::<T>(entity)` | Borrow a component, if the entity is alive and has one. |
| `get_component_mut::<T>(entity)` | Mutably borrow a component. |
| `remove_component::<T>(entity)` | Remove and return a component. |
| `iter_components::<T>()` | Iterate every `(Entity, &T)` for a single component type. |
| `query2::<A, B>()` | Iterate every `(Entity, &A, &B)` for entities that have both. |

`query2` is implemented as a filter over `iter_components::<A>()`, looking up
`B` on each match -- it favors simplicity over building out a join for every
arity.

## Roadmap / known limitations

- Only two-component queries (`query2`) exist today; wider joins (`query3+`)
  aren't implemented yet.
- No systems/scheduler -- you drive the update loop yourself.
- `World::despawn_entity` walks every registered storage on each despawn,
  which is fine for typical component counts but isn't free.
- No serialization support.
- `#[derive(Component)]` only implements the marker trait; there's no
  attribute support (e.g. for opting into extra behavior per component).

Contributions and issues are welcome.

## LICENSE

MIT
