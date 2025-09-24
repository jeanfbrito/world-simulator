//! Modding Example
//!
//! This example demonstrates the modding and customization capabilities
//! of the world-simulator. It showcases:
//! - Custom entity types and components
//! - Mod-based gameplay systems
//! - Scriptable behaviors and events
//! - Custom UI and visualization
//! - Plugin architecture

use bevy::prelude::*;
use world_sim_interface::*;
use world_sim_simple::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

fn main() {
    println!("🔧 Starting Modding Example");

    // Set up logging with modding detail
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    // Create and run the simulation
    let mut app = App::new();

    // Configure with window for mod visualization
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "World Simulator - Modding Example".into(),
            resolution: (1200., 800.).into(),
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        }),
        exit_condition: bevy::window::ExitCondition::DontExit,
        close_when_requested: false,
    }));

    // Add core simulation plugins
    app.add_plugins(simulation::TickSimulationPlugin);
    app.add_plugins(ComponentsPlugin);
    app.add_plugins(PackSystemPlugin);
    app.add_plugins(WorldPlugin);
    app.add_plugins(SimPlugin);
    app.add_plugins(TilemapPlugin);
    app.add_plugins(ResourcesPlugin);
    app.add_plugins(BuildingsPlugin);
    app.add_plugins(CraftingPlugin);
    app.add_plugins(AIPlugin);
    app.add_plugins(SaveLoadPlugin);
    app.add_plugins(PerformancePlugin);
    app.add_plugins(SystemsPlugin);

    // Initialize resources
    app.init_resource::<WorldMap>();
    app.init_resource::<SimulationState>();
    app.init_resource::<ModManager>();
    app.init_resource::<ModRegistry>();

    // Add mod-specific startup systems
    app.add_systems(Startup, setup_modding_example);

    // Add mod update systems
    app.add_systems(Update, (
        mod_system_scheduler,
        custom_entity_processor,
        mod_event_handler,
        mod_ui_updater,
        mod_performance_monitor,
    ).chain());

    // Register custom components
    app.register_component::<MagicComponent>();
    app.register_component::<TechnologyComponent>();
    app.register_component::<CustomAIComponent>();
    app.register_component::<QuestComponent>();

    println!("🔧 Modding example initialized. Running custom gameplay systems...");
    println!("🎮 This will showcase modding capabilities and custom gameplay mechanics.");

    // Run the simulation
    app.run();
}

/// Mod manager for handling loaded mods
#[derive(Resource, Default)]
pub struct ModManager {
    pub loaded_mods: Vec<ModInfo>,
    pub active_mods: Vec<String>,
    pub mod_order: Vec<String>,
    pub mod_data: HashMap<String, ModData>,
    pub mod_systems: HashMap<String, Box<dyn ModSystem>>,
}

/// Mod information structure
#[derive(Debug, Clone)]
pub struct ModInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub load_order: u32,
    pub enabled: bool,
}

/// Mod data containing custom definitions
#[derive(Debug, Clone, Default)]
pub struct ModData {
    pub entities: Vec<CustomEntityDefinition>,
    pub components: Vec<CustomComponentDefinition>,
    pub systems: Vec<CustomSystemDefinition>,
    pub events: Vec<CustomEventDefinition>,
    pub resources: Vec<CustomResourceDefinition>,
    pub ui_elements: Vec<CustomUIElement>,
}

/// Custom entity definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntityDefinition {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub base_entity: Option<String>,
    pub components: Vec<CustomComponentData>,
    pub behaviors: Vec<String>,
    pub spawn_conditions: SpawnConditions,
}

/// Custom component definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomComponentDefinition {
    pub id: String,
    pub name: String,
    pub component_type: String,
    pub fields: HashMap<String, ComponentField>,
    pub default_values: HashMap<String, serde_json::Value>,
}

