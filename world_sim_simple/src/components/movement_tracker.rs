use bevy::prelude::*;

/// Simple component to track total tiles walked - using u64 to prevent overflow
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct TilesWalked(pub u64);

impl TilesWalked {
    pub fn new() -> Self {
        Self(0)
    }

    /// Add distance to the total (converts float distance to integer tiles)
    pub fn add(&mut self, distance: f32) {
        // Add the distance as integer tiles (rounding to nearest)
        self.0 += distance.round() as u64;
    }

    /// Get formatted string for display
    pub fn display(&self) -> String {
        format!("📏{}t", self.0)
    }
}

// For backward compatibility, keep the old name as an alias
pub type MovementTracker = TilesWalked;
