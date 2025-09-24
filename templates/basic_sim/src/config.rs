//! Configuration management for the basic simulation

use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::Result;

/// Main simulation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// World configuration
    pub world: WorldConfig,

    /// Entity configuration
    pub entities: EntityConfig,

    /// Resource configuration
    pub resources: ResourceConfig,

    /// System configuration
    pub systems: SystemConfig,

    /// Simulation settings
    pub simulation: SimulationSettings,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            world: WorldConfig::default(),
            entities: EntityConfig::default(),
            resources: ResourceConfig::default(),
            systems: SystemConfig::default(),
            simulation: SimulationSettings::default(),
        }
    }
}

impl SimulationConfig {
    /// Load configuration from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: SimulationConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate world size
        if self.world.width == 0 || self.world.height == 0 {
            return Err(anyhow::anyhow!("World dimensions must be positive"));
        }

        if self.world.width > 1000 || self.world.height > 1000 {
            return Err(anyhow::anyhow!("World dimensions too large (max 1000x1000)"));
        }

        // Validate entity counts
        if self.entities.peasant_count > 1000 {
            return Err(anyhow::anyhow!("Too many entities (max 1000)"));
        }

        // Validate tick duration
        if self.simulation.tick_duration_ms < 16 {
            return Err(anyhow::anyhow!("Tick duration too short (min 16ms)"));
        }

        if self.simulation.tick_duration_ms > 1000 {
            return Err(anyhow::anyhow!("Tick duration too long (max 1000ms)"));
        }

        // Validate resource densities
        for resource_type in &self.resources.types {
            if resource_type.density < 0.0 || resource_type.density > 1.0 {
                return Err(anyhow::anyhow!(
                    "Resource density must be between 0.0 and 1.0"
                ));
            }
        }

        Ok(())
    }
}

/// World configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    /// World width in tiles
    pub width: u32,

    /// World height in tiles
    pub height: u32,

    /// Random seed for procedural generation
    pub seed: Option<u64>,

    /// Tile size in pixels
    pub tile_size: u32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            seed: None,
            tile_size: 32,
        }
    }
}

/// Entity configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityConfig {
    /// Number of peasants to spawn
    pub peasant_count: usize,

    /// Peasant movement speed
    pub peasant_speed: f32,

    /// Peasant health points
    pub peasant_health: u32,

    /// Peasant inventory size
    pub peasant_inventory_size: usize,
}

impl Default for EntityConfig {
    fn default() -> Self {
        Self {
            peasant_count: 20,
            peasant_speed: 1.0,
            peasant_health: 100,
            peasant_inventory_size: 10,
        }
    }
}

/// Resource configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfig {
    /// Resource type definitions
    pub types: Vec<ResourceType>,

    /// Global resource respawn time
    pub respawn_time_seconds: f64,
}

impl Default for ResourceConfig {
    fn default() -> Self {
        Self {
            types: vec![
                ResourceType {
                    name: "wood".to_string(),
                    density: 0.1,
                    max_per_tile: 5,
                    respawn_time: 30.0,
                    color: "#8B4513".to_string(),
                },
                ResourceType {
                    name: "stone".to_string(),
                    density: 0.05,
                    max_per_tile: 3,
                    respawn_time: 60.0,
                    color: "#808080".to_string(),
                },
                ResourceType {
                    name: "food".to_string(),
                    density: 0.08,
                    max_per_tile: 10,
                    respawn_time: 20.0,
                    color: "#FF6347".to_string(),
                },
            ],
            respawn_time_seconds: 30.0,
        }
    }
}

/// Individual resource type configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceType {
    /// Resource name
    pub name: String,

    /// Spawn density (0.0 to 1.0)
    pub density: f32,

    /// Maximum amount per tile
    pub max_per_tile: u32,

    /// Respawn time in seconds
    pub respawn_time: f64,

    /// Display color
    pub color: String,
}

/// System configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// Movement system settings
    pub movement: SystemSettings,

    /// Resource gathering system settings
    pub gathering: SystemSettings,

    /// Crafting system settings
    pub crafting: SystemSettings,

    /// AI system settings
    pub ai: SystemSettings,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            movement: SystemSettings { enabled: true },
            gathering: SystemSettings { enabled: true },
            crafting: SystemSettings { enabled: true },
            ai: SystemSettings { enabled: true },
        }
    }
}

/// Individual system settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSettings {
    /// Whether the system is enabled
    pub enabled: bool,

    /// System update interval in seconds
    #[serde(default = "default_interval")]
    pub update_interval: f64,

    /// System priority (higher = runs first)
    #[serde(default = "default_priority")]
    pub priority: i32,
}

impl Default for SystemSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            update_interval: default_interval(),
            priority: default_priority(),
        }
    }
}

fn default_interval() -> f64 {
    0.1
}

fn default_priority() -> i32 {
    0
}

/// Simulation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationSettings {
    /// Tick duration in milliseconds
    pub tick_duration_ms: u64,

    /// Maximum number of ticks to run
    pub max_ticks: Option<u64>,

    /// Enable performance monitoring
    pub enable_monitoring: bool,

    /// Enable debug mode
    pub debug_mode: bool,

    /// Save simulation state on exit
    pub save_state_on_exit: bool,

    /// Enable WebSocket server
    pub enable_websocket: bool,

    /// WebSocket port
    pub websocket_port: u16,

    /// Enable save/load functionality
    pub enable_saves: bool,

    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            tick_duration_ms: 100,
            max_ticks: None,
            enable_monitoring: true,
            debug_mode: false,
            save_state_on_exit: false,
            enable_websocket: false,
            websocket_port: 8080,
            enable_saves: true,
            auto_save_interval: 300,
        }
    }
}

/// Performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable performance tracking
    pub enabled: bool,

    /// Metrics collection interval in seconds
    pub metrics_interval: f64,

    /// Enable memory profiling
    pub enable_memory_profiling: bool,

    /// Enable CPU profiling
    pub enable_cpu_profiling: bool,

    /// Performance log level
    pub log_level: String,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_interval: 1.0,
            enable_memory_profiling: false,
            enable_cpu_profiling: false,
            log_level: "info".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SimulationConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation() {
        let mut config = SimulationConfig::default();
        config.world.width = 0;
        assert!(config.validate().is_err());

        config.world.width = 100;
        config.world.height = 2000;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_serialization() {
        let config = SimulationConfig::default();
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: SimulationConfig = toml::from_str(&serialized).unwrap();
        assert_eq!(config.world.width, deserialized.world.width);
    }

    #[test]
    fn test_resource_type_validation() {
        let mut config = SimulationConfig::default();
        config.resources.types[0].density = 1.5;
        assert!(config.validate().is_err());
    }
}