//! Building definition loader from Lua scripts

use bevy::prelude::*;
use bevy::prelude::AssetServer;
use super::types::{LuaScript, ScriptCollection};
use std::collections::HashMap;
use world_sim_interface::BuildingType;

/// Component to mark entities with building scripts
#[derive(Component)]
pub struct BuildingScript {
    pub script_path: String,
    pub loaded: bool,
}

/// Lua-defined building
#[derive(Clone, Debug, Reflect)]
pub struct LuaBuilding {
    pub id: String,
    pub name: String,
    pub category: String,
    pub tier: u32,
    pub size: BuildingSize,
    pub cost: HashMap<String, u32>,
    pub build_time: u32,
    pub workers_required: u32,
    pub max_health: f32,
    pub properties: HashMap<String, f32>,
    pub description: String,
}

#[derive(Clone, Debug, Reflect)]
pub struct BuildingSize {
    pub width: u32,
    pub height: u32,
}

/// Building registry for Lua-defined buildings
#[derive(Resource, Default)]
pub struct BuildingRegistry {
    buildings: HashMap<String, LuaBuilding>,
    categories: HashMap<String, Vec<String>>,
    upgrade_paths: HashMap<String, String>,
}

impl BuildingRegistry {
    pub fn register(&mut self, building: LuaBuilding) {
        // Add to category index
        self.categories
            .entry(building.category.clone())
            .or_insert_with(Vec::new)
            .push(building.id.clone());
        
        // Store building
        self.buildings.insert(building.id.clone(), building);
    }
    
    pub fn register_upgrade(&mut self, from: String, to: String) {
        self.upgrade_paths.insert(from, to);
    }
    
    pub fn get(&self, id: &str) -> Option<&LuaBuilding> {
        self.buildings.get(id)
    }
    
    pub fn get_upgrade(&self, id: &str) -> Option<&String> {
        self.upgrade_paths.get(id)
    }
    
    pub fn get_by_category(&self, category: &str) -> Vec<&LuaBuilding> {
        self.categories
            .get(category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.buildings.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    pub fn get_by_tier(&self, tier: u32) -> Vec<&LuaBuilding> {
        self.buildings
            .values()
            .filter(|b| b.tier == tier)
            .collect()
    }
}

/// Command event to trigger building script reloading
#[derive(Event)]
pub struct ReloadBuildingScriptsCommand;

/// System to load building scripts on command
pub fn load_building_scripts(
    mut commands: Commands,
    mut reload_events: EventReader<ReloadBuildingScriptsCommand>,
    asset_server: Res<AssetServer>,
    existing_scripts: Query<Entity, With<BuildingScript>>,
) {
    // Only load when commanded
    if reload_events.is_empty() {
        return;
    }
    
    reload_events.clear();
    
    // Clear existing scripts
    for entity in existing_scripts.iter() {
        commands.entity(entity).despawn();
    }
    
    // Load building definition scripts
    let building_scripts = vec![
        "scripts/buildings/buildings.lua",
        "scripts/buildings/military.lua",
        "scripts/buildings/infrastructure.lua",
        "scripts/buildings/decorations.lua",
    ];
    
    for script_path in building_scripts {
        commands.spawn((
            BuildingScript {
                script_path: script_path.to_string(),
                loaded: false,
            },
            ScriptCollection::<LuaScript>::default(),
        ));
        
        tracing::info!("Loading building script: {}", script_path);
    }
}

/// System to process loaded building scripts
pub fn process_building_scripts(
    mut scripts: Query<(Entity, &mut BuildingScript, &ScriptCollection<LuaScript>)>,
    mut building_registry: ResMut<BuildingRegistry>,
) {
    for (entity, mut script, collection) in scripts.iter_mut() {
        if !script.loaded && !collection.scripts.is_empty() {
            script.loaded = true;
            tracing::debug!("Building script loaded: {}", script.script_path);
        }
    }
}

/// Building functionality components
#[derive(Component, Clone, Debug, Reflect)]
pub struct ProductionBuildingComponent {
    pub worker_slots: u32,
    pub current_workers: Vec<Entity>,
    pub production_bonus: f32,
    pub enabled_recipes: Vec<String>,
}

impl ProductionBuildingComponent {
    pub fn new(worker_slots: u32) -> Self {
        Self {
            worker_slots,
            current_workers: Vec::new(),
            production_bonus: 1.0,
            enabled_recipes: Vec::new(),
        }
    }
    
    pub fn add_worker(&mut self, worker: Entity) -> bool {
        if self.current_workers.len() < self.worker_slots as usize {
            self.current_workers.push(worker);
            true
        } else {
            false
        }
    }
    
    pub fn remove_worker(&mut self, worker: Entity) -> bool {
        if let Some(pos) = self.current_workers.iter().position(|&w| w == worker) {
            self.current_workers.remove(pos);
            true
        } else {
            false
        }
    }
    
    pub fn is_full(&self) -> bool {
        self.current_workers.len() >= self.worker_slots as usize
    }
    
    pub fn efficiency(&self) -> f32 {
        let staffing_ratio = self.current_workers.len() as f32 / self.worker_slots as f32;
        staffing_ratio * self.production_bonus
    }
}

/// Storage building component
#[derive(Component, Clone, Debug, Reflect)]
pub struct StorageBuildingComponent {
    pub capacity: u32,
    pub current_stored: u32,
    pub storage_types: Vec<String>,
    pub preservation_bonus: f32,
}

impl StorageBuildingComponent {
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            current_stored: 0,
            storage_types: vec!["all".to_string()],
            preservation_bonus: 1.0,
        }
    }
    