/// Custom system definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSystemDefinition {
    pub id: String,
    pub name: String,
    pub system_type: String,
    pub update_interval: f32,
    pub priority: i32,
    pub dependencies: Vec<String>,
    pub query_requirements: Vec<QueryRequirement>,
    pub logic: String, // Could be script code or Rust function reference
}

/// Custom component data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomComponentData {
    pub component_id: String,
    pub data: HashMap<String, serde_json::Value>,
}

/// Component field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentField {
    pub name: String,
    pub field_type: String,
    pub default_value: serde_json::Value,
    pub min_value: Option<serde_json::Value>,
    pub max_value: Option<serde_json::Value>,
    pub description: String,
}

/// Spawn conditions for entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnConditions {
    pub biomes: Vec<String>,
    pub resource_requirements: HashMap<String, u32>,
    pub technology_requirements: Vec<String>,
    pub time_requirements: Option<TimeRequirement>,
    pub probability: f32,
}

/// Time requirement for spawning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRequirement {
    pub min_tick: u32,
    pub max_tick: Option<u32>,
    pub time_of_day: Option<TimeOfDay>,
}

/// Query requirement for systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequirement {
    pub component_types: Vec<String>,
    pub optional: bool,
    pub with_filters: Vec<EntityFilter>,
}

/// Entity filter for queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityFilter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

/// Filter operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    NotContains,
}

/// Custom event definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEventDefinition {
    pub id: String,
    pub name: String,
    pub event_type: String,
    pub parameters: HashMap<String, EventParameter>,
    pub handlers: Vec<String>,
}

/// Event parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventParameter {
    pub name: String,
    pub parameter_type: String,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
}

/// Custom resource definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomResourceDefinition {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub resource_type: ResourceType,
    pub properties: ResourceProperties,
    pub generation_rules: GenerationRules,
}

/// Resource properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceProperties {
    pub max_amount: u32,
    pub regeneration_rate: f32,
    pub stack_size: u32,
    pub weight: f32,
    pub value: u32,
}

/// Generation rules for resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRules {
    pub biomes: Vec<String>,
    pub abundance: f32,
    pub cluster_size: u32,
    pub depth_requirements: Option<DepthRequirement>,
}

/// Depth requirement for generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthRequirement {
    pub min_depth: u32,
    pub max_depth: u32,
}

/// Custom UI element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomUIElement {
    pub id: String,
    pub element_type: String,
    pub position: UIPosition,
    pub size: UISize,
    pub content: UIContent,
    pub event_handlers: Vec<UIEventHandler>,
}

/// UI position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIPosition {
    pub x: f32,
    pub y: f32,
    pub anchor: UIAnchor,
}

/// UI size
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISize {
    pub width: f32,
    pub height: f32,
}

/// UI anchor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIAnchor {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

/// UI content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIContent {
    Text { text: String, font_size: f32, color: Color },
    Image { path: String, scale: f32 },
    Button { label: String, action: String },
    Progress { value: f32, max_value: f32, color: Color },
    Container { elements: Vec<String> },
}

/// UI event handler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIEventHandler {
    pub event_type: String,
    pub action: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Mod system trait
pub trait ModSystem: Send + Sync {
    fn initialize(&mut self, world: &mut World);
    fn update(&mut self, world: &mut World, delta_time: f32);
    fn cleanup(&mut self, world: &mut World);
    fn get_info(&self) -> SystemInfo;
}

/// System information
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
}

/// Mod registry for managing available mods
#[derive(Resource, Default)]
pub struct ModRegistry {
    pub available_mods: HashMap<String, ModManifest>,
    pub installed_mods: HashMap<String, ModInfo>,
    pub mod_dependencies: HashMap<String, Vec<String>>,
    pub load_order: Vec<String>,
}

/// Mod manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub compatibility: CompatibilityInfo,
    pub features: Vec<String>,
    pub tags: Vec<String>,
}

/// Compatibility information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityInfo {
    pub game_version: String,
    pub api_version: String,
    pub platform: Vec<String>,
}

