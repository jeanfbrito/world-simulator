//! Pack loading module for Sim Viewer
//!
//! This module handles loading and processing pack files to extract
//! visual definitions and other metadata for the web viewer.

use mlua::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{info, error, warn, debug};
use world_sim_interface::ipc::{VisualRegistry, TileVisual, EntityVisual, PackMetadata as IpcPackMetadata};

/// Pack metadata loaded from pack.lua
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub load_order: Vec<String>,
    pub config: PackConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackConfig {
    pub debug: bool,
    pub validate_strict: bool,
    pub allow_hot_reload: bool,
}

/// Visual configuration for entities and resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualConfig {
    pub color: String,
    pub emoji: Option<String>,
    pub sprite: Option<String>,
    pub size: (f32, f32),
}

/// Resource definition with visual config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDefinition {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub visuals: Option<VisualConfig>,
}

/// Entity definition with visual config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDefinition {
    pub id: String,
    pub name: String,
    pub category: String,
    pub visual: Option<VisualConfig>,
}

/// Item definition with visual config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDefinition {
    pub id: String,
    pub name: String,
    pub category: String,
    pub visual: Option<VisualConfig>,
}

/// Pack loader that handles loading and processing pack files
pub struct PackLoader {
    pack_path: PathBuf,
    lua: Lua,
}

impl PackLoader {
    /// Create a new pack loader for the given pack path
    pub fn new(pack_path: impl AsRef<Path>) -> Result<Self, PackLoaderError> {
        let pack_path = pack_path.as_ref().to_path_buf();
        let lua = Lua::new();

        Ok(Self {
            pack_path,
            lua,
        })
    }

    /// Load pack metadata from pack.lua
    pub fn load_metadata(&self) -> Result<PackMetadata, PackLoaderError> {
        let pack_lua_path = self.pack_path.join("pack.lua");
        if !pack_lua_path.exists() {
            return Err(PackLoaderError::PackNotFound(pack_lua_path));
        }

        let pack_content = fs::read_to_string(&pack_lua_path)
            .map_err(|e| PackLoaderError::IoError(e))?;

        // Load and execute the pack.lua file
        self.lua.load(&pack_content).exec().map_err(|e| PackLoaderError::LuaError(e))?;

        // Extract metadata from global table
        let global: LuaTable = self.lua.globals().get("_G")
            .map_err(|_| PackLoaderError::MissingMetadata("Global table".to_string()))?;

        let pack_table: LuaTable = global.get("pack")
            .map_err(|_| PackLoaderError::MissingMetadata("pack table".to_string()))?;

        let config_table: LuaTable = pack_table.get("config")
            .map_err(|_| PackLoaderError::MissingMetadata("config table".to_string()))?;

        let dependencies: Vec<String> = pack_table.get("dependencies")
            .unwrap_or_else(|_| Vec::new());

        let load_order: Vec<String> = pack_table.get("load_order")
            .unwrap_or_else(|_| Vec::new());

        let metadata = PackMetadata {
            id: pack_table.get("id")?,
            name: pack_table.get("name")?,
            version: pack_table.get("version")?,
            author: pack_table.get("author")?,
            description: pack_table.get("description")?,
            dependencies,
            load_order,
            config: PackConfig {
                debug: config_table.get("debug")?,
                validate_strict: config_table.get("validate_strict")?,
                allow_hot_reload: config_table.get("allow_hot_reload")?,
            },
        };

        info!("Loaded pack metadata: {}", metadata.name);
        Ok(metadata)
    }

    /// Load all definitions from the pack and create visual registry
    pub fn load_visual_registry(&self) -> Result<VisualRegistry, PackLoaderError> {
        let mut registry = VisualRegistry {
            tiles: HashMap::new(),
            entities: HashMap::new(),
            ui_themes: HashMap::new(),
            animations: HashMap::new(),
            sprite_sheets: HashMap::new(),
        };

        // Load tile visuals
        self.load_tile_visuals(&mut registry)?;

        // Load entity visuals
        self.load_entity_visuals(&mut registry)?;

        info!("Loaded visual registry with {} tiles and {} entities",
              registry.tiles.len(), registry.entities.len());

        Ok(registry)
    }

