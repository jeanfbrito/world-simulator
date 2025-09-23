use crate::ai::WorkerAI;
use crate::components::{
    ClaimedResource, EnergyComponent, HasEnergy, HasFood, HasStone, HasWood, HealthComponent,
    IsHungry, IsWorking, NameComponent, PeasantConfig, PeasantTag, PositionComponent, TilesWalked,
    UnitInventory, UnitLocation, UnitNeeds, UnitOwnership, UnitStats, UnitTag, UnitType,
    UnitWorkState, WorkProgress, WorkSpeed,
};
use crate::TileEntity;
use bevy::prelude::*;
use colored::Colorize;
use rand::Rng;

// OLD SPAWNING SYSTEM REMOVED - USE PACK-BASED SPAWNING ONLY
// The SpawningPlugin and initial_unit_spawn_system have been removed to prevent
// conflicts with the new pack-based entity spawning system.

// All spawning is now handled by the pack-based system in systems/entity_spawning.rs
// which reads entity definitions from Lua files in assets/packs/dev-world/data/entities/