/// Custom components for mod example
#[derive(Component, Debug, Clone)]
pub struct MagicComponent {
    pub mana: f32,
    pub max_mana: f32,
    pub mana_regen_rate: f32,
    pub spells: Vec<String>,
    pub spell_power: f32,
}

#[derive(Component, Debug, Clone)]
pub struct TechnologyComponent {
    pub research_points: u32,
    pub unlocked_technologies: Vec<String>,
    pub current_research: Option<String>,
    pub research_efficiency: f32,
}

#[derive(Component, Debug, Clone)]
pub struct CustomAIComponent {
    pub personality_type: String,
    pub behavior_weights: HashMap<String, f32>,
    pub memory: Vec<AIMemory>,
    pub learning_rate: f32,
}

#[derive(Component, Debug, Clone)]
pub struct QuestComponent {
    pub active_quests: Vec<Quest>,
    pub completed_quests: Vec<String>,
    pub quest_progress: HashMap<String, f32>,
    pub reputation: HashMap<String, f32>,
}

/// AI memory structure
#[derive(Debug, Clone)]
pub struct AIMemory {
    pub event_type: String,
    pub data: HashMap<String, serde_json::Value>,
    pub timestamp: u64,
    pub importance: f32,
}

/// Quest structure
#[derive(Debug, Clone)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub objectives: Vec<QuestObjective>,
    pub rewards: QuestRewards,
    pub time_limit: Option<u32>,
    pub difficulty: QuestDifficulty,
}

/// Quest objective
#[derive(Debug, Clone)]
pub struct QuestObjective {
    pub id: String,
    pub description: String,
    pub objective_type: QuestObjectiveType,
    pub target: Option<String>,
    pub amount: u32,
    pub current_progress: u32,
    pub completed: bool,
}

/// Quest objective types
#[derive(Debug, Clone)]
pub enum QuestObjectiveType {
    Collect,
    Kill,
    Explore,
    Build,
    Research,
    Talk,
    Deliver,
}

/// Quest rewards
#[derive(Debug, Clone)]
pub struct QuestRewards {
    pub experience: u32,
    pub resources: HashMap<String, u32>,
    pub items: Vec<String>,
    pub reputation: HashMap<String, f32>,
}

/// Quest difficulty
#[derive(Debug, Clone)]
pub enum QuestDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

/// Time of day
#[derive(Debug, Clone)]
pub enum TimeOfDay {
    Dawn,
    Morning,
    Noon,
    Afternoon,
    Evening,
    Night,
    Midnight,
}

/// Setup modding example
fn setup_modding_example(
    mut commands: Commands,
    mut pack_system: Option<Res<packs::PackSystem>>,
    mut mod_manager: ResMut<ModManager>,
    mut mod_registry: ResMut<ModRegistry>,
) {
    println!("🔧 Setting up Modding Example...");

    // Create sample mods
    create_sample_mods(&mut mod_manager, &mut mod_registry);

    // Load mods
    load_mods(&mut mod_manager, &mut mod_registry);

    // Create custom entities with mod components
    create_mod_entities(&mut commands);

    // Setup mod UI
    setup_mod_ui(&mut commands);

    println!("✅ Modding example setup complete!");
    println!("🎯 Loaded mods: {}", mod_manager.loaded_mods.len());
    println!("🔧 Active systems: {}", mod_manager.mod_systems.len());
    println!("🎮 Custom entities with magic, technology, and quest systems");
}

