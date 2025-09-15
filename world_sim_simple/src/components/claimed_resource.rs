use bevy::prelude::*;

/// Tracks which resource a unit has claimed for harvesting
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct ClaimedResource {
    /// The resource entity this unit has claimed
    pub resource_entity: Option<Entity>,
    /// The position of the claimed resource
    pub resource_position: Option<(u32, u32)>,
}

impl ClaimedResource {
    pub fn new() -> Self {
        Self {
            resource_entity: None,
            resource_position: None,
        }
    }

    /// Claim a resource with its position
    pub fn claim(&mut self, entity: Entity) {
        self.resource_entity = Some(entity);
    }

    /// Claim a resource with its position
    pub fn claim_with_position(&mut self, entity: Entity, position: (u32, u32)) {
        self.resource_entity = Some(entity);
        self.resource_position = Some(position);
    }

    /// Release the current claim
    pub fn release(&mut self) {
        self.resource_entity = None;
        self.resource_position = None;
    }

    /// Check if unit has a claim
    pub fn has_claim(&self) -> bool {
        self.resource_entity.is_some()
    }

    /// Get the claimed entity
    pub fn get_claimed(&self) -> Option<Entity> {
        self.resource_entity
    }

    /// Get the claimed position
    pub fn get_claimed_position(&self) -> Option<(u32, u32)> {
        self.resource_position
    }
}