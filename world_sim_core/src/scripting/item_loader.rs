//! Item definition loader from Lua scripts

use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_lua::prelude::*;
use std::collections::HashMap;
use world_sim_interface::ResourceType;

/// Component to mark entities with item scripts
#[derive(Component)]
pub struct ItemScript {
    pub script_path: String,
    pub loaded: bool,
}

/// Lua-defined item/resource
#[derive(Clone, Debug, Reflect)]
pub struct LuaItem {
    pub id: String,
    pub name: String,
    pub category: String,
    pub stack_size: u32,
    pub weight: f32,
    pub base_value: u32,
    pub decay_rate: f32,
    pub description: String,
    pub properties: HashMap<String, f32>,
}

/// Resource registry for Lua-defined items
#[derive(Resource, Default)]
pub struct ItemRegistry {
    items: HashMap<String, LuaItem>,
    categories: HashMap<String, Vec<String>>,
}

impl ItemRegistry {
    pub fn register(&mut self, item: LuaItem) {
        // Add to category index
        self.categories
            .entry(item.category.clone())
            .or_insert_with(Vec::new)
            .push(item.id.clone());
        
        // Store item
        self.items.insert(item.id.clone(), item);
    }
    
    pub fn get(&self, id: &str) -> Option<&LuaItem> {
        self.items.get(id)
    }
    