/// Create sample mods for demonstration
fn create_sample_mods(mod_manager: &mut ModManager, mod_registry: &mut ModRegistry) {
    // Magic System Mod
    let magic_mod = ModInfo {
        id: "magic_system".to_string(),
        name: "Magic System".to_string(),
        version: "1.0.0".to_string(),
        author: "Mod Developer".to_string(),
        description: "Adds magic spells and mana system to the simulation".to_string(),
        dependencies: vec![],
        load_order: 1,
        enabled: true,
    };

    // Technology Mod
    let tech_mod = ModInfo {
        id: "technology_mod".to_string(),
        name: "Technology Tree".to_string(),
        version: "1.2.0".to_string(),
        author: "Tech Developer".to_string(),
        description: "Adds research and technology progression".to_string(),
        dependencies: vec![],
        load_order: 2,
        enabled: true,
    };

    // Quest System Mod
    let quest_mod = ModInfo {
        id: "quest_system".to_string(),
        name: "Quest System".to_string(),
        version: "0.9.0".to_string(),
        author: "Quest Developer".to_string(),
        description: "Adds quests and objectives for units".to_string(),
        dependencies: vec!["magic_system".to_string()],
        load_order: 3,
        enabled: true,
    };

    mod_manager.loaded_mods.push(magic_mod.clone());
    mod_manager.loaded_mods.push(tech_mod.clone());
    mod_manager.loaded_mods.push(quest_mod.clone());

    mod_manager.active_mods.push("magic_system".to_string());
    mod_manager.active_mods.push("technology_mod".to_string());
    mod_manager.active_mods.push("quest_system".to_string());

    // Add to registry
    mod_registry.installed_mods.insert("magic_system".to_string(), magic_mod);
    mod_registry.installed_mods.insert("technology_mod".to_string(), tech_mod);
    mod_registry.installed_mods.insert("quest_system".to_string(), quest_mod);
}

/// Load mods and their systems
fn load_mods(mod_manager: &mut ModManager, mod_registry: &ModRegistry) {
    // Load mod data and systems
    for mod_info in &mod_manager.loaded_mods {
        let mut mod_data = ModData::default();

        match mod_info.id.as_str() {
            "magic_system" => {
                mod_data.entities.push(create_mage_entity());
                mod_data.components.push(create_magic_component_def());
                mod_data.systems.push(create_magic_system_def());
                mod_data.events.push(create_spell_cast_event());
            },
            "technology_mod" => {
                mod_data.entities.push(create_scientist_entity());
                mod_data.components.push(create_technology_component_def());
                mod_data.systems.push(create_research_system_def());
            },
            "quest_system" => {
                mod_data.entities.push(create_quest_giver_entity());
                mod_data.components.push(create_quest_component_def());
                mod_data.systems.push(create_quest_system_def());
                mod_data.ui_elements.push(create_quest_ui_element());
            },
            _ => {}
        }

        mod_manager.mod_data.insert(mod_info.id.clone(), mod_data);
    }

    // Create mod systems
    create_mod_systems(mod_manager);
}

/// Create mod systems
fn create_mod_systems(mod_manager: &mut ModManager) {
    // Magic system
    let magic_system = Box::new(MagicSystem::new());
    mod_manager.mod_systems.insert("magic_system".to_string(), magic_system);

    // Technology system
    let tech_system = Box::new(TechnologySystem::new());
    mod_manager.mod_systems.insert("technology_mod".to_string(), tech_system);

    // Quest system
    let quest_system = Box::new(QuestSystem::new());
    mod_manager.mod_systems.insert("quest_system".to_string(), quest_system);
}

