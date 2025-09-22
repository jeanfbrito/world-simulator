// Pack System Module
// Manages loading and registration of data-driven content packs

use bevy::prelude::*;
use mlua::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

pub mod registry;
// pub mod loader_simple;  // Disabled - using loader_v2 instead
pub mod loader_v2;
pub mod definitions;

pub use registry::*;
// pub use loader_simple::*;  // Using loader_v2 instead
pub use loader_v2::*;
pub use definitions::*;

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

// Pack metadata loaded from pack.lua
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

// Main pack system that manages all registries
#[derive(Resource)]
pub struct PackSystem {
    pub metadata: PackMetadata,
    pub resource_registry: ResourceRegistry,
    pub item_registry: ItemRegistry,
    pub recipe_registry: RecipeRegistry,
    pub entity_registry: EntityRegistry,
    pub pack_path: PathBuf,
    lua: Lua,
    pending_registrations: Option<PendingRegistrations>,
}

impl PackSystem {
    /// Load a pack from the given path
    pub fn load_pack(pack_path: impl AsRef<Path>) -> Result<Self, PackError> {
        let pack_path = pack_path.as_ref();
        println!("[PACK] Loading pack from: {:?}", pack_path);
        info!("[PACK] Loading pack from: {:?}", pack_path);

        // Load metadata
        let metadata = Self::load_metadata(pack_path)?;
        println!("[PACK] Loaded metadata for pack: {}", metadata.name);
        info!("[PACK] Loaded metadata for pack: {}", metadata.name);

        // Create Lua context
        let lua = Lua::new();

        // Create pack system
        let mut pack_system = Self {
            metadata: metadata.clone(),
            resource_registry: ResourceRegistry::new(),
            item_registry: ItemRegistry::new(),
            recipe_registry: RecipeRegistry::new(),
            entity_registry: EntityRegistry::new(),
            pack_path: pack_path.to_path_buf(),
            lua,
            pending_registrations: None,
        };

        // Bind Lua API
        pack_system.bind_lua_api()?;

        // Load data files in order
        for category in &metadata.load_order.clone() {
            info!("[PACK] Loading category: {}", category);
            pack_system.load_category(category)?;
        }


        // Process all pending registrations
        pack_system.process_pending_registrations()?;

        // Validate all registries
        pack_system.validate_all()?;

        println!("[PACK] Successfully loaded pack: {}", pack_system.metadata.name);
        println!("[PACK] Loaded {} resources", pack_system.resource_registry.count());
        println!("[PACK] Loaded {} items", pack_system.item_registry.count());
        println!("[PACK] Loaded {} recipes", pack_system.recipe_registry.count());
        println!("[PACK] Loaded {} entities", pack_system.entity_registry.count());
        info!("[PACK] Successfully loaded pack: {}", pack_system.metadata.name);
        info!("[PACK] Loaded {} resources", pack_system.resource_registry.count());
        info!("[PACK] Loaded {} items", pack_system.item_registry.count());
        info!("[PACK] Loaded {} recipes", pack_system.recipe_registry.count());
        info!("[PACK] Loaded {} entities", pack_system.entity_registry.count());

        Ok(pack_system)
    }

    /// Load pack metadata from pack.lua
    fn load_metadata(pack_path: &Path) -> Result<PackMetadata, PackError> {
        let metadata_path = pack_path.join("pack.lua");
        if !metadata_path.exists() {
            return Err(PackError::MetadataNotFound(metadata_path));
        }

        let lua = Lua::new();
        let content = fs::read_to_string(&metadata_path)
            .map_err(|e| PackError::IoError(e))?;

        let table: LuaTable = lua.load(&content)
            .set_name(&format!("{:?}", metadata_path))
            .eval()
            .map_err(|e| PackError::LuaError {
                file: metadata_path.clone(),
                error: e,
            })?;

        // Extract metadata from Lua table
        Ok(PackMetadata {
            id: table.get("id").map_err(|e| PackError::InvalidMetadata(
                format!("Missing 'id' field: {}", e)
            ))?,
            name: table.get("name").map_err(|e| PackError::InvalidMetadata(
                format!("Missing 'name' field: {}", e)
            ))?,
            version: table.get("version").map_err(|e| PackError::InvalidMetadata(
                format!("Missing 'version' field: {}", e)
            ))?,
            author: table.get("author").map_err(|e| PackError::InvalidMetadata(
                format!("Missing 'author' field: {}", e)
            ))?,
            description: table.get("description").map_err(|e| PackError::InvalidMetadata(
                format!("Missing 'description' field: {}", e)
            ))?,
            dependencies: table.get("dependencies").unwrap_or_else(|_| Vec::new()),
            load_order: table.get("load_order").map_err(|e| PackError::InvalidMetadata(
                format!("Missing 'load_order' field: {}", e)
            ))?,
            config: {
                let config_table: LuaTable = table.get("config").unwrap_or_else(|_| {
                    let t = lua.create_table().unwrap();
                    t.set("debug", false).unwrap();
                    t.set("validate_strict", true).unwrap();
                    t.set("allow_hot_reload", true).unwrap();
                    t
                });
                PackConfig {
                    debug: config_table.get("debug").unwrap_or(false),
                    validate_strict: config_table.get("validate_strict").unwrap_or(true),
                    allow_hot_reload: config_table.get("allow_hot_reload").unwrap_or(true),
                }
            },
        })
    }

