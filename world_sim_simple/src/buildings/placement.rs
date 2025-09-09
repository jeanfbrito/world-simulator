use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use super::{BuildingType, BuildingSize};
use crate::tilemap::TerrainType;
use crate::debug::{DebugSystem, DebugLevel};

#[derive(Resource, Default)]
pub struct BuildingPlacementSystem {
    pub preview_building: Option<BuildingType>,
    pub preview_position: Option<(i32, i32)>,
    pub placement_valid: bool,
}

impl BuildingPlacementSystem {
    pub fn start_placement(&mut self, building_type: BuildingType) {
        self.preview_building = Some(building_type);
        self.preview_position = None;
        self.placement_valid = false;
        
        info!("[PLACEMENT] Started placement for {:?}", building_type);
    }
    
    pub fn update_preview(&mut self, position: (i32, i32), is_valid: bool) {
        self.preview_position = Some(position);
        self.placement_valid = is_valid;
    }
    
    pub fn cancel_placement(&mut self) {
        self.preview_building = None;
        self.preview_position = None;
        self.placement_valid = false;
        
        info!("[PLACEMENT] Placement cancelled");
    }
    
    pub fn confirm_placement(&mut self) -> Option<(BuildingType, (i32, i32))> {
        if self.placement_valid {
            if let (Some(building), Some(pos)) = (self.preview_building, self.preview_position) {
                info!("[PLACEMENT] Confirmed placement of {:?} at {:?}", building, pos);
                self.cancel_placement();
                return Some((building, pos));
            }
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacementValidation {
    pub terrain_valid: bool,
    pub space_available: bool,
    pub resources_available: bool,
    pub distance_valid: bool,
}

impl PlacementValidation {
    pub fn is_valid(&self) -> bool {
        self.terrain_valid && 
        self.space_available && 
        self.resources_available && 
        self.distance_valid
    }
    
    pub fn reason(&self) -> &str {
        if !self.terrain_valid {
            "Invalid terrain for building"
        } else if !self.space_available {
            "Not enough space"
        } else if !self.resources_available {
            "Insufficient resources"
        } else if !self.distance_valid {
            "Too far from other buildings"
        } else {
            "Valid placement"
        }
    }
}

pub fn can_place_building(
    building_type: BuildingType,
    position: (i32, i32),
    terrain_map: &[[TerrainType; 64]; 64], // Simplified for now
    existing_buildings: &[(BuildingType, (i32, i32))],
    debug: &DebugSystem,
) -> PlacementValidation {
    let size = building_type.size().tiles();
    
    // Check terrain
    let terrain_valid = check_terrain(position, size, terrain_map);
    
    // Check space
    let space_available = check_space(position, size, existing_buildings);
    
    // For now, assume resources are available
    let resources_available = true;
    
    // Check distance (buildings should be somewhat close to each other)
    let distance_valid = check_distance(position, existing_buildings);
    
    let validation = PlacementValidation {
        terrain_valid,
        space_available,
        resources_available,
        distance_valid,
    };
    
    debug.log(
        DebugLevel::Debug,
        "PLACEMENT",
        &format!("Validation for {:?} at {:?}: {}", 
            building_type, position, validation.reason())
    );
    
    validation
}

fn check_terrain(position: (i32, i32), size: usize, terrain_map: &[[TerrainType; 64]; 64]) -> bool {
    for dx in 0..size {
        for dy in 0..size {
            let x = position.0 + dx as i32;
            let y = position.1 + dy as i32;
            
            if x < 0 || x >= 64 || y < 0 || y >= 64 {
                return false;
            }
            
            let terrain = terrain_map[y as usize][x as usize];
            if !terrain.is_walkable() {
                return false;
            }
        }
    }
    true
}

fn check_space(position: (i32, i32), size: usize, existing_buildings: &[(BuildingType, (i32, i32))]) -> bool {
    for (other_type, other_pos) in existing_buildings {
        let other_size = other_type.size().tiles();
        
        // Check if buildings overlap
        if position.0 < other_pos.0 + other_size as i32 &&
           position.0 + size as i32 > other_pos.0 &&
           position.1 < other_pos.1 + other_size as i32 &&
           position.1 + size as i32 > other_pos.1 {
            return false;
        }
    }
    true
}

fn check_distance(position: (i32, i32), existing_buildings: &[(BuildingType, (i32, i32))]) -> bool {
    // If no buildings exist, first building is always valid
    if existing_buildings.is_empty() {
        return true;
    }
    
    // Check if at least one building is within reasonable distance
    const MAX_DISTANCE: i32 = 10;
    for (_, other_pos) in existing_buildings {
        let distance = ((position.0 - other_pos.0).abs() + (position.1 - other_pos.1).abs());
        if distance <= MAX_DISTANCE {
            return true;
        }
    }
    false
}