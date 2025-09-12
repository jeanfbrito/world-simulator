use bevy::prelude::*;

/// Tracks which resource a unit has claimed for harvesting
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct ClaimedResource {
    /// The resource entity this unit has claimed
    pub resource_entity: Option<Entity>,
}

impl ClaimedResource {
    pub fn new() -> Self {
        Self {
            resource_entity: None,
        }
    }
    
    /// Claim a resource
    pub fn claim(&mut self, entity: Entity) {
        self.resource_entity = Some(entity);
    }
    
    /// Release the current claim
    pub fn release(&mut self) {
        self.resource_entity = None;
    }
    
    /// Check if unit has a claim
    pub fn has_claim(&self) -> bool {
        self.resource_entity.is_some()
    }
    
    /// Get the claimed entity
    pub fn get_claimed(&self) -> Option<Entity> {
        self.resource_entity
    }
}