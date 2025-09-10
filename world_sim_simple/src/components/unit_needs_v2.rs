/// Tick-based unit needs using integer counters
/// 
/// This is the new tick-based implementation that uses integer counters
/// instead of floats, following the Dwarf Fortress model.
/// Updates only happen on simulation ticks, not every frame.

use bevy::prelude::*;
use crate::simulation::*;

/// The new tick-based needs component using counters
#[derive(Component, Clone, Debug, Reflect)]
pub struct UnitNeedsV2 {
    /// Internal counters (not exposed directly)
    hunger_counter: u32,
    energy_counter: u32,
    morale_counter: u32,
    
    /// Cached states for quick checks
    is_hungry: bool,
    is_starving: bool,
    is_tired: bool,
    is_exhausted: bool,
    
    /// Shelter status
    pub has_shelter: bool,
}

impl Default for UnitNeedsV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl UnitNeedsV2 {
    pub fn new() -> Self {
        Self {
            // Start with reasonable defaults
            hunger_counter: 30_000,  // 30% hungry
            energy_counter: 100_000, // Full energy
            morale_counter: 70_000,  // 70% morale
            
            // Update cached states
            is_hungry: false,
            is_starving: false,
            is_tired: false,
            is_exhausted: false,
            
            has_shelter: false,
        }
    }
    
    /// Create from existing float-based needs (for migration)
    pub fn from_floats(hunger: f32, energy: f32, morale: f32, shelter: bool) -> Self {
        let mut needs = Self {
            hunger_counter: float_to_counter(hunger, MAX_HUNGER),
            energy_counter: float_to_counter(energy, MAX_ENERGY),
            morale_counter: float_to_counter(morale, MAX_MORALE),
            is_hungry: false,
            is_starving: false,
            is_tired: false,
            is_exhausted: false,
            has_shelter: shelter,
        };
        needs.update_cached_states();
        needs
    }
    
    // =========================================================================
    // TICK-BASED UPDATE (Called only on simulation ticks)
    // =========================================================================
    
    /// Update needs for one simulation tick
    pub fn tick_update(&mut self) {
        // Hunger increases every tick
        self.hunger_counter = (self.hunger_counter + HUNGER_PER_TICK).min(MAX_HUNGER);
        
        // Energy decreases every tick (unless resting)
        if self.energy_counter >= ENERGY_LOSS_PER_TICK {
            self.energy_counter -= ENERGY_LOSS_PER_TICK;
        } else {
            self.energy_counter = 0;
        }
        
        // Morale affected by other needs
        if self.is_starving || self.is_exhausted {
            // Lose morale when in bad state
            if self.morale_counter >= MORALE_LOSS_PER_TICK * 2 {
                self.morale_counter -= MORALE_LOSS_PER_TICK * 2;
            } else {
                self.morale_counter = 0;
            }
        } else if !self.is_hungry && !self.is_tired && self.has_shelter {
            // Gain morale when all needs met
            self.morale_counter = (self.morale_counter + MORALE_LOSS_PER_TICK).min(MAX_MORALE);
        }
        
        // Update cached states after changes
        self.update_cached_states();
    }
    
    /// Update when resting (called instead of normal tick_update)
    pub fn tick_rest(&mut self) {
        // Still get hungry while resting
        self.hunger_counter = (self.hunger_counter + HUNGER_PER_TICK).min(MAX_HUNGER);
        
        // But recover energy
        self.energy_counter = (self.energy_counter + ENERGY_RECOVERY_PER_TICK).min(MAX_ENERGY);
        
        // Morale slowly improves while resting
        if !self.is_starving {
            self.morale_counter = (self.morale_counter + MORALE_LOSS_PER_TICK / 2).min(MAX_MORALE);
        }
        
        self.update_cached_states();
    }
    
    // =========================================================================
    // ACTIONS (Immediate effects)
    // =========================================================================
    
    /// Eat food to reduce hunger
    pub fn eat_food(&mut self, amount: u32) {
        let reduction = HUNGER_REDUCTION_PER_FOOD * amount;
        if self.hunger_counter >= reduction {
            self.hunger_counter -= reduction;
        } else {
            self.hunger_counter = 0;
        }
        self.update_cached_states();
    }
    