    /// Load tile visual definitions
    fn load_tile_visuals(&self, registry: &mut VisualRegistry) -> Result<(), PackLoaderError> {
        // Add basic tile types with emojis
        registry.tiles.insert("grass".to_string(), TileVisual {
            name: "Grass".to_string(),
            color: "#3a5f3a".to_string(),
            emoji: Some("🌱".to_string()),
            sprite: None,
            animation: None,
            variant_selector: None,
            blocks_movement: false,
            blocks_sight: false,
        });

        registry.tiles.insert("water".to_string(), TileVisual {
            name: "Water".to_string(),
            color: "#1e3a8a".to_string(),
            emoji: Some("🌊".to_string()),
            sprite: None,
            animation: None,
            variant_selector: None,
            blocks_movement: true,
            blocks_sight: false,
        });

        registry.tiles.insert("sand".to_string(), TileVisual {
            name: "Sand".to_string(),
            color: "#c2b280".to_string(),
            emoji: Some("🏖️".to_string()),
            sprite: None,
            animation: None,
            variant_selector: None,
            blocks_movement: false,
            blocks_sight: false,
        });

        registry.tiles.insert("stone".to_string(), TileVisual {
            name: "Stone".to_string(),
            color: "#696969".to_string(),
            emoji: Some("🪨".to_string()),
            sprite: None,
            animation: None,
            variant_selector: None,
            blocks_movement: false,
            blocks_sight: false,
        });

        Ok(())
    }

    /// Load entity visual definitions from pack files
    fn load_entity_visuals(&self, registry: &mut VisualRegistry) -> Result<(), PackLoaderError> {
        // First try to load the comprehensive visual definitions file
        let visual_defs_path = self.pack_path.join("visual_definitions.lua");
        if visual_defs_path.exists() {
            if let Err(e) = self.load_visual_definitions_file(&visual_defs_path, registry) {
                warn!("Failed to load visual definitions file: {}", e);
            } else {
                info!("Successfully loaded visual definitions");
                return Ok(());
            }
        }

        // Fallback: try to load from individual entity files
        let entities_dir = self.pack_path.join("data").join("entities");
        if entities_dir.exists() {
            if let Ok(entries) = fs::read_dir(&entities_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("lua") {
                        if let Err(e) = self.load_entity_from_file(&path, registry) {
                            warn!("Failed to load entity from {:?}: {}", path, e);
                        }
                    }
                }
            }
        }

        // Add fallback entity visuals
        self.add_fallback_entity_visuals(registry);

