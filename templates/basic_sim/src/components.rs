//! Custom components for the basic simulation

use world_sim::prelude::*;

/// Position component for entity location
#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn direction_to(&self, other: &Position) -> (f32, f32) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let distance = self.distance_to(other);

        if distance > 0.0 {
            (dx / distance, dy / distance)
        } else {
            (0.0, 0.0)
        }
    }
}

/// Velocity component for entity movement
#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > 0.0 {
            self.x /= mag;
            self.y /= mag;
        }
    }
}

/// Health component for entity health
#[derive(Component, Debug, Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub regeneration_rate: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
            regeneration_rate: 0.1,
        }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    pub fn health_percentage(&self) -> f32 {
        self.current / self.max
    }
}

/// Energy component for entity energy/stamina
#[derive(Component, Debug, Clone)]
pub struct Energy {
    pub current: f32,
    pub max: f32,
    pub regeneration_rate: f32,
}

impl Energy {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
            regeneration_rate: 0.2,
        }
    }

    pub fn consume(&mut self, amount: f32) -> bool {
        if self.current >= amount {
            self.current -= amount;
            true
        } else {
            false
        }
    }

    pub fn restore(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn percentage(&self) -> f32 {
        self.current / self.max
    }
}

/// Inventory component for carrying items
#[derive(Component, Debug, Clone)]
pub struct Inventory {
    pub items: std::collections::HashMap<String, u32>,
    pub capacity: usize,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: std::collections::HashMap::new(),
            capacity,
        }
    }

    pub fn add_item(&mut self, item_type: &str, amount: u32) -> bool {
        if self.total_items() + amount as usize <= self.capacity {
            *self.items.entry(item_type.to_string()).or_insert(0) += amount;
            true
        } else {
            false
        }
    }

    pub fn remove_item(&mut self, item_type: &str, amount: u32) -> bool {
        if let Some(count) = self.items.get_mut(item_type) {
            if *count >= amount {
                *count -= amount;
                if *count == 0 {
                    self.items.remove(item_type);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_item_count(&self, item_type: &str) -> u32 {
        *self.items.get(item_type).unwrap_or(&0)
    }

    pub fn has_item(&self, item_type: &str, amount: u32) -> bool {
        self.get_item_count(item_type) >= amount
    }

    pub fn total_items(&self) -> usize {
        self.items.values().map(|&count| count as usize).sum()
    }

    pub fn is_full(&self) -> bool {
        self.total_items() >= self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

/// Skills component for entity abilities
#[derive(Component, Debug, Clone)]
pub struct Skills {
    pub gathering: f32,
    pub crafting: f32,
    pub building: f32,
    pub combat: f32,
    pub experience_points: u32,
}

impl Skills {
    pub fn new() -> Self {
        Self {
            gathering: 0.5,
            crafting: 0.5,
            building: 0.5,
            combat: 0.5,
            experience_points: 0,
        }
    }

    pub fn add_experience(&mut self, amount: u32) {
        self.experience_points += amount;
        self.update_skills();
    }

    pub fn update_skills(&mut self) {
        // Simple skill progression based on experience
        let total_exp = self.experience_points as f32;
        self.gathering = (total_exp / 100.0).min(10.0);
        self.crafting = (total_exp / 150.0).min(10.0);
        self.building = (total_exp / 200.0).min(10.0);
        self.combat = (total_exp / 250.0).min(10.0);
    }

    pub fn get_skill_level(&self, skill_name: &str) -> f32 {
        match skill_name {
            "gathering" => self.gathering,
            "crafting" => self.crafting,
            "building" => self.building,
            "combat" => self.combat,
            _ => 0.0,
        }
    }
}

/// Equipment component for equipped items
#[derive(Component, Debug, Clone)]
pub struct Equipment {
    pub weapon: Option<String>,
    pub armor: Option<String>,
    pub tool: Option<String>,
    pub accessories: Vec<String>,
}

impl Equipment {
    pub fn new() -> Self {
        Self {
            weapon: None,
            armor: None,
            tool: None,
            accessories: Vec::new(),
        }
    }

    pub fn equip_weapon(&mut self, weapon: String) -> Option<String> {
        std::mem::replace(&mut self.weapon, Some(weapon))
    }

    pub fn equip_armor(&mut self, armor: String) -> Option<String> {
        std::mem::replace(&mut self.armor, Some(armor))
    }

    pub fn equip_tool(&mut self, tool: String) -> Option<String> {
        std::mem::replace(&mut self.tool, Some(tool))
    }

    pub fn add_accessory(&mut self, accessory: String) -> bool {
        if self.accessories.len() < 4 {
            self.accessories.push(accessory);
            true
        } else {
            false
        }
    }

    pub fn unequip_weapon(&mut self) -> Option<String> {
        self.weapon.take()
    }

    pub fn unequip_armor(&mut self) -> Option<String> {
        self.armor.take()
    }

    pub fn unequip_tool(&mut self) -> Option<String> {
        self.tool.take()
    }

    pub fn remove_accessory(&mut self, index: usize) -> Option<String> {
        if index < self.accessories.len() {
            Some(self.accessories.remove(index))
        } else {
            None
        }
    }
}

/// State machine component for AI behavior
#[derive(Component, Debug, Clone)]
pub struct StateMachine {
    pub current_state: String,
    pub previous_state: Option<String>,
    pub state_timer: f32,
    pub state_data: std::collections::HashMap<String, String>,
}

impl StateMachine {
    pub fn new(initial_state: &str) -> Self {
        Self {
            current_state: initial_state.to_string(),
            previous_state: None,
            state_timer: 0.0,
            state_data: std::collections::HashMap::new(),
        }
    }

    pub fn change_state(&mut self, new_state: &str) {
        self.previous_state = Some(self.current_state.clone());
        self.current_state = new_state.to_string();
        self.state_timer = 0.0;
    }

    pub fn update(&mut self, dt: f32) {
        self.state_timer += dt;
    }

    pub fn get_state_data(&self, key: &str) -> Option<&String> {
        self.state_data.get(key)
    }

    pub fn set_state_data(&mut self, key: &str, value: String) {
        self.state_data.insert(key.to_string(), value);
    }

    pub fn is_in_state(&self, state: &str) -> bool {
        self.current_state == state
    }

    pub fn was_in_state(&self, state: &str) -> bool {
        self.previous_state.as_ref().map_or(false, |s| s == state)
    }
}

/// Memory component for AI
#[derive(Component, Debug, Clone)]
pub struct Memory {
    pub short_term: Vec<MemoryEntry>,
    pub long_term: Vec<MemoryEntry>,
    pub working_memory: std::collections::HashMap<String, String>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            short_term: Vec::new(),
            long_term: Vec::new(),
            working_memory: std::collections::HashMap::new(),
        }
    }

    pub fn add_short_term_memory(&mut self, entry: MemoryEntry) {
        self.short_term.push(entry);
        // Keep only recent memories
        if self.short_term.len() > 50 {
            self.short_term.remove(0);
        }
    }

    pub fn add_long_term_memory(&mut self, entry: MemoryEntry) {
        self.long_term.push(entry);
        // Consolidate memories periodically
        if self.long_term.len() > 1000 {
            self.consolidate_memories();
        }
    }

    pub fn remember_resource_location(&mut self, resource_type: &str, position: Position) {
        let entry = MemoryEntry {
            memory_type: "resource_location".to_string(),
            data: format!("{}:{},{}", resource_type, position.x, position.y),
            importance: 0.7,
            timestamp: std::time::SystemTime::now(),
        };
        self.add_short_term_memory(entry);
    }

    pub fn recall_resource_locations(&self, resource_type: &str) -> Vec<Position> {
        let mut positions = Vec::new();

        for memory in &self.short_term {
            if memory.memory_type == "resource_location" && memory.data.starts_with(resource_type) {
                if let Some(pos_str) = memory.data.split(':').nth(1) {
                    if let Some((x_str, y_str)) = pos_str.split_once(',') {
                        if let (Ok(x), Ok(y)) = (x_str.parse::<f32>(), y_str.parse::<f32>()) {
                            positions.push(Position::new(x, y));
                        }
                    }
                }
            }
        }

        positions
    }

    pub fn set_working_memory(&mut self, key: &str, value: String) {
        self.working_memory.insert(key.to_string(), value);
    }

    pub fn get_working_memory(&self, key: &str) -> Option<&String> {
        self.working_memory.get(key)
    }

    pub fn consolidate_memories(&mut self) {
        // Remove old or unimportant memories
        self.long_term.retain(|memory| {
            memory.importance > 0.3 || memory.is_recent()
        });
    }
}

/// Individual memory entry
#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub memory_type: String,
    pub data: String,
    pub importance: f32,
    pub timestamp: std::time::SystemTime,
}

impl MemoryEntry {
    pub fn is_recent(&self) -> bool {
        self.timestamp.elapsed().unwrap_or_default().as_secs() < 3600 // 1 hour
    }

    pub fn age(&self) -> std::time::Duration {
        self.timestamp.elapsed().unwrap_or_default()
    }
}

/// Relationships component for social interactions
#[derive(Component, Debug, Clone)]
pub struct Relationships {
    pub relationships: std::collections::HashMap<u64, Relationship>,
    pub reputation: f32,
    pub factions: std::collections::HashMap<String, f32>,
}

impl Relationships {
    pub fn new() -> Self {
        Self {
            relationships: std::collections::HashMap::new(),
            reputation: 0.0,
            factions: std::collections::HashMap::new(),
        }
    }

    pub fn set_relationship(&mut self, entity_id: u64, relationship: Relationship) {
        self.relationships.insert(entity_id, relationship);
    }

    pub fn get_relationship(&self, entity_id: u64) -> Option<&Relationship> {
        self.relationships.get(&entity_id)
    }

    pub fn update_relationship(&mut self, entity_id: u64, change: f32) {
        if let Some(relationship) = self.relationships.get_mut(&entity_id) {
            relationship.standing = (relationship.standing + change).clamp(-1.0, 1.0);
        } else {
            self.set_relationship(entity_id, Relationship::new(change));
        }
    }

    pub fn add_faction_reputation(&mut self, faction: &str, reputation: f32) {
        *self.factions.entry(faction.to_string()).or_insert(0.0) += reputation;
    }

    pub fn get_faction_reputation(&self, faction: &str) -> f32 {
        *self.factions.get(faction).unwrap_or(&0.0)
    }
}

/// Individual relationship data
#[derive(Debug, Clone)]
pub struct Relationship {
    pub standing: f32, // -1.0 (enemy) to 1.0 (friend)
    pub trust: f32,
    pub familiarity: f32,
    pub last_interaction: std::time::SystemTime,
}

impl Relationship {
    pub fn new(standing: f32) -> Self {
        Self {
            standing: standing.clamp(-1.0, 1.0),
            trust: 0.5,
            familiarity: 0.0,
            last_interaction: std::time::SystemTime::now(),
        }
    }

    pub fn is_friendly(&self) -> bool {
        self.standing > 0.3
    }

    pub fn is_hostile(&self) -> bool {
        self.standing < -0.3
    }

    pub fn is_trusted(&self) -> bool {
        self.trust > 0.7
    }
}

/// Needs component for AI motivation
#[derive(Component, Debug, Clone)]
pub struct Needs {
    pub hunger: f32,
    pub thirst: f32,
    pub energy: f32,
    pub social: f32,
    pub safety: f32,
}

impl Needs {
    pub fn new() -> Self {
        Self {
            hunger: 0.5,
            thirst: 0.5,
            energy: 1.0,
            social: 0.5,
            safety: 0.8,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Needs decay over time
        self.hunger -= dt * 0.01;
        self.thirst -= dt * 0.02;
        self.energy -= dt * 0.005;
        self.social -= dt * 0.003;
        self.safety -= dt * 0.001;

        // Clamp values
        self.hunger = self.hunger.clamp(0.0, 1.0);
        self.thirst = self.thirst.clamp(0.0, 1.0);
        self.energy = self.energy.clamp(0.0, 1.0);
        self.social = self.social.clamp(0.0, 1.0);
        self.safety = self.safety.clamp(0.0, 1.0);
    }

    pub fn satisfy_hunger(&mut self, amount: f32) {
        self.hunger = (self.hunger + amount).min(1.0);
    }

    pub fn satisfy_thirst(&mut self, amount: f32) {
        self.thirst = (self.thirst + amount).min(1.0);
    }

    pub fn restore_energy(&mut self, amount: f32) {
        self.energy = (self.energy + amount).min(1.0);
    }

    pub fn get_most_urgent_need(&self) -> Option<&str> {
        let needs = [
            ("hunger", self.hunger),
            ("thirst", self.thirst),
            ("energy", self.energy),
            ("social", self.social),
            ("safety", self.safety),
        ];

        needs.iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .and_then(|(name, value)| if *value < 0.3 { Some(*name) } else { None })
    }

    pub fn are_all_needs_met(&self) -> bool {
        self.hunger > 0.7 && self.thirst > 0.7 && self.energy > 0.7
    }
}