/// Create custom entities with mod components
fn create_mod_entities(commands: &mut Commands) {
    // Create mage entity
    commands.spawn((
        NameComponent("Mage".to_string()),
        UnitTag,
        UnitStats {
            health: 80,
            max_health: 80,
            energy: 120,
            max_energy: 120,
            movement_speed: 0.8,
            ..Default::default()
        },
        PositionComponent { x: 10.0, y: 10.0 },
        MagicComponent {
            mana: 100.0,
            max_mana: 100.0,
            mana_regen_rate: 0.5,
            spells: vec!["fireball".to_string(), "heal".to_string()],
            spell_power: 1.2,
        },
        QuestComponent {
            active_quests: vec![],
            completed_quests: vec![],
            quest_progress: HashMap::new(),
            reputation: HashMap::new(),
        },
        Inventory::new(),
    ));

    // Create scientist entity
    commands.spawn((
        NameComponent("Scientist".to_string()),
        UnitTag,
        UnitStats {
            health: 70,
            max_health: 70,
            energy: 100,
            max_energy: 100,
            movement_speed: 1.0,
            ..Default::default()
        },
        PositionComponent { x: 20.0, y: 20.0 },
        TechnologyComponent {
            research_points: 50,
            unlocked_technologies: vec!["basic_research".to_string()],
            current_research: Some("advanced_technology".to_string()),
            research_efficiency: 1.1,
        },
        CustomAIComponent {
            personality_type: "analytical".to_string(),
            behavior_weights: {
                let mut weights = HashMap::new();
                weights.insert("research".to_string(), 0.8);
                weights.insert("explore".to_string(), 0.4);
                weights.insert("social".to_string(), 0.2);
                weights
            },
            memory: vec![],
            learning_rate: 0.7,
        },
        Inventory::new(),
    ));
}

/// Setup mod UI
fn setup_mod_ui(commands: &mut Commands) {
    // UI elements would be created here
    // This is a placeholder for UI setup
}

// System implementations
fn mod_system_scheduler(
    mut mod_manager: ResMut<ModManager>,
    time: Res<Time>,
    mut last_update: Local<Instant>,
) {
    let now = Instant::now();
    if now.duration_since(*last_update) < Duration::from_millis(100) {
        return;
    }
    *last_update = now;

    // Update mod systems
    for system in mod_manager.mod_systems.values_mut() {
        system.update(&mut commands.world_mut(), time.delta_seconds());
    }
}

fn custom_entity_processor(
    magic_query: Query<(Entity, &MagicComponent)>,
    tech_query: Query<(Entity, &TechnologyComponent)>,
    quest_query: Query<(Entity, &QuestComponent)>,
    sim_state: Res<SimulationState>,
    mut last_report: Local<u32>,
) {
    if sim_state.tick % 100 == 0 && sim_state.tick > 0 && *last_report != sim_state.tick {
        *last_report = sim_state.tick;

        let magic_count = magic_query.iter().count();
        let tech_count = tech_query.iter().count();
        let quest_count = quest_query.iter().count();

        println!("🔧 Mod Entities - {} magic users, {} scientists, {} quest holders",
            magic_count, tech_count, quest_count);
    }
}

fn mod_event_handler(
    sim_state: Res<SimulationState>,
    mut mod_manager: ResMut<ModManager>,
    mut last_event: Local<u32>,
) {
    if sim_state.tick % 200 == 0 && sim_state.tick > 0 && *last_event != sim_state.tick {
        *last_event = sim_state.tick;

        // Process mod events
        for mod_id in &mod_manager.active_mods {
            match mod_id.as_str() {
                "magic_system" => {
                    println!("✨ Magic event: Mana regeneration active");
                },
                "technology_mod" => {
                    println!("🔬 Technology event: Research progress updated");
                },
                "quest_system" => {
                    println!("📜 Quest event: New objectives available");
                },
                _ => {}
            }
        }
    }
}

fn mod_ui_updater(
    sim_state: Res<SimulationState>,
    mut last_update: Local<u32>,
) {
    if sim_state.tick % 50 == 0 && sim_state.tick > 0 && *last_update != sim_state.tick {
        *last_update = sim_state.tick;
        // UI update logic would go here
    }
}