    /// Sleep to recover energy faster
    pub fn sleep(&mut self, ticks: u32) {
        let recovery = ENERGY_RECOVERY_PER_TICK * ticks * 2; // Double recovery when sleeping
        self.energy_counter = (self.energy_counter + recovery).min(MAX_ENERGY);
        self.update_cached_states();
    }
    
    /// Assign shelter
    pub fn assign_shelter(&mut self) {
        self.has_shelter = true;
    }
    
    /// Remove shelter
    pub fn remove_shelter(&mut self) {
        self.has_shelter = false;
    }
    
    // =========================================================================
    // STATE QUERIES (Public API)
    // =========================================================================
    
    /// Get hunger as a float (0.0 = full, 1.0 = starving)
    pub fn hunger(&self) -> f32 {
        counter_to_float(self.hunger_counter, MAX_HUNGER)
    }
    
    /// Get energy as a float (0.0 = exhausted, 1.0 = full)
    pub fn energy(&self) -> f32 {
        counter_to_float(self.energy_counter, MAX_ENERGY)
    }
    
    /// Get morale as a float (0.0 = depressed, 1.0 = happy)
    pub fn morale(&self) -> f32 {
        counter_to_float(self.morale_counter, MAX_MORALE)
    }
    
    /// Quick state checks (cached for performance)
    pub fn is_hungry(&self) -> bool {
        self.is_hungry
    }
    
    pub fn is_starving(&self) -> bool {
        self.is_starving
    }
    
    pub fn is_tired(&self) -> bool {
        self.is_tired
    }
    
    pub fn is_exhausted(&self) -> bool {
        self.is_exhausted
    }
    
    pub fn needs_shelter(&self) -> bool {
        !self.has_shelter
    }
    
    /// Get raw counter values (for save/load and debugging)
    pub fn get_counters(&self) -> (u32, u32, u32) {
        (self.hunger_counter, self.energy_counter, self.morale_counter)
    }
    
    /// Set raw counter values (for save/load)
    pub fn set_counters(&mut self, hunger: u32, energy: u32, morale: u32) {
        self.hunger_counter = hunger.min(MAX_HUNGER);
        self.energy_counter = energy.min(MAX_ENERGY);
        self.morale_counter = morale.min(MAX_MORALE);
        self.update_cached_states();
    }
    
    // =========================================================================
    // INTERNAL HELPERS
    // =========================================================================
    
    /// Update cached state flags based on counter values
    fn update_cached_states(&mut self) {
        self.is_hungry = self.hunger_counter >= HUNGER_THRESHOLD_HUNGRY;
        self.is_starving = self.hunger_counter >= HUNGER_THRESHOLD_STARVING;
        self.is_tired = self.energy_counter <= ENERGY_THRESHOLD_TIRED;
        self.is_exhausted = self.energy_counter <= ENERGY_THRESHOLD_EXHAUSTED;
    }
    
    /// Get a debug string showing current state
    pub fn debug_string(&self) -> String {
        format!(
            "H:{}/{} ({:.0}%) E:{}/{} ({:.0}%) M:{}/{} ({:.0}%) [{}{}{}{}{}]",
            self.hunger_counter, MAX_HUNGER, self.hunger() * 100.0,
            self.energy_counter, MAX_ENERGY, self.energy() * 100.0,
            self.morale_counter, MAX_MORALE, self.morale() * 100.0,
            if self.is_starving { "STARVING " } else if self.is_hungry { "Hungry " } else { "" },
            if self.is_exhausted { "EXHAUSTED " } else if self.is_tired { "Tired " } else { "" },
            if self.has_shelter { "Sheltered" } else { "Homeless" },
            if self.morale_counter < 30_000 { " Unhappy" } else { "" },
            if self.morale_counter > 80_000 { " Happy" } else { "" },
        )
    }
}

// ============================================================================
// MIGRATION HELPERS
// ============================================================================

/// System to migrate from old float-based needs to new counter-based
pub fn migrate_needs_system(
    mut commands: Commands,
    query: Query<(Entity, &crate::components::UnitNeeds), Without<UnitNeedsV2>>,
) {
    for (entity, old_needs) in query.iter() {
        // Create new needs from old
        let new_needs = UnitNeedsV2::from_floats(
            old_needs.hunger,
            old_needs.energy,
            old_needs.morale,
            old_needs.shelter,
        );
        
        // Add new component (keep old for now during transition)
        commands.entity(entity).insert(new_needs);
        
        println!("Migrated entity {:?} to tick-based needs", entity);
    }
}