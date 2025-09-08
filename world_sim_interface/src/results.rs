//! Command result types

use serde::{Deserialize, Serialize};

/// Result of executing a command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<serde_json::Value>,
}

impl CommandResult {
    pub fn success() -> Self {
        Self {
            success: true,
            message: None,
            data: None,
        }
    }
    
    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: Some(message.into()),
            data: None,
        }
    }
    
    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// World configuration for creating new worlds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub width: u32,
    pub height: u32,
    pub seed: Option<u64>,
    pub resource_density: f32,
    pub starting_workers: u32,
    pub seasons_enabled: bool,
    pub resource_regeneration: bool,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            seed: None,
            resource_density: 0.1,
            starting_workers: 5,
            seasons_enabled: false,
            resource_regeneration: true,
        }
    }
}