    pub fn get_by_category(&self, category: &str) -> Vec<&LuaItem> {
        self.categories
            .get(category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.items.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    pub fn all(&self) -> Vec<&LuaItem> {
        self.items.values().collect()
    }
}

/// Command event to trigger item script reloading
#[derive(Event)]
pub struct ReloadItemScriptsCommand;

/// System to load item scripts on command
pub fn load_item_scripts(
    mut commands: Commands,
    mut reload_events: EventReader<ReloadItemScriptsCommand>,
    asset_server: Res<AssetServer>,
    existing_scripts: Query<Entity, With<ItemScript>>,
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
    
    // Load item definition scripts
    let item_scripts = vec![
        "scripts/items/resources.lua",
        "scripts/items/tools.lua",
        "scripts/items/consumables.lua",
        "scripts/items/rare_items.lua",
    ];
    
    for script_path in item_scripts {
        commands.spawn((
            ItemScript {
                script_path: script_path.to_string(),
                loaded: false,
            },
            ScriptCollection::<LuaScript>::default(),
        ));
        
        tracing::info!("Loading item script: {}", script_path);
    }
}

/// System to process loaded item scripts
pub fn process_item_scripts(
    mut scripts: Query<(Entity, &mut ItemScript, &ScriptCollection<LuaScript>)>,
    mut item_registry: ResMut<ItemRegistry>,
) {
    for (entity, mut script, collection) in scripts.iter_mut() {
        if !script.loaded && !collection.scripts.is_empty() {
            script.loaded = true;
            tracing::debug!("Item script loaded: {}", script.script_path);
        }
    }
}

/// Convert Lua item definitions to game items
pub fn apply_lua_items(
    item_registry: Res<ItemRegistry>,
    mut commands: Commands,
) {
    // This system would convert Lua items to actual game entities
    // when needed, such as when spawning items in the world
}

/// Quality tier for items
#[derive(Clone, Debug, Reflect)]
pub enum ItemQuality {
    Poor,
    Normal,
    Good,
    Excellent,
    Masterwork,
}

impl ItemQuality {
    pub fn multiplier(&self) -> f32 {
        match self {
            ItemQuality::Poor => 0.7,
            ItemQuality::Normal => 1.0,
            ItemQuality::Good => 1.3,
            ItemQuality::Excellent => 1.6,
            ItemQuality::Masterwork => 2.0,
        }
    }
}

/// Component for items with quality
#[derive(Component, Clone, Debug, Reflect)]
pub struct ItemQualityComponent {
    pub quality: ItemQuality,
    pub durability: f32,
    pub max_durability: f32,
}

impl ItemQualityComponent {
    pub fn new(quality: ItemQuality) -> Self {
        let durability = 100.0 * quality.multiplier();
        Self {
            quality,
            durability,
            max_durability: durability,
        }
    }
    
    pub fn damage(&mut self, amount: f32) {
        self.durability = (self.durability - amount).max(0.0);
    }
    
    pub fn repair(&mut self, amount: f32) {
        self.durability = (self.durability + amount).min(self.max_durability);
    }
    
    pub fn is_broken(&self) -> bool {
        self.durability <= 0.0
    }
    
    pub fn condition_percentage(&self) -> f32 {
        self.durability / self.max_durability
    }
}

/// Component for stackable items
#[derive(Component, Clone, Debug, Reflect)]
pub struct StackableComponent {
    pub current_stack: u32,
    pub max_stack: u32,
}

impl StackableComponent {
    pub fn new(max_stack: u32) -> Self {
        Self {
            current_stack: 1,
            max_stack,
        }
    }
    
    pub fn add(&mut self, amount: u32) -> u32 {
        let space = self.max_stack - self.current_stack;
        let to_add = amount.min(space);
        self.current_stack += to_add;
        amount - to_add // Return overflow
    }
    
    pub fn remove(&mut self, amount: u32) -> u32 {
        let to_remove = amount.min(self.current_stack);
        self.current_stack -= to_remove;
        to_remove
    }
    
    pub fn is_full(&self) -> bool {
        self.current_stack >= self.max_stack
    }
    
    pub fn is_empty(&self) -> bool {
        self.current_stack == 0
    }
}

/// Component for items that decay over time
#[derive(Component, Clone, Debug, Reflect)]
pub struct DecayComponent {
    pub decay_rate: f32,
    pub current_freshness: f32,
    pub spoiled: bool,
}

impl DecayComponent {
    pub fn new(decay_rate: f32) -> Self {
        Self {
            decay_rate,
            current_freshness: 100.0,
            spoiled: false,
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        if !self.spoiled {
            self.current_freshness -= self.decay_rate * delta_time;
            if self.current_freshness <= 0.0 {
                self.current_freshness = 0.0;
                self.spoiled = true;
            }
        }
    }
    
    pub fn preserve(&mut self, amount: f32) {
        self.current_freshness = (self.current_freshness + amount).min(100.0);
        if self.current_freshness > 0.0 {
            self.spoiled = false;
        }
    }
}

/// System to update item decay
pub fn update_item_decay(
    mut items: Query<(&mut DecayComponent, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut decay, entity) in items.iter_mut() {
        decay.update(time.delta_seconds());
        
        if decay.spoiled {
            // Mark item as spoiled or remove it
            commands.entity(entity).insert(SpoiledMarker);
        }
    }
}

/// Marker component for spoiled items
#[derive(Component)]
pub struct SpoiledMarker;

/// Component for tool items
#[derive(Component, Clone, Debug, Reflect)]
pub struct ToolComponent {
    pub tool_type: String,
    pub efficiency: f32,
    pub harvest_bonus: HashMap<String, f32>,
}

impl ToolComponent {
    pub fn get_bonus_for(&self, resource: &str) -> f32 {
        self.harvest_bonus.get(resource).copied().unwrap_or(1.0)
    }
}

/// Component for consumable items
#[derive(Component, Clone, Debug, Reflect)]
pub struct ConsumableComponent {
    pub nutrition: f32,
    pub morale_bonus: f32,
    pub effects: Vec<ConsumableEffect>,
}

#[derive(Clone, Debug, Reflect)]
pub struct ConsumableEffect {
    pub effect_type: String,
    pub value: f32,
    pub duration: f32,
}

/// System to handle item consumption
pub fn consume_item_system(
    mut commands: Commands,
    consumables: Query<(&ConsumableComponent, Entity)>,
    mut consumers: Query<&mut crate::components::WorkerComponent>,
    mut events: EventReader<ConsumeItemEvent>,
) {
    for event in events.read() {
        if let Ok((consumable, item_entity)) = consumables.get(event.item) {
            if let Ok(mut worker) = consumers.get_mut(event.consumer) {
                // Apply nutrition
                worker.hunger = (worker.hunger - consumable.nutrition).max(0.0);
                
                // Apply morale bonus
                worker.morale += consumable.morale_bonus;
                
                // Apply special effects
                for effect in &consumable.effects {
                    // Apply effect to worker
                    tracing::debug!("Applied effect {} to worker", effect.effect_type);
                }
                
                // Remove consumed item
                commands.entity(item_entity).despawn();
            }
        }
    }
}

/// Event for consuming an item
#[derive(Event)]
pub struct ConsumeItemEvent {
    pub consumer: Entity,
    pub item: Entity,
}