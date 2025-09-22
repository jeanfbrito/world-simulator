// Registry System
// Generic registry trait and implementations for different data types

use super::{PackError, ResourceDefinition, ItemDefinition, RecipeDefinition, EntityDefinition};
use std::collections::HashMap;

/// Generic registry trait for storing and retrieving definitions
pub trait Registry<T> {
    fn register(&mut self, id: String, data: T) -> Result<(), PackError>;
    fn get(&self, id: &str) -> Option<&T>;
    fn get_all(&self) -> Vec<&T>;
    fn exists(&self, id: &str) -> bool;
    fn count(&self) -> usize;
    fn validate(&self) -> Result<(), PackError>;
}

/// Resource registry
pub struct ResourceRegistry {
    resources: HashMap<String, ResourceDefinition>,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }
}

impl Registry<ResourceDefinition> for ResourceRegistry {
    fn register(&mut self, id: String, data: ResourceDefinition) -> Result<(), PackError> {
        if self.resources.contains_key(&id) {
            return Err(PackError::DuplicateId(id));
        }
        self.resources.insert(id, data);
        Ok(())
    }

    fn get(&self, id: &str) -> Option<&ResourceDefinition> {
        self.resources.get(id)
    }

    fn get_all(&self) -> Vec<&ResourceDefinition> {
        self.resources.values().collect()
    }

    fn exists(&self, id: &str) -> bool {
        self.resources.contains_key(id)
    }

    fn count(&self) -> usize {
        self.resources.len()
    }

    fn validate(&self) -> Result<(), PackError> {
        // Validate each resource
        for (id, resource) in &self.resources {
            // Check that ID matches
            if resource.id != *id {
                return Err(PackError::ValidationError(
                    format!("Resource ID mismatch: key '{}' vs definition '{}'", id, resource.id)
                ));
            }

            // Validate required fields
            if resource.name.is_empty() {
                return Err(PackError::ValidationError(
                    format!("Resource '{}' has empty name", id)
                ));
            }

            // Validate physical properties
            if resource.properties.weight < 0.0 {
                return Err(PackError::ValidationError(
                    format!("Resource '{}' has negative weight", id)
                ));
            }

            if resource.properties.stack_size < 0 {
                return Err(PackError::ValidationError(
                    format!("Resource '{}' has negative stack size", id)
                ));
            }
        }
        Ok(())
    }
}

/// Item registry
pub struct ItemRegistry {
    items: HashMap<String, ItemDefinition>,
}

impl ItemRegistry {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

impl Registry<ItemDefinition> for ItemRegistry {
    fn register(&mut self, id: String, data: ItemDefinition) -> Result<(), PackError> {
        if self.items.contains_key(&id) {
            return Err(PackError::DuplicateId(id));
        }
        self.items.insert(id, data);
        Ok(())
    }

    fn get(&self, id: &str) -> Option<&ItemDefinition> {
        self.items.get(id)
    }

    fn get_all(&self) -> Vec<&ItemDefinition> {
        self.items.values().collect()
    }

    fn exists(&self, id: &str) -> bool {
        self.items.contains_key(id)
    }

    fn count(&self) -> usize {
        self.items.len()
    }