    pub fn can_store(&self, item_type: &str) -> bool {
        self.storage_types.contains(&"all".to_string()) ||
        self.storage_types.contains(&item_type.to_string())
    }
    
    pub fn add_items(&mut self, amount: u32) -> u32 {
        let space = self.capacity - self.current_stored;
        let to_store = amount.min(space);
        self.current_stored += to_store;
        amount - to_store // Return overflow
    }
    
    pub fn remove_items(&mut self, amount: u32) -> u32 {
        let to_remove = amount.min(self.current_stored);
        self.current_stored -= to_remove;
        to_remove
    }
    
    pub fn space_remaining(&self) -> u32 {
        self.capacity - self.current_stored
    }
}

/// Housing building component
#[derive(Component, Clone, Debug, Reflect)]
pub struct HousingBuildingComponent {
    pub population_capacity: u32,
    pub current_residents: Vec<Entity>,
    pub warmth_provided: f32,
    pub morale_bonus: f32,
}

impl HousingBuildingComponent {
    pub fn new(capacity: u32) -> Self {
        Self {
            population_capacity: capacity,
            current_residents: Vec::new(),
            warmth_provided: 20.0,
            morale_bonus: 5.0,
        }
    }
    
    pub fn add_resident(&mut self, resident: Entity) -> bool {
        if self.current_residents.len() < self.population_capacity as usize {
            self.current_residents.push(resident);
            true
        } else {
            false
        }
    }
    
    pub fn remove_resident(&mut self, resident: Entity) -> bool {
        if let Some(pos) = self.current_residents.iter().position(|&r| r == resident) {
            self.current_residents.remove(pos);
            true
        } else {
            false
        }
    }
    
    pub fn occupancy_rate(&self) -> f32 {
        self.current_residents.len() as f32 / self.population_capacity as f32
    }
}

/// Building maintenance component
#[derive(Component, Clone, Debug, Reflect)]
pub struct MaintenanceComponent {
    pub maintenance_resources: HashMap<String, u32>,
    pub maintenance_interval: u32,
    pub ticks_since_maintenance: u32,
    pub condition: f32,
}

impl MaintenanceComponent {
    pub fn new(resources: HashMap<String, u32>, interval: u32) -> Self {
        Self {
            maintenance_resources: resources,
            maintenance_interval: interval,
            ticks_since_maintenance: 0,
            condition: 100.0,
        }
    }
    
    pub fn update(&mut self) {
        self.ticks_since_maintenance += 1;
        
        if self.ticks_since_maintenance >= self.maintenance_interval {
            // Building needs maintenance
            self.condition -= 5.0;
            self.condition = self.condition.max(0.0);
        }
    }
    
    pub fn perform_maintenance(&mut self) {
        self.ticks_since_maintenance = 0;
        self.condition = 100.0;
    }
    
    pub fn is_due(&self) -> bool {
        self.ticks_since_maintenance >= self.maintenance_interval
    }
}

/// System to update building maintenance
pub fn update_building_maintenance(
    mut buildings: Query<(&mut MaintenanceComponent, &mut crate::components::BuildingComponent)>,
) {
    for (mut maintenance, mut building) in buildings.iter_mut() {
        maintenance.update();
        
        // Apply condition effects to building
        if maintenance.condition < 50.0 {
            building.efficiency_modifier = 0.5;
        } else {
            building.efficiency_modifier = maintenance.condition / 100.0;
        }
    }
}

/// Building construction state
#[derive(Component, Clone, Debug, Reflect)]
pub struct ConstructionComponent {
    pub total_work_required: f32,
    pub work_completed: f32,
    pub workers: Vec<Entity>,
    pub materials_delivered: HashMap<String, u32>,
    pub materials_required: HashMap<String, u32>,
}

impl ConstructionComponent {
    pub fn new(work_required: f32, materials: HashMap<String, u32>) -> Self {
        Self {
            total_work_required: work_required,
            work_completed: 0.0,
            workers: Vec::new(),
            materials_delivered: HashMap::new(),
            materials_required: materials,
        }
    }
    
    pub fn add_work(&mut self, amount: f32) {
        self.work_completed = (self.work_completed + amount).min(self.total_work_required);
    }
    
    pub fn deliver_material(&mut self, material: String, amount: u32) {
        *self.materials_delivered.entry(material).or_insert(0) += amount;
    }
    
    pub fn is_complete(&self) -> bool {
        self.work_completed >= self.total_work_required && self.has_all_materials()
    }
    
    pub fn has_all_materials(&self) -> bool {
        for (material, required) in &self.materials_required {
            if self.materials_delivered.get(material).copied().unwrap_or(0) < *required {
                return false;
            }
        }
        true
    }
    
    pub fn progress_percentage(&self) -> f32 {
        (self.work_completed / self.total_work_required) * 100.0
    }
}

/// System to handle building construction
pub fn construction_system(
    mut commands: Commands,
    mut constructions: Query<(Entity, &mut ConstructionComponent, &BuildingType)>,
    workers: Query<&crate::components::WorkerComponent>,
) {
    for (entity, mut construction, building_type) in constructions.iter_mut() {
        // Calculate work done by assigned workers
        let mut work_done = 0.0;
        for worker_entity in &construction.workers {
            if let Ok(worker) = workers.get(*worker_entity) {
                work_done += worker.work_speed;
            }
        }
        
        construction.add_work(work_done);
        
        // Check if construction is complete
        if construction.is_complete() {
            // Remove construction component and mark as complete
            commands.entity(entity).remove::<ConstructionComponent>();
            tracing::info!("Building construction complete: {:?}", building_type);
        }
    }
}