        Ok(())
    }

    /// Load comprehensive visual definitions from visual_definitions.lua
    fn load_visual_definitions_file(&self, path: &Path, registry: &mut VisualRegistry) -> Result<(), PackLoaderError> {
        let content = fs::read_to_string(path)
            .map_err(|e| PackLoaderError::IoError(e))?;

        // Execute Lua file
        self.lua.load(&content).exec().map_err(|e| PackLoaderError::LuaError(e))?;

        // Get the visual definitions table (returned directly from the file)
        let visual_defs: LuaTable = self.lua.globals().get("_G")
            .map_err(|_| PackLoaderError::MissingMetadata("Global table".to_string()))?;

        // Load tile visuals
        if let Ok(tiles_table) = visual_defs.get::<_, LuaTable>("tiles") {
            for pair in tiles_table.pairs::<String, LuaTable>() {
                let (tile_id, tile_table) = pair?;
                let tile_visual = self.table_to_tile_visual(&tile_table)?;
                registry.tiles.insert(tile_id, tile_visual);
            }
        }

        // Load entity visuals
        if let Ok(entities_table) = visual_defs.get::<_, LuaTable>("entities") {
            for pair in entities_table.pairs::<String, LuaTable>() {
                let (entity_id, entity_table) = pair?;
                let entity_visual = self.table_to_entity_visual(&entity_table)?;
                registry.entities.insert(entity_id, entity_visual);
            }
        }

        // Load UI themes
        if let Ok(themes_table) = visual_defs.get::<_, LuaTable>("ui_themes") {
            for pair in themes_table.pairs::<String, LuaTable>() {
                let (theme_id, theme_table) = pair?;
                // UI theme loading would go here
                debug!("Loaded UI theme: {}", theme_id);
            }
        }

        // Load animations
        if let Ok(animations_table) = visual_defs.get::<_, LuaTable>("animations") {
            for pair in animations_table.pairs::<String, LuaTable>() {
                let (anim_id, anim_table) = pair?;
                // Animation loading would go here
                debug!("Loaded animation: {}", anim_id);
            }
        }

        Ok(())
    }

    /// Convert Lua table to TileVisual
    fn table_to_tile_visual(&self, table: &LuaTable) -> Result<TileVisual, PackLoaderError> {
        Ok(TileVisual {
            name: table.get("name")?,
            color: table.get("color")?,
            emoji: table.get("emoji").ok(),
            sprite: table.get("sprite").ok(),
            animation: table.get("animation").ok(),
            variant_selector: None, // TODO: Implement variant selector
            blocks_movement: table.get("blocks_movement")?,
            blocks_sight: table.get("blocks_sight")?,
        })
    }

    /// Convert Lua table to EntityVisual
    fn table_to_entity_visual(&self, table: &LuaTable) -> Result<EntityVisual, PackLoaderError> {
        // Get size as separate values since tuples aren't directly supported
        let size = if let Ok(size_table) = table.get::<_, LuaTable>("size") {
            let width: f32 = size_table.get(1).unwrap_or(1.0);
            let height: f32 = size_table.get(2).unwrap_or(1.0);
            (width, height)
        } else {
            (1.0, 1.0)
        };

        let mut animations = HashMap::new();
        if let Ok(anim_table) = table.get::<_, LuaTable>("animations") {
            for pair in anim_table.pairs::<String, String>() {
                let (key, value) = pair?;
                animations.insert(key, value);
            }
        }

        let mut visual_states = HashMap::new();
        if let Ok(states_table) = table.get::<_, LuaTable>("visual_states") {
            for pair in states_table.pairs::<String, LuaTable>() {
                let (state_name, state_table) = pair?;
                let state_emoji: String = state_table.get("emoji").unwrap_or_else(|_| "❓".to_string());
                let state_color: String = state_table.get("color").unwrap_or_else(|_| "#808080".to_string());
                // Store as simple string for now - emoji representation
                visual_states.insert(state_name, state_emoji);
            }
        }

        Ok(EntityVisual {
            name: table.get("name")?,
            category: table.get("category")?,
            color: table.get("color")?,
            emoji: table.get("emoji").ok(),
            sprite: table.get("sprite").ok(),
            size,
            animations,
            attachment_points: vec![],
            color_variations: table.get("color_variations").unwrap_or(false),
            visual_states,
        })
    }

    /// Add fallback entity visuals for basic entities
    fn add_fallback_entity_visuals(&self, registry: &mut VisualRegistry) {
        registry.entities.insert("peasant".to_string(), EntityVisual {
            name: "Peasant".to_string(),
            category: "unit".to_string(),
            color: "#8B4513".to_string(),
            emoji: Some("👨‍🌾".to_string()),
            sprite: None,
            size: (1.0, 1.0),
            animations: HashMap::new(),
            attachment_points: vec![],
            color_variations: false,
            visual_states: HashMap::new(),
        });

        registry.entities.insert("tree".to_string(), EntityVisual {
            name: "Tree".to_string(),
            category: "resource".to_string(),
            color: "#166534".to_string(),
            emoji: Some("🌳".to_string()),
            sprite: None,
            size: (1.0, 1.0),
            animations: HashMap::new(),
            attachment_points: vec![],
            color_variations: false,
            visual_states: HashMap::new(),
        });
    }

    /// Load a single entity definition from a Lua file
    fn load_entity_from_file(&self, path: &Path, registry: &mut VisualRegistry) -> Result<(), PackLoaderError> {
        let content = fs::read_to_string(path)
            .map_err(|e| PackLoaderError::IoError(e))?;

        // Execute Lua file to get entity definition
        self.lua.load(&content).exec().map_err(|e| PackLoaderError::LuaError(e))?;

        // Get entity definition from global scope
        let global: LuaTable = self.lua.globals().get("_G")
            .map_err(|_| PackLoaderError::MissingMetadata("Global table".to_string()))?;

        // Look for register_entity function calls by examining the global scope
        // We'll try to extract entity data from the last registered entity
        if let Ok(entity_table) = global.get::<_, LuaTable>("_last_entity") {
            let entity_def: EntityDefinition = self.table_to_entity_def(&entity_table)?;

            let visual = entity_def.visual.unwrap_or(VisualConfig {
                color: "#8B4513".to_string(),
                emoji: None,
                sprite: None,
                size: (1.0, 1.0),
            });

            let entity_visual = EntityVisual {
                name: entity_def.name,
                category: entity_def.category.clone(),
                color: visual.color,
                sprite: visual.sprite,
                emoji: visual.emoji.or_else(|| {
                    match entity_def.category.as_str() {
                        "unit" => Some("👤".to_string()),
                        "building" => Some("🏠".to_string()),
                        "resource" => Some("🌳".to_string()),
                        _ => None,
                    }
                }),
                size: visual.size,
                animations: HashMap::new(),
                attachment_points: vec![],
                color_variations: false,
                visual_states: HashMap::new(),
            };

            registry.entities.insert(entity_def.id, entity_visual);
        }

        Ok(())
    }

    /// Convert Lua table to EntityDefinition
    fn table_to_entity_def(&self, table: &LuaTable) -> Result<EntityDefinition, PackLoaderError> {
        let entity_id: String = table.get("id")?;
        let entity_name: String = table.get("name")?;
        let entity_type: String = table.get("type")?;

        // Extract visual information from the visuals table
        let visual_config = if let Ok(visuals_table) = table.get::<_, LuaTable>("visuals") {
            Some(VisualConfig {
                color: visuals_table.get("color").unwrap_or_else(|_| "#8B4513".to_string()),
                emoji: visuals_table.get("emoji").ok(),
                sprite: visuals_table.get("sprite").ok(),
                size: (1.0, 1.0),
            })
        } else {
            None
        };

        Ok(EntityDefinition {
            id: entity_id,
            name: entity_name,
            category: entity_type,
            visual: visual_config,
        })
    }
}

/// Errors that can occur during pack loading
#[derive(Debug, thiserror::Error)]
pub enum PackLoaderError {
    #[error("Pack not found: {0}")]
    PackNotFound(PathBuf),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Lua error: {0}")]
    LuaError(#[from] mlua::Error),

    #[error("Missing metadata: {0}")]
    MissingMetadata(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Load a pack and convert it to IPC format
pub fn load_pack_for_ipc(pack_path: impl AsRef<Path>) -> Result<(IpcPackMetadata, VisualRegistry), PackLoaderError> {
    let loader = PackLoader::new(pack_path)?;
    let metadata = loader.load_metadata()?;
    let visual_registry = loader.load_visual_registry()?;

    let ipc_metadata = IpcPackMetadata {
        id: metadata.id,
        name: metadata.name,
        version: metadata.version,
        author: metadata.author,
        description: metadata.description,
        dependencies: metadata.dependencies,
        features: vec!["entities".to_string(), "resources".to_string(), "items".to_string()],
        priority: 0,
        supports_hot_reload: metadata.config.allow_hot_reload,
    };

    Ok((ipc_metadata, visual_registry))
}