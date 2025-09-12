use bevy::prelude::*;

// Marker components for resources
// These are still used by other systems
#[derive(Component)]
pub struct TreeTag;

#[derive(Component)]
pub struct RockTag;

#[derive(Component)]
pub struct BerryBushTag;