mod save_state;
mod save_manager;

pub use save_state::{SaveState, EntityData, ChunkData};
pub use save_manager::{SaveManager, SaveError, SaveFormat};

use bevy::prelude::*;
use crate::debug::{DebugSystem, DebugLevel};

pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveManager>()
            .add_systems(Startup, save_load_init_system)
            .add_systems(Update, (
                save_command_system,
                load_command_system,
                autosave_system,
            ));
    }
}

fn save_load_init_system(debug: Res<DebugSystem>) {
    debug.log(
        DebugLevel::Info,
        "SAVE",
        "Save/Load system initialized"
    );
    info!("[SAVE] Save/Load system initialized");
}

fn save_command_system(
    // Keyboard input disabled for headless operation
    // keys: Res<ButtonInput<KeyCode>>,
    save_manager: Res<SaveManager>,
    sim_state: Res<crate::SimulationState>,
    chunk_query: Query<&crate::tilemap::Chunk>,
    entity_query: Query<(Entity, &crate::components::PositionComponent, &crate::components::NameComponent)>,
    building_query: Query<(Entity, &crate::buildings::BuildingComponent)>,
    debug: Res<DebugSystem>,
) {
    // Keyboard saves disabled for headless operation - can be triggered via WebSocket API instead
    if false { // Disabled: keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::KeyS) {
        debug.log(DebugLevel::Info, "SAVE", "Quick save triggered");
        info!("[SAVE] Quick save triggered by Ctrl+S");
        
        // Collect chunk data
        let mut chunks = Vec::new();
        for chunk in chunk_query.iter() {
            let mut tiles = Vec::new();
            for row in chunk.tiles.iter() {
                tiles.push(row.to_vec());
            }
            
            chunks.push(ChunkData {
                coordinate: chunk.coordinate,
                biome: chunk.biome,
                tiles,
                resources: std::collections::HashMap::new(),
            });
        }
        
        // Collect entity data
        let mut entities = Vec::new();
        for (entity, pos, name) in entity_query.iter() {
            entities.push(EntityData {
                id: entity.index() as u64,
                entity_type: save_state::EntityType::Worker,
                position: (pos.x, pos.y, pos.z),
                components: save_state::ComponentData {
                    name: Some(name.name.clone()),
                    health: None,
                    max_health: None,
                    energy: None,
                    max_energy: None,
                    inventory: None,
                    worker_stats: None,
                    current_task: None,
                },
            });
        }
        
        // Collect building data
        let mut buildings = Vec::new();
        for (entity, building) in building_query.iter() {
            buildings.push(save_state::BuildingData {
                id: entity.index() as u64,
                building_type: building.building_type,
                position: (0, 0), // TODO: get actual position
                health: 100.0,
                construction_progress: 1.0,
                is_complete: true,
            });
        }
        
        // Create save state
        let save_state = SaveState::from_world(
            "quicksave".to_string(),
            sim_state.tick,
            chunks,
            entities,
            buildings,
        );
        
        // Save to file
        match save_manager.quick_save(&save_state) {
            Ok(_) => {
                debug.log(DebugLevel::Info, "SAVE", "Quick save successful");
                info!("[SAVE] Quick save successful");
            }
            Err(e) => {
                debug.log(DebugLevel::Error, "SAVE", &format!("Quick save failed: {:?}", e));
                error!("[SAVE] Quick save failed: {:?}", e);
            }
        }
    }
}

fn load_command_system(
    // Keyboard input disabled for headless operation
    // keys: Res<ButtonInput<KeyCode>>,
    save_manager: Res<SaveManager>,
    mut sim_state: ResMut<crate::SimulationState>,
    mut commands: Commands,
    debug: Res<DebugSystem>,
) {
    // Keyboard loads disabled for headless operation - can be triggered via WebSocket API instead
    if false { // Disabled: keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::KeyL) {
        debug.log(DebugLevel::Info, "SAVE", "Quick load triggered");
        info!("[SAVE] Quick load triggered by Ctrl+L");
        
        match save_manager.quick_load() {
            Ok(save_state) => {
                // Update simulation state
                sim_state.tick = save_state.tick;
                
                // TODO: Clear existing entities
                // TODO: Recreate world from save_state.chunks
                // TODO: Spawn entities from save_state.entities
                // TODO: Spawn buildings from save_state.buildings
                
                debug.log(DebugLevel::Info, "SAVE", &format!(
                    "Loaded save: {} chunks, {} entities, {} buildings",
                    save_state.chunks.len(),
                    save_state.entities.len(),
                    save_state.buildings.len()
                ));
                info!("[SAVE] Quick load successful: {} chunks, {} entities, {} buildings",
                    save_state.chunks.len(),
                    save_state.entities.len(),
                    save_state.buildings.len());
            }
            Err(e) => {
                debug.log(DebugLevel::Error, "SAVE", &format!("Quick load failed: {:?}", e));
                error!("[SAVE] Quick load failed: {:?}", e);
            }
        }
    }
}

fn autosave_system(
    time: Res<Time>,
    mut save_manager: ResMut<SaveManager>,
    sim_state: Res<crate::SimulationState>,
    chunk_query: Query<&crate::tilemap::Chunk>,
    entity_query: Query<(Entity, &crate::components::PositionComponent, &crate::components::NameComponent)>,
    building_query: Query<(Entity, &crate::buildings::BuildingComponent)>,
    debug: Res<DebugSystem>,
) {
    save_manager.update_autosave_timer(time.delta_secs());
    
    if save_manager.should_autosave() {
        debug.log(DebugLevel::Info, "SAVE", "Autosave triggered");
        info!("[SAVE] Autosave triggered after {} seconds", save_manager.get_autosave_interval());
        
        // Collect chunk data
        let mut chunks = Vec::new();
        for chunk in chunk_query.iter() {
            let mut tiles = Vec::new();
            for row in chunk.tiles.iter() {
                tiles.push(row.to_vec());
            }
            
            chunks.push(ChunkData {
                coordinate: chunk.coordinate,
                biome: chunk.biome,
                tiles,
                resources: std::collections::HashMap::new(),
            });
        }
        
        // Collect entity data
        let mut entities = Vec::new();
        for (entity, pos, name) in entity_query.iter() {
            entities.push(EntityData {
                id: entity.index() as u64,
                entity_type: save_state::EntityType::Worker,
                position: (pos.x, pos.y, pos.z),
                components: save_state::ComponentData {
                    name: Some(name.name.clone()),
                    health: None,
                    max_health: None,
                    energy: None,
                    max_energy: None,
                    inventory: None,
                    worker_stats: None,
                    current_task: None,
                },
            });
        }
        
        // Collect building data
        let mut buildings = Vec::new();
        for (entity, building) in building_query.iter() {
            buildings.push(save_state::BuildingData {
                id: entity.index() as u64,
                building_type: building.building_type,
                position: (0, 0), // TODO: get actual position
                health: 100.0,
                construction_progress: 1.0,
                is_complete: true,
            });
        }
        
        // Create save state
        let save_state = SaveState::from_world(
            format!("autosave_{}", sim_state.tick),
            sim_state.tick,
            chunks,
            entities,
            buildings,
        );
        
        // Perform autosave
        match save_manager.autosave(&save_state) {
            Ok(_) => {
                debug.log(DebugLevel::Info, "SAVE", "Autosave successful");
                info!("[SAVE] Autosave successful");
            }
            Err(e) => {
                debug.log(DebugLevel::Error, "SAVE", &format!("Autosave failed: {:?}", e));
                error!("[SAVE] Autosave failed: {:?}", e);
            }
        }
        
        save_manager.reset_autosave_timer();
    }
}