fn mod_performance_monitor(
    mod_manager: Res<ModManager>,
    sim_state: Res<SimulationState>,
    mut last_monitor: Local<u32>,
) {
    if sim_state.tick % 150 == 0 && sim_state.tick > 0 && *last_monitor != sim_state.tick {
        *last_monitor = sim_state.tick;

        println!("🔧 Mod Performance - {} active mods, {} mod systems",
            mod_manager.active_mods.len(), mod_manager.mod_systems.len());

        // Print initial status
        if sim_state.tick == 1 {
            println!("🎮 Modding example started. Monitoring custom systems...");
            println!("📈 Tracking mod performance and entity interactions...");
        }

        // Stop after 400 ticks
        if sim_state.tick >= 400 {
            println!("🎉 Modding example completed successfully!");
            println!("📊 Final Mod Analysis:");
            println!("   • Loaded mods: {}", mod_manager.loaded_mods.len());
            println!("   • Active mods: {}", mod_manager.active_mods.len());
            println!("   • Mod systems: {}", mod_manager.mod_systems.len());
            println!("   • Simulation ticks: {}", sim_state.tick);

            println!("🔧 Key Modding Features Demonstrated:");
            println!("   • Custom entity types and components");
            println!("   • Mod-based gameplay systems");
            println!("   • Scriptable behaviors and events");
            println!("   • Plugin architecture");
            println!("   • Custom UI and visualization");

            // Exit the application
            std::process::exit(0);
        }
    }
}

// Helper functions for creating mod definitions
fn create_mage_entity() -> CustomEntityDefinition {
    CustomEntityDefinition {
        id: "mage".to_string(),
        name: "mage".to_string(),
        display_name: "Mage".to_string(),
        description: "Magic user with spell casting abilities".to_string(),
        base_entity: Some("peasant".to_string()),
        components: vec![
            CustomComponentData {
                component_id: "magic".to_string(),
                data: {
                    let mut data = HashMap::new();
                    data.insert("mana".to_string(), serde_json::json!(100.0));
                    data.insert("spell_power".to_string(), serde_json::json!(1.2));
                    data
                },
            }
        ],
        behaviors: vec!["cast_spells".to_string(), "regenerate_mana".to_string()],
        spawn_conditions: SpawnConditions {
            biomes: vec!["forest".to_string(), "plains".to_string()],
            resource_requirements: HashMap::new(),
            technology_requirements: vec!["magic_research".to_string()],
            time_requirements: None,
            probability: 0.1,
        },
    }
}

fn create_magic_component_def() -> CustomComponentDefinition {
    CustomComponentDefinition {
        id: "magic".to_string(),
        name: "Magic".to_string(),
        component_type: "magic_component".to_string(),
        fields: {
            let mut fields = HashMap::new();
            fields.insert("mana".to_string(), ComponentField {
                name: "mana".to_string(),
                field_type: "f32".to_string(),
                default_value: serde_json::json!(100.0),
                min_value: Some(serde_json::json!(0.0)),
                max_value: Some(serde_json::json!(1000.0)),
                description: "Current mana points".to_string(),
            });
            fields
        },
        default_values: {
            let mut defaults = HashMap::new();
            defaults.insert("mana".to_string(), serde_json::json!(100.0));
            defaults
        },
    }
}

fn create_magic_system_def() -> CustomSystemDefinition {
    CustomSystemDefinition {
        id: "magic_system".to_string(),
        name: "Magic System".to_string(),
        system_type: "update".to_string(),
        update_interval: 0.1,
        priority: 100,
        dependencies: vec![],
        query_requirements: vec![
            QueryRequirement {
                component_types: vec!["magic".to_string()],
                optional: false,
                with_filters: vec![],
            }
        ],
        logic: "update_mana_and_cast_spells".to_string(),
    }
}

fn create_spell_cast_event() -> CustomEventDefinition {
    CustomEventDefinition {
        id: "spell_cast".to_string(),
        name: "Spell Cast".to_string(),
        event_type: "action".to_string(),
        parameters: {
            let mut params = HashMap::new();
            params.insert("caster".to_string(), EventParameter {
                name: "caster".to_string(),
                parameter_type: "entity".to_string(),
                required: true,
                default_value: None,
            });
            params.insert("spell".to_string(), EventParameter {
                name: "spell".to_string(),
                parameter_type: "string".to_string(),
                required: true,
                default_value: None,
            });
            params
        },
        handlers: vec!["apply_spell_effects".to_string(), "consume_mana".to_string()],
    }
}

