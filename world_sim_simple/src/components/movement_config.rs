/// Configurable movement speeds for different unit types
///
/// This allows units to have different movement speeds based on their
/// type, status, terrain, equipment, etc. Movement speed is defined
/// in ticks per tile.
use bevy::prelude::*;

/// Movement speed configuration for a unit
#[derive(Component, Clone, Debug, Reflect)]
pub struct MovementSpeed {
    /// Base ticks required to move one tile (lower = faster)
    pub base_ticks_per_tile: u32,

    /// Current speed modifier (1.0 = normal, 2.0 = twice as fast, 0.5 = half speed)
    pub speed_modifier: f32,

    /// Movement type affects which terrain can be traversed
    pub movement_type: MovementType,

    /// Whether unit can move diagonally
    pub can_move_diagonally: bool,

    /// Maximum tiles that can be moved in a single action
    pub max_move_distance: u32,
}

impl Default for MovementSpeed {
    fn default() -> Self {
        Self {
            base_ticks_per_tile: 3, // Standard speed: 3 ticks per tile
            speed_modifier: 1.0,
            movement_type: MovementType::Walking,
            can_move_diagonally: true,
            max_move_distance: 100, // Effectively unlimited
        }
    }
}

impl MovementSpeed {
    /// Create a fast unit (e.g., scout, messenger)
    pub fn fast() -> Self {
        Self {
            base_ticks_per_tile: 2, // Faster: 2 ticks per tile
            speed_modifier: 1.0,
            movement_type: MovementType::Walking,
            can_move_diagonally: true,
            max_move_distance: 100,
        }
    }

    /// Create a slow unit (e.g., heavily armored, carrying heavy load)
    pub fn slow() -> Self {
        Self {
            base_ticks_per_tile: 5, // Slower: 5 ticks per tile
            speed_modifier: 1.0,
            movement_type: MovementType::Walking,
            can_move_diagonally: true,
            max_move_distance: 100,
        }
    }

    /// Create a very slow unit (e.g., siege equipment, caravan)
    pub fn very_slow() -> Self {
        Self {
            base_ticks_per_tile: 10, // Very slow: 10 ticks per tile
            speed_modifier: 1.0,
            movement_type: MovementType::Walking,
            can_move_diagonally: false, // Can only move orthogonally
            max_move_distance: 20,
        }
    }

    /// Create a flying unit (can move over obstacles)
    pub fn flying() -> Self {
        Self {
            base_ticks_per_tile: 2, // Fast movement
            speed_modifier: 1.0,
            movement_type: MovementType::Flying,
            can_move_diagonally: true,
            max_move_distance: 100,
        }
    }

    /// Create an aquatic unit (can only move in water)
    pub fn aquatic() -> Self {
        Self {
            base_ticks_per_tile: 3,
            speed_modifier: 1.0,
            movement_type: MovementType::Swimming,
            can_move_diagonally: true,
            max_move_distance: 100,
        }
    }

    /// Calculate actual ticks needed for one tile of movement
    pub fn get_ticks_per_tile(&self) -> u32 {
        // Apply speed modifier and ensure at least 1 tick
        ((self.base_ticks_per_tile as f32 / self.speed_modifier).max(1.0)) as u32
    }

    /// Apply a temporary speed modifier (stacks multiplicatively)
    pub fn apply_modifier(&mut self, modifier: f32) {
        self.speed_modifier *= modifier;
    }

    /// Reset speed modifier to normal
    pub fn reset_modifier(&mut self) {
        self.speed_modifier = 1.0;
    }

    /// Check if unit can traverse a specific terrain type
    pub fn can_traverse(&self, terrain: TerrainType) -> bool {
        match self.movement_type {
            MovementType::Walking => terrain.is_walkable(),
            MovementType::Flying => !terrain.is_solid(), // Can fly over water, not through walls
            MovementType::Swimming => terrain.is_water(),
            MovementType::Amphibious => terrain.is_walkable() || terrain.is_water(),
            MovementType::Burrowing => !terrain.is_void(),
            MovementType::Ethereal => true, // Can move through anything
        }
    }

    /// Get movement cost for specific terrain (in additional ticks)
    pub fn get_terrain_cost(&self, terrain: TerrainType) -> u32 {
        if !self.can_traverse(terrain) {
            return u32::MAX; // Cannot traverse
        }

        match self.movement_type {
            MovementType::Walking => terrain.walking_cost(),
            MovementType::Flying => 0, // No terrain penalties when flying
            MovementType::Swimming => terrain.swimming_cost(),
            MovementType::Amphibious => {
                if terrain.is_water() {
                    terrain.swimming_cost()
                } else {
                    terrain.walking_cost()
                }
            }
            MovementType::Burrowing => terrain.burrowing_cost(),
            MovementType::Ethereal => 0, // No terrain penalties
        }
    }
}

