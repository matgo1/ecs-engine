//! Entity and its getters
#[derive(Default, Debug, Clone, Copy)]
pub struct EntityID(pub(crate) u32);

#[derive(Default, Debug, Clone, Copy)]
pub struct EntityGen(pub(crate) u32);

#[derive(Default, Debug, Clone, Copy)]
pub struct Entity {
    id: EntityID,
    generation: EntityGen,
}

impl Entity {
    /// Constructor
    pub fn new(id: EntityID, generation: EntityGen) -> Self {
        Self { id, generation }
    }

    /// Getter of id
    pub(crate) fn id(&self) -> EntityID {
        self.id
    }

    /// Getter of gen
    pub(crate) fn generation(&self) -> EntityGen {
        self.generation
    }
}