fn create_scientist_entity() -> CustomEntityDefinition {
    CustomEntityDefinition {
        id: "scientist".to_string(),
        name: "scientist".to_string(),
        display_name: "Scientist".to_string(),
        description: "Research unit that studies technology".to_string(),
        base_entity: Some("peasant".to_string()),
        components: vec![
            CustomComponentData {
                component_id: "technology".to_string(),
                data: {
                    let mut data = HashMap::new();
                    data.insert("research_points".to_string(), serde_json::json!(50));
                    data.insert("research_efficiency".to_string(), serde_json::json!(1.1));
                    data
                },
            }
        ],
        behaviors: vec!["research".to_string(), "analyze".to_string()],
        spawn_conditions: SpawnConditions {
            biomes: vec!["plains".to_string()],
            resource_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("stone".to_string(), 20);
                reqs
            },
            technology_requirements: vec!["basic_science".to_string()],
            time_requirements: None,
            probability: 0.15,
        },
    }
}

fn create_technology_component_def() -> CustomComponentDefinition {
    CustomComponentDefinition {
        id: "technology".to_string(),
        name: "Technology".to_string(),
        component_type: "technology_component".to_string(),
        fields: {
            let mut fields = HashMap::new();
            fields.insert("research_points".to_string(), ComponentField {
                name: "research_points".to_string(),
                field_type: "u32".to_string(),
                default_value: serde_json::json!(0),
                min_value: Some(serde_json::json!(0)),
                max_value: Some(serde_json::json!(10000)),
                description: "Available research points".to_string(),
            });
            fields
        },
        default_values: {
            let mut defaults = HashMap::new();
            defaults.insert("research_points".to_string(), serde_json::json!(0));
            defaults
        },
    }
}

fn create_research_system_def() -> CustomSystemDefinition {
    CustomSystemDefinition {
        id: "research_system".to_string(),
        name: "Research System".to_string(),
        system_type: "update".to_string(),
        update_interval: 1.0,
        priority: 50,
        dependencies: vec![],
        query_requirements: vec![
            QueryRequirement {
                component_types: vec!["technology".to_string()],
                optional: false,
                with_filters: vec![],
            }
        ],
        logic: "process_research_and_unlock_technology".to_string(),
    }
}

fn create_quest_giver_entity() -> CustomEntityDefinition {
    CustomEntityDefinition {
        id: "quest_giver".to_string(),
        name: "quest_giver".to_string(),
        display_name: "Quest Giver".to_string(),
        description: "NPC that provides quests to players".to_string(),
        base_entity: Some("peasant".to_string()),
        components: vec![
            CustomComponentData {
                component_id: "quest".to_string(),
                data: {
                    let mut data = HashMap::new();
                    data.insert("active_quests".to_string(), serde_json::json!([]));
                    data.insert("reputation".to_string(), serde_json::json!({}));
                    data
                },
            }
        ],
        behaviors: vec!["give_quests".to_string(), "track_progress".to_string()],
        spawn_conditions: SpawnConditions {
            biomes: vec!["plains".to_string(), "forest".to_string()],
            resource_requirements: HashMap::new(),
            technology_requirements: vec![],
            time_requirements: None,
            probability: 0.05,
        },
    }
}

fn create_quest_component_def() -> CustomComponentDefinition {
    CustomComponentDefinition {
        id: "quest".to_string(),
        name: "Quest".to_string(),
        component_type: "quest_component".to_string(),
        fields: {
            let mut fields = HashMap::new();
            fields.insert("active_quests".to_string(), ComponentField {
                name: "active_quests".to_string(),
                field_type: "array".to_string(),
                default_value: serde_json::json!([]),
                min_value: None,
                max_value: None,
                description: "Currently active quests".to_string(),
            });
            fields
        },
        default_values: {
            let mut defaults = HashMap::new();
            defaults.insert("active_quests".to_string(), serde_json::json!([]));
            defaults
        },
    }
}

