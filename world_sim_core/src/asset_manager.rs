//! Asset Pack Manager for switchable game configurations

use std::path::{Path, PathBuf};
use std::fs;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for an asset pack
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PackConfig {
    pub pack: PackMetadata,
    pub features: PackFeatures,
    pub balance: BalanceSettings,
    pub world_generation: WorldGenSettings,
    pub performance: PerformanceSettings,
    pub ui: UISettings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PackMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PackFeatures {
    pub z_levels: bool,
    pub temperature_simulation: bool,
    pub fluid_dynamics: bool,
    pub weather_system: bool,
    pub seasons: bool,
    pub complex_crafting: bool,
    pub material_properties: bool,
    
    // Optional features
    pub moods_and_artifacts: Option<bool>,
    pub nobles_and_mandates: Option<bool>,
    pub military_squads: Option<bool>,
    pub trade_caravans: Option<bool>,
    pub castle_building: Option<bool>,
    pub siege_equipment: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BalanceSettings {
    pub starting_population: u32,
    pub starting_supplies: u32,
    pub difficulty: String,
    
    // Optional settings
    pub embark_points: Option<u32>,
    pub starting_gold: Option<u32>,
    pub map_size: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorldGenSettings {
    #[serde(default)]
    pub biome_types: Vec<String>,
    #[serde(default)]
    pub terrain_types: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PerformanceSettings {
    pub update_rate: u32,
    pub max_pathfinding_distance: Option<u32>,
    pub max_units: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UISettings {
    pub default_view: String,
    pub zoom_levels: Vec<f32>,
}

/// Resource for managing asset packs
#[derive(Resource)]
pub struct AssetPackManager {
    pub active_pack: String,
    pub pack_path: PathBuf,
    pub common_path: PathBuf,
    pub config: Option<PackConfig>,
    pub available_packs: HashMap<String, PackMetadata>,
}

impl Default for AssetPackManager {
    fn default() -> Self {
        Self {
            active_pack: "dwarf_fortress".to_string(),
            pack_path: PathBuf::from("assets/packs/dwarf_fortress"),
            common_path: PathBuf::from("assets/common"),
            config: None,
            available_packs: HashMap::new(),
        }
    }
}

impl AssetPackManager {
    /// Create a new asset pack manager
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.scan_available_packs();
        
        // Load default pack from environment or use dwarf_fortress
        let default_pack = std::env::var("WORLD_SIM_PACK")
            .unwrap_or_else(|_| "dwarf_fortress".to_string());
            
        if let Err(e) = manager.load_pack(&default_pack) {
            tracing::error!("Failed to load default pack '{}': {}", default_pack, e);
        }
        
        manager
    }
    
    /// Scan for all available asset packs
    pub fn scan_available_packs(&mut self) {
        self.available_packs.clear();
        
        let packs_dir = Path::new("assets/packs");
        if let Ok(entries) = fs::read_dir(packs_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let pack_name = entry.file_name().to_string_lossy().to_string();
                    let config_path = entry.path().join("config.toml");
                    
                    if config_path.exists() {
                        if let Ok(config_str) = fs::read_to_string(&config_path) {
                            if let Ok(config) = toml::from_str::<PackConfig>(&config_str) {
                                self.available_packs.insert(pack_name.clone(), config.pack);
                                tracing::info!("Found asset pack: {}", pack_name);
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Load a specific asset pack
    pub fn load_pack(&mut self, pack_name: &str) -> Result<(), String> {
        let pack_path = PathBuf::from(format!("assets/packs/{}", pack_name));
        
        if !pack_path.exists() {
            return Err(format!("Pack '{}' does not exist", pack_name));
        }
        
        let config_path = pack_path.join("config.toml");
        if !config_path.exists() {
            return Err(format!("Pack '{}' missing config.toml", pack_name));
        }
        
        // Load configuration
        let config_str = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config: {}", e))?;
            
        let config = toml::from_str::<PackConfig>(&config_str)
            .map_err(|e| format!("Failed to parse config: {}", e))?;
        
        // Update manager state
        self.active_pack = pack_name.to_string();
        self.pack_path = pack_path;
        self.config = Some(config);
        
        tracing::info!("Loaded asset pack: {}", pack_name);
        Ok(())
    }
    
    /// Get path to a script file
    pub fn get_script_path(&self, script: &str) -> PathBuf {
        self.pack_path.join("scripts").join(script)
    }
    
    /// Get path to a data file
    pub fn get_data_path(&self, data_file: &str) -> PathBuf {
        self.pack_path.join("data").join(data_file)
    }
    
    /// Get path to a balance file
    pub fn get_balance_path(&self, balance_file: &str) -> PathBuf {
        self.pack_path.join("balance").join(balance_file)
    }
    
    /// Get path to a common asset
    pub fn get_common_path(&self, asset: &str) -> PathBuf {
        self.common_path.join(asset)
    }
    
    /// Check if a feature is enabled in the current pack
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        if let Some(config) = &self.config {
            match feature {
                "z_levels" => config.features.z_levels,
                "temperature" => config.features.temperature_simulation,
                "fluids" => config.features.fluid_dynamics,
                "weather" => config.features.weather_system,
                "seasons" => config.features.seasons,
                "complex_crafting" => config.features.complex_crafting,
                "materials" => config.features.material_properties,
                "moods" => config.features.moods_and_artifacts.unwrap_or(false),
                "nobles" => config.features.nobles_and_mandates.unwrap_or(false),
                "military" => config.features.military_squads.unwrap_or(true),
                "trade" => config.features.trade_caravans.unwrap_or(true),
                _ => false,
            }
        } else {
            false
        }
    }
    
    /// Get all scripts in a category
    pub fn get_scripts_in_category(&self, category: &str) -> Vec<PathBuf> {
        let scripts_dir = self.pack_path.join("scripts").join(category);
        let mut scripts = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&scripts_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("lua") {
                    scripts.push(path);
                }
            }
        }
        
        scripts
    }
    
    /// Get a balance value
    pub fn get_balance_value(&self, key: &str) -> Option<u32> {
        if let Some(config) = &self.config {
            match key {
                "starting_population" => Some(config.balance.starting_population),
                "starting_supplies" => Some(config.balance.starting_supplies),
                "embark_points" => config.balance.embark_points,
                "starting_gold" => config.balance.starting_gold,
                _ => None,
            }
        } else {
            None
        }
    }
}

/// Plugin for managing asset packs
pub struct AssetPackPlugin;

impl Plugin for AssetPackPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetPackManager::new())
           .add_systems(Startup, setup_asset_pack)
           .add_event::<SwitchPackEvent>()
           .add_systems(Update, handle_pack_switch);
    }
}

/// Event to switch asset packs at runtime
#[derive(Event)]
pub struct SwitchPackEvent {
    pub pack_name: String,
}

/// System to set up the initial asset pack
fn setup_asset_pack(
    manager: Res<AssetPackManager>,
) {
    if let Some(config) = &manager.config {
        tracing::info!(
            "Active pack: {} v{} by {}",
            config.pack.name,
            config.pack.version,
            config.pack.author
        );
        tracing::info!("Description: {}", config.pack.description);
        
        // Log enabled features
        tracing::info!("Features enabled:");
        if config.features.z_levels { tracing::info!("  - Z-levels"); }
        if config.features.temperature_simulation { tracing::info!("  - Temperature"); }
        if config.features.fluid_dynamics { tracing::info!("  - Fluids"); }
        if config.features.complex_crafting { tracing::info!("  - Complex crafting"); }
    }
}

/// System to handle pack switching
fn handle_pack_switch(
    mut events: EventReader<SwitchPackEvent>,
    mut manager: ResMut<AssetPackManager>,
    mut reload_commands: EventWriter<crate::scripting::commands::ReloadAllScriptsCommand>,
) {
    for event in events.read() {
        match manager.load_pack(&event.pack_name) {
            Ok(()) => {
                tracing::info!("Switched to pack: {}", event.pack_name);
                // Trigger reload of all scripts
                reload_commands.send(crate::scripting::commands::ReloadAllScriptsCommand);
            }
            Err(e) => {
                tracing::error!("Failed to switch pack: {}", e);
            }
        }
    }
}