    /// Validate all registries for consistency
    fn validate_all(&self) -> Result<(), PackError> {
        // Validate individual registries
        self.resource_registry.validate()?;
        self.item_registry.validate()?;
        self.recipe_registry.validate()?;
        self.entity_registry.validate()?;

        // Cross-validate references
        self.validate_cross_references()?;

        Ok(())
    }

    /// Validate cross-references between registries
    fn validate_cross_references(&self) -> Result<(), PackError> {
        // Check that items reference valid resources
        for item in self.item_registry.get_all() {
            // Validate tool materials reference valid resources
            if let Some(tool) = &item.tool {
                if !tool.material.is_empty() && !self.resource_registry.exists(&tool.material) {
                    return Err(PackError::ValidationError(
                        format!("Item '{}' references unknown material resource '{}'",
                            item.id, tool.material)
                    ));
                }
            }
        }

        // Check that recipes reference valid items
        for recipe in self.recipe_registry.get_all() {
            for req in &recipe.requirements {
                if !self.item_registry.exists(&req.item) {
                    return Err(PackError::ValidationError(
                        format!("Recipe '{}' references unknown item '{}'",
                            recipe.id, req.item)
                    ));
                }
            }
            for output in &recipe.outputs {
                if !self.item_registry.exists(&output.item) {
                    return Err(PackError::ValidationError(
                        format!("Recipe '{}' outputs unknown item '{}'",
                            recipe.id, output.item)
                    ));
                }
            }
        }

        Ok(())
    }
}

// Error types
#[derive(Debug)]
pub enum PackError {
    MetadataNotFound(PathBuf),
    InvalidMetadata(String),
    LuaError { file: PathBuf, error: LuaError },
    IoError(std::io::Error),
    ValidationError(String),
    DuplicateId(String),
    CategoryNotFound(String),
}

impl From<std::io::Error> for PackError {
    fn from(error: std::io::Error) -> Self {
        PackError::IoError(error)
    }
}

impl From<mlua::Error> for PackError {
    fn from(error: mlua::Error) -> Self {
        PackError::LuaError {
            file: std::path::PathBuf::from("unknown"),
            error,
        }
    }
}

impl std::fmt::Display for PackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MetadataNotFound(path) => write!(f, "Pack metadata not found at: {:?}", path),
            Self::InvalidMetadata(msg) => write!(f, "Invalid pack metadata: {}", msg),
            Self::LuaError { file, error } => write!(f, "Lua error in {:?}: {}", file, error),
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::DuplicateId(id) => write!(f, "Duplicate ID: {}", id),
            Self::CategoryNotFound(cat) => write!(f, "Category not found: {}", cat),
        }
    }
}

impl std::error::Error for PackError {}

// Pack system plugin
pub struct PackSystemPlugin;

impl Plugin for PackSystemPlugin {
    fn build(&self, app: &mut App) {
        println!("[PACK] PackSystemPlugin::build called");
        // Load the default pack on startup
        let pack_path = "assets/packs/dev-world";
        println!("[PACK] Attempting to load pack from: {}", pack_path);

        match PackSystem::load_pack(pack_path) {
            Ok(pack_system) => {
                println!("[PACK] Successfully initialized pack system");
                info!("[PACK] Successfully initialized pack system");
                app.insert_resource(pack_system);
            }
            Err(e) => {
                println!("[PACK] Failed to load pack: {}", e);
                error!("[PACK] Failed to load pack: {}", e);
                // For now, continue without pack system
                // In production, this might panic or load a fallback
            }
        }
    }
}