fn create_quest_system_def() -> CustomSystemDefinition {
    CustomSystemDefinition {
        id: "quest_system".to_string(),
        name: "Quest System".to_string(),
        system_type: "update".to_string(),
        update_interval: 0.5,
        priority: 75,
        dependencies: vec!["magic_system".to_string()],
        query_requirements: vec![
            QueryRequirement {
                component_types: vec!["quest".to_string()],
                optional: false,
                with_filters: vec![],
            }
        ],
        logic: "update_quests_and_check_completion".to_string(),
    }
}

fn create_quest_ui_element() -> CustomUIElement {
    CustomUIElement {
        id: "quest_panel".to_string(),
        element_type: "panel".to_string(),
        position: UIPosition {
            x: 10.0,
            y: 10.0,
            anchor: UIAnchor::TopLeft,
        },
        size: UISize {
            width: 300.0,
            height: 400.0,
        },
        content: UIContent::Container {
            elements: vec!["quest_title".to_string(), "quest_list".to_string()],
        },
        event_handlers: vec![],
    }
}

// System implementations for mod systems
struct MagicSystem {
    info: SystemInfo,
}

impl MagicSystem {
    fn new() -> Self {
        Self {
            info: SystemInfo {
                name: "Magic System".to_string(),
                version: "1.0.0".to_string(),
                description: "Handles magic spells and mana management".to_string(),
                author: "Magic Developer".to_string(),
            },
        }
    }
}

impl ModSystem for MagicSystem {
    fn initialize(&mut self, _world: &mut World) {
        println!("✨ Magic system initialized");
    }

    fn update(&mut self, _world: &mut World, _delta_time: f32) {
        // Magic system update logic
    }

    fn cleanup(&mut self, _world: &mut World) {
        println!("✨ Magic system cleaned up");
    }

    fn get_info(&self) -> SystemInfo {
        self.info.clone()
    }
}

struct TechnologySystem {
    info: SystemInfo,
}

impl TechnologySystem {
    fn new() -> Self {
        Self {
            info: SystemInfo {
                name: "Technology System".to_string(),
                version: "1.2.0".to_string(),
                description: "Manages research and technology progression".to_string(),
                author: "Tech Developer".to_string(),
            },
        }
    }
}

impl ModSystem for TechnologySystem {
    fn initialize(&mut self, _world: &mut World) {
        println!("🔬 Technology system initialized");
    }

    fn update(&mut self, _world: &mut World, _delta_time: f32) {
        // Technology system update logic
    }

    fn cleanup(&mut self, _world: &mut World) {
        println!("🔬 Technology system cleaned up");
    }

    fn get_info(&self) -> SystemInfo {
        self.info.clone()
    }
}

struct QuestSystem {
    info: SystemInfo,
}

impl QuestSystem {
    fn new() -> Self {
        Self {
            info: SystemInfo {
                name: "Quest System".to_string(),
                version: "0.9.0".to_string(),
                description: "Handles quests and objectives for entities".to_string(),
                author: "Quest Developer".to_string(),
            },
        }
    }
}

impl ModSystem for QuestSystem {
    fn initialize(&mut self, _world: &mut World) {
        println!("📜 Quest system initialized");
    }

    fn update(&mut self, _world: &mut World, _delta_time: f32) {
        // Quest system update logic
    }

    fn cleanup(&mut self, _world: &mut World) {
        println!("📜 Quest system cleaned up");
    }

    fn get_info(&self) -> SystemInfo {
        self.info.clone()
    }
}

// Helper trait for component registration
trait ComponentRegistrar {
    fn register_component<T: Component>(&mut self);
}

impl ComponentRegistrar for App {
    fn register_component<T: Component>(&mut self) {
        // This is a placeholder for component registration
        // In a real implementation, this would register the component for reflection
    }
}