/// Different types of movement that affect terrain traversal
#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum MovementType {
    /// Normal ground movement
    Walking,
    /// Can fly over obstacles and water
    Flying,
    /// Can only move in water
    Swimming,
    /// Can move on land and water
    Amphibious,
    /// Can move underground
    Burrowing,
    /// Can move through solid objects
    Ethereal,
}

/// Simplified terrain type for movement calculations
#[derive(Clone, Copy, Debug, PartialEq, Reflect)]
pub enum TerrainType {
    Grass,
    Forest,
    Hills,
    Mountains,
    Sand,
    Swamp,
    ShallowWater,
    DeepWater,
    Road,
    Stone,
    Wall,
    Void,
}

impl TerrainType {
    pub fn is_walkable(&self) -> bool {
        !matches!(
            self,
            TerrainType::DeepWater | TerrainType::Wall | TerrainType::Void | TerrainType::Mountains
        )
    }

    pub fn is_water(&self) -> bool {
        matches!(self, TerrainType::ShallowWater | TerrainType::DeepWater)
    }

    pub fn is_solid(&self) -> bool {
        matches!(self, TerrainType::Wall | TerrainType::Mountains)
    }

    pub fn is_void(&self) -> bool {
        matches!(self, TerrainType::Void)
    }

    /// Additional movement cost for walking units (0 = no penalty)
    pub fn walking_cost(&self) -> u32 {
        match self {
            TerrainType::Road => 0, // Bonus: actually reduces movement cost
            TerrainType::Grass | TerrainType::Stone => 0,
            TerrainType::Sand => 1, // +1 tick penalty
            TerrainType::Forest => 1,
            TerrainType::Hills => 2, // +2 ticks penalty
            TerrainType::Swamp => 3, // +3 ticks penalty
            TerrainType::ShallowWater => 2,
            _ => u32::MAX, // Cannot walk here
        }
    }

    pub fn swimming_cost(&self) -> u32 {
        match self {
            TerrainType::ShallowWater => 0,
            TerrainType::DeepWater => 0,
            TerrainType::Swamp => 1, // Harder to swim in swamp
            _ => u32::MAX,           // Cannot swim here
        }
    }

    pub fn burrowing_cost(&self) -> u32 {
        match self {
            TerrainType::Grass | TerrainType::Sand | TerrainType::Swamp => 0,
            TerrainType::Forest | TerrainType::Hills => 1,
            TerrainType::Stone | TerrainType::Road => 2,
            TerrainType::Mountains => 3,
            _ => u32::MAX, // Cannot burrow here
        }
    }
}

/// Movement status effects that can modify speed
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct MovementEffects {
    /// Unit is slowed (mud, web, ice, etc.)
    pub slowed: Option<f32>,
    /// Unit is hasted (magic, stimulants, etc.)
    pub hasted: Option<f32>,
    /// Unit is encumbered by weight
    pub encumbered: Option<f32>,
    /// Unit is exhausted
    pub exhausted: Option<f32>,
    /// Unit is injured
    pub injured: Option<f32>,
}

impl MovementEffects {
    /// Calculate total speed modifier from all effects
    pub fn get_total_modifier(&self) -> f32 {
        let mut modifier = 1.0;

        if let Some(slow) = self.slowed {
            modifier *= slow;
        }
        if let Some(haste) = self.hasted {
            modifier *= haste;
        }
        if let Some(encumber) = self.encumbered {
            modifier *= encumber;
        }
        if let Some(exhaust) = self.exhausted {
            modifier *= exhaust;
        }
        if let Some(injury) = self.injured {
            modifier *= injury;
        }

        modifier.max(0.1) // Minimum 10% speed
    }

    /// Apply encumbrance based on inventory weight
    pub fn update_encumbrance(&mut self, weight_ratio: f32) {
        if weight_ratio > 0.9 {
            self.encumbered = Some(0.5); // 50% speed when nearly full
        } else if weight_ratio > 0.75 {
            self.encumbered = Some(0.75); // 75% speed when heavily loaded
        } else if weight_ratio > 0.5 {
            self.encumbered = Some(0.9); // 90% speed when moderately loaded
        } else {
            self.encumbered = None; // No penalty
        }
    }

    /// Apply exhaustion based on energy level
    pub fn update_exhaustion(&mut self, energy: f32) {
        if energy < 0.1 {
            self.exhausted = Some(0.3); // 30% speed when exhausted
        } else if energy < 0.3 {
            self.exhausted = Some(0.6); // 60% speed when tired
        } else if energy < 0.5 {
            self.exhausted = Some(0.8); // 80% speed when somewhat tired
        } else {
            self.exhausted = None;
        }
    }
}