    fn validate(&self) -> Result<(), PackError> {
        for (id, item) in &self.items {
            // Check ID matches
            if item.id != *id {
                return Err(PackError::ValidationError(
                    format!("Item ID mismatch: key '{}' vs definition '{}'", id, item.id)
                ));
            }

            // Validate required fields
            if item.name.is_empty() {
                return Err(PackError::ValidationError(
                    format!("Item '{}' has empty name", id)
                ));
            }

            // Validate properties
            if item.properties.weight < 0.0 {
                return Err(PackError::ValidationError(
                    format!("Item '{}' has negative weight", id)
                ));
            }

            if item.properties.stack_size < 1 {
                return Err(PackError::ValidationError(
                    format!("Item '{}' has invalid stack size", id)
                ));
            }

            // Validate tool properties if present
            if let Some(tool) = &item.tool {
                if tool.durability < 0.0 || tool.max_durability < 0.0 {
                    return Err(PackError::ValidationError(
                        format!("Item '{}' has invalid durability values", id)
                    ));
                }

                if tool.efficiency <= 0.0 {
                    return Err(PackError::ValidationError(
                        format!("Item '{}' has non-positive efficiency", id)
                    ));
                }
            }

            // Validate consumable properties if present
            if let Some(consumable) = &item.consumable {
                if let Some(cooldown) = consumable.cooldown {
                    if cooldown < 0.0 {
                        return Err(PackError::ValidationError(
                            format!("Item '{}' has negative cooldown", id)
                        ));
                    }
                }

                if let Some(perish_time) = consumable.perish_time {
                    if perish_time <= 0.0 {
                        return Err(PackError::ValidationError(
                            format!("Item '{}' has invalid perish time", id)
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}

/// Recipe registry
pub struct RecipeRegistry {
    recipes: HashMap<String, RecipeDefinition>,
}

impl RecipeRegistry {
    pub fn new() -> Self {
        Self {
            recipes: HashMap::new(),
        }
    }
}

impl Registry<RecipeDefinition> for RecipeRegistry {
    fn register(&mut self, id: String, data: RecipeDefinition) -> Result<(), PackError> {
        if self.recipes.contains_key(&id) {
            return Err(PackError::DuplicateId(id));
        }
        self.recipes.insert(id, data);
        Ok(())
    }

    fn get(&self, id: &str) -> Option<&RecipeDefinition> {
        self.recipes.get(id)
    }

    fn get_all(&self) -> Vec<&RecipeDefinition> {
        self.recipes.values().collect()
    }

    fn exists(&self, id: &str) -> bool {
        self.recipes.contains_key(id)
    }

    fn count(&self) -> usize {
        self.recipes.len()
    }

    fn validate(&self) -> Result<(), PackError> {
        for (id, recipe) in &self.recipes {
            // Check ID matches
            if recipe.id != *id {
                return Err(PackError::ValidationError(
                    format!("Recipe ID mismatch: key '{}' vs definition '{}'", id, recipe.id)
                ));
            }

            // Must have at least one requirement
            if recipe.requirements.is_empty() {
                return Err(PackError::ValidationError(
                    format!("Recipe '{}' has no requirements", id)
                ));
            }

            // Must have at least one output
            if recipe.outputs.is_empty() {
                return Err(PackError::ValidationError(
                    format!("Recipe '{}' has no outputs", id)
                ));
            }

            // Validate requirement counts
            for req in &recipe.requirements {
                if req.count < 1 {
                    return Err(PackError::ValidationError(
                        format!("Recipe '{}' has invalid requirement count for '{}'", id, req.item)
                    ));
                }
            }

            // Validate output counts and chances
            for output in &recipe.outputs {
                if output.count < 1 {
                    return Err(PackError::ValidationError(
                        format!("Recipe '{}' has invalid output count for '{}'", id, output.item)
                    ));
                }

                if let Some(chance) = output.chance {
                    if !(0.0..=1.0).contains(&chance) {
                        return Err(PackError::ValidationError(
                            format!("Recipe '{}' has invalid chance for output '{}'", id, output.item)
                        ));
                    }
                }
            }

            // Validate crafting time
            if recipe.crafting.time <= 0.0 {
                return Err(PackError::ValidationError(
                    format!("Recipe '{}' has invalid crafting time", id)
                ));
            }
        }
        Ok(())
    }
}

/// Entity registry
pub struct EntityRegistry {
    entities: HashMap<String, EntityDefinition>,
}

impl EntityRegistry {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }
}

impl Registry<EntityDefinition> for EntityRegistry {
    fn register(&mut self, id: String, data: EntityDefinition) -> Result<(), PackError> {
        if self.entities.contains_key(&id) {
            return Err(PackError::DuplicateId(id));
        }
        self.entities.insert(id, data);
        Ok(())
    }

    fn get(&self, id: &str) -> Option<&EntityDefinition> {
        self.entities.get(id)
    }

    fn get_all(&self) -> Vec<&EntityDefinition> {
        self.entities.values().collect()
    }

    fn exists(&self, id: &str) -> bool {
        self.entities.contains_key(id)
    }

    fn count(&self) -> usize {
        self.entities.len()
    }

    fn validate(&self) -> Result<(), PackError> {
        for (id, entity) in &self.entities {
            // Check ID matches
            if entity.id != *id {
                return Err(PackError::ValidationError(
                    format!("Entity ID mismatch: key '{}' vs definition '{}'", id, entity.id)
                ));
            }

            // Validate required fields
            if entity.name.is_empty() {
                return Err(PackError::ValidationError(
                    format!("Entity '{}' has empty name", id)
                ));
            }

            // Validate health
            if entity.properties.health <= 0.0 || entity.properties.max_health <= 0.0 {
                return Err(PackError::ValidationError(
                    format!("Entity '{}' has invalid health values", id)
                ));
            }

            if entity.properties.health > entity.properties.max_health {
                return Err(PackError::ValidationError(
                    format!("Entity '{}' has health greater than max health", id)
                ));
            }

            // Validate unit properties if present
            if let Some(unit) = &entity.unit {
                if unit.movement_speed <= 0.0 {
                    return Err(PackError::ValidationError(
                        format!("Entity '{}' has invalid movement speed", id)
                    ));
                }

                if unit.energy <= 0.0 || unit.max_energy <= 0.0 {
                    return Err(PackError::ValidationError(
                        format!("Entity '{}' has invalid energy values", id)
                    ));
                }

                if unit.inventory.slots < 1 {
                    return Err(PackError::ValidationError(
                        format!("Entity '{}' has invalid inventory slots", id)
                    ));
                }
            }
        }
        Ok(())
    }
}