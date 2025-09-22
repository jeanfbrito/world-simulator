//! IPC Output Module for World Simulator
//!
//! This module handles output of simulation state via JSON over stdout/stdin
//! for communication with external viewers or processes.

use bevy::prelude::*;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::{self, Write, BufWriter};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use world_sim_interface::ipc::{
    IpcMessage, MessagePayload, TileVisual, EntityVisual,
    VisualRegistry, GameStateData, GlobalStateData,
};
use world_sim_interface::{EntitySnapshot, Position, EntityId};

/// IPC output configuration
#[derive(Resource)]
pub struct IpcOutputConfig {
    /// Enable/disable IPC output
    pub enabled: bool,
    /// Output to stdout if true, stderr if false
    pub use_stdout: bool,
    /// Buffer size for batching messages
    pub buffer_size: usize,
    /// Flush interval in milliseconds
    pub flush_interval_ms: u64,
    /// Enable message compression
    pub compression_enabled: bool,
    /// Maximum message size before splitting
    pub max_message_size: usize,
}

impl Default for IpcOutputConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            use_stdout: true,
            buffer_size: 100,
            flush_interval_ms: 50,
            compression_enabled: false,
            max_message_size: 1024 * 1024, // 1MB
        }
    }
}

/// IPC output buffer for message batching
#[derive(Resource)]
pub struct IpcOutputBuffer {
    /// Buffered messages waiting to be sent
    messages: Vec<IpcMessage>,
    /// Sequence number counter
    sequence_counter: u64,
    /// Last flush timestamp
    last_flush_ms: u64,
    /// Output writer
    writer: Arc<Mutex<BufWriter<Box<dyn Write + Send>>>>,
}

impl FromWorld for IpcOutputBuffer {
    fn from_world(world: &mut World) -> Self {
        let config = world.resource::<IpcOutputConfig>();
        Self::new(config)
    }
}

impl IpcOutputBuffer {
    pub fn new(config: &IpcOutputConfig) -> Self {
        let writer: Box<dyn Write + Send> = if config.use_stdout {
            Box::new(io::stdout())
        } else {
            Box::new(io::stderr())
        };

        Self {
            messages: Vec::with_capacity(config.buffer_size),
            sequence_counter: 0,
            last_flush_ms: 0,
            writer: Arc::new(Mutex::new(BufWriter::new(writer))),
        }
    }

    /// Add a message to the buffer
    pub fn add_message(&mut self, payload: MessagePayload) -> Result<()> {
        let message = IpcMessage {
            version: 1,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            seq_num: self.sequence_counter,
            payload,
        };

        self.sequence_counter += 1;
        self.messages.push(message);

        Ok(())
    }

    /// Flush buffered messages to output
    pub fn flush(&mut self) -> Result<()> {
        if self.messages.is_empty() {
            return Ok(());
        }

        let writer = self.writer.clone();
        let messages = std::mem::take(&mut self.messages);

        // Update last flush timestamp BEFORE sending
        self.last_flush_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        // Send in a separate thread to avoid blocking
        std::thread::spawn(move || {
            let mut writer = writer.lock().unwrap();

            for message in messages {
                match serde_json::to_string(&message) {
                    Ok(json_str) => {
                        writeln!(writer, "{}", json_str).unwrap();
                        writer.flush().unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error serializing IPC message: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    /// Check if buffer should be flushed
    pub fn should_flush(&self, config: &IpcOutputConfig) -> bool {
        if self.messages.is_empty() {
            return false;
        }

        // Flush if buffer is full
        if self.messages.len() >= config.buffer_size {
            return true;
        }

        // Flush if interval has passed
        let current_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        current_ms - self.last_flush_ms >= config.flush_interval_ms
    }

    /// Get current buffer size
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

/// IPC output plugin
pub struct IpcOutputPlugin;

impl Plugin for IpcOutputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IpcOutputConfig>()
            .init_resource::<IpcOutputBuffer>()
            .add_event::<IpcOutputEvent>()
            .add_systems(Startup, setup_ipc_output)
            .add_systems(
                Update,
                (
                    handle_ipc_output_events,
                    flush_ipc_buffer,
                    broadcast_game_state_ipc.after(crate::simulation::run_simulation_ticks),
                ).chain(),
            );
    }
}

/// Events for IPC output
#[derive(Event)]
pub enum IpcOutputEvent {
    /// Send a custom message
    SendMessage(MessagePayload),
    /// Send game state snapshot
    SendGameState,
    /// Send visual registry update
    SendVisualRegistry,
    /// Send pack metadata
    SendPackMetadata,
}

/// IPC output errors
#[derive(Debug, thiserror::Error)]
pub enum IpcOutputError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Buffer overflow: {0}")]
    BufferOverflow(String),
}

type Result<T> = std::result::Result<T, IpcOutputError>;

/// Initialize IPC output system
fn setup_ipc_output(
    config: Res<IpcOutputConfig>,
    mut buffer: ResMut<IpcOutputBuffer>,
) {
    if !config.enabled {
        info!("IPC output disabled");
        return;
    }

    info!("IPC output enabled (buffer size: {}, flush interval: {}ms)",
          config.buffer_size, config.flush_interval_ms);

    // Send initial heartbeat message
    let heartbeat_payload = MessagePayload::Heartbeat(world_sim_interface::ipc::HeartbeatData {
        sender: "simulator".to_string(),
        sent_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        metrics: None,
    });

    if let Err(e) = buffer.add_message(heartbeat_payload) {
        error!("Failed to send IPC heartbeat message: {:?}", e);
    }

    if let Err(e) = buffer.flush() {
        error!("Failed to flush IPC buffer: {:?}", e);
    }
}

/// Handle IPC output events
fn handle_ipc_output_events(
    mut events: EventReader<IpcOutputEvent>,
    mut buffer: ResMut<IpcOutputBuffer>,
    config: Res<IpcOutputConfig>,
) {
    if !config.enabled {
        return;
    }

    for event in events.read() {
        match event {
            IpcOutputEvent::SendMessage(payload) => {
                if let Err(e) = buffer.add_message(payload.clone()) {
                    error!("Failed to add IPC message: {:?}", e);
                }
            }
            IpcOutputEvent::SendGameState => {
                // This will be handled by the broadcast system
            }
            IpcOutputEvent::SendVisualRegistry => {
                // Send visual registry
                let registry = create_visual_registry();
                let payload = MessagePayload::PackDefinitions(world_sim_interface::ipc::PackDefinitionsData {
                    packs: vec![],
                    visual_registry: registry,
                    load_order: vec!["visual-registry".to_string()],
                });
                if let Err(e) = buffer.add_message(payload) {
                    error!("Failed to send visual registry: {:?}", e);
                }
            }
            IpcOutputEvent::SendPackMetadata => {
                // TODO: Implement pack metadata sending
                println!("🔧 IPC Debug: Pack metadata sending not yet implemented");
            }
        }
    }
}

/// Flush IPC buffer based on configuration
fn flush_ipc_buffer(
    mut buffer: ResMut<IpcOutputBuffer>,
    config: Res<IpcOutputConfig>,
    _time: Res<Time>,
) {
    if !config.enabled {
        return;
    }

    if buffer.should_flush(&config) {
        println!("💧 IPC Debug: Flushing {} messages", buffer.len());
        if let Err(e) = buffer.flush() {
            error!("Failed to flush IPC buffer: {:?}", e);
        } else {
            println!("✅ IPC Debug: Successfully flushed {} messages", buffer.len());
        }
    } else {
        // Debug: Show why we're not flushing
        println!("⏳ IPC Debug: Not flushing - buffer has {} messages, threshold: {}, interval: {}ms",
                 buffer.len(), config.buffer_size, config.flush_interval_ms);
    }
}

/// Broadcast game state via IPC
fn broadcast_game_state_ipc(
    mut buffer: ResMut<IpcOutputBuffer>,
    config: Res<IpcOutputConfig>,
    sim_state: Res<crate::SimulationState>,
    world_map: Res<crate::WorldMap>,
    entities_query: Query<(
        Entity,
        &crate::components::NameComponent,
        &crate::components::HealthComponent,
        &crate::TileEntity,
        Option<&crate::components::UnitNeedsV2>,
        Option<&crate::components::UnitInventory>,
        Option<&crate::components::WorkProgress>,
        Option<&crate::components::UnitMind>,
    ), Without<crate::components::ResourceNode>>,
    resource_query: Query<(
        &crate::TileEntity,
        &crate::components::ResourceNode,
        &crate::components::NameComponent,
    ), With<crate::components::ResourceNode>>,
    pack_system: Option<Res<crate::packs::PackSystem>>,
) {
    if !config.enabled {
        return;
    }

    // Debug: Log simulation state every tick
    println!("🔍 IPC Debug: tick={}, just_ticked={}, running={}, changed={}",
        sim_state.tick, sim_state.just_ticked, sim_state.running, sim_state.is_changed());

    // Only broadcast on simulation ticks or when state changes
    if !sim_state.just_ticked && !sim_state.is_changed() {
        println!("🚫 IPC Debug: Skipping broadcast - no tick and no change");
        return;
    }

    println!("📡 IPC Debug: Broadcasting game state at tick {}", sim_state.tick);

    // Create entity snapshots
    let mut entity_snapshots = Vec::new();

    // Process regular entities (units, buildings, etc.)
    let entity_count = entities_query.iter().count();
    let resource_count = resource_query.iter().count();

    println!("🔍 IPC Debug: Found {} entities and {} resources", entity_count, resource_count);

    for (entity, name, health, tile, needs, inventory, work, mind) in entities_query.iter() {
        let mut components = HashMap::new();
        components.insert("health".to_string(), serde_json::Value::Number(serde_json::Number::from(health.current as i64)));
        components.insert("max_health".to_string(), serde_json::Value::Number(serde_json::Number::from(health.maximum as i64)));
        components.insert("display_name".to_string(), serde_json::Value::String(name.display_name.clone()));

        if let Some(needs) = needs {
            components.insert("hunger".to_string(), serde_json::Value::Number(serde_json::Number::from(needs.hunger() as i64)));
            components.insert("energy".to_string(), serde_json::Value::Number(serde_json::Number::from(needs.energy() as i64)));
            components.insert("morale".to_string(), serde_json::Value::Number(serde_json::Number::from(needs.morale() as i64)));
        }

            // Map name to EntityType
        let entity_type = match name.name.as_str() {
            "peasant" => world_sim_interface::entities::EntityType::Worker,
            _ => world_sim_interface::entities::EntityType::Worker, // Default fallback
        };

        let snapshot = EntitySnapshot {
            id: entity.index() as u64,
            entity_type,
            position: Position {
                x: tile.x as i32,
                y: tile.y as i32,
            },
            components,
        };
        entity_snapshots.push(snapshot);
    }

    // Process resource entities
    for (tile, resource, name) in resource_query.iter() {
        let resource_id = format!("resource_{}_{}", tile.x, tile.y);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    resource_id.hash(&mut hasher);

    let mut components = HashMap::new();
    components.insert("amount".to_string(), serde_json::Value::Number(serde_json::Number::from(resource.amount as i64)));
    components.insert("max_amount".to_string(), serde_json::Value::Number(serde_json::Number::from(resource.max_amount as i64)));
    components.insert("resource_type".to_string(), serde_json::Value::String(format!("{:?}", resource.resource_type)));

        // Map resource name to EntityType
        let entity_type = match name.name.as_str() {
            "tree" => world_sim_interface::entities::EntityType::Tree,
            "berry" => world_sim_interface::entities::EntityType::BerryBush,
            _ => world_sim_interface::entities::EntityType::Tree, // Default fallback
        };

        let snapshot = EntitySnapshot {
            id: hasher.finish(),
            entity_type,
            position: Position {
                x: tile.x as i32,
                y: tile.y as i32,
            },
            components,
        };
        entity_snapshots.push(snapshot);
    }

    // Create game state snapshot
    let game_state = GameStateData {
        tick: sim_state.tick as u64,
        world_size: (crate::MAP_SIZE, crate::MAP_SIZE),
        entities: entity_snapshots,
        global_state: GlobalStateData {
            simulation_speed: sim_state.speed,
            is_running: sim_state.running,
            elapsed_time: sim_state.tick as f32 * 0.1, // Assuming 0.1s per tick
            resources: HashMap::new(), // TODO: Populate from actual resources
            population: world_sim_interface::ipc::PopulationData {
                total: 0, // TODO: Calculate actual population
                by_type: HashMap::new(),
                by_region: HashMap::new(),
            },
        },
        changed_entities: None, // TODO: Implement delta updates
    };

    let payload = MessagePayload::GameState(game_state);

    println!("📡 IPC Debug: Adding game state message to buffer");
    if let Err(e) = buffer.add_message(payload) {
        error!("Failed to add game state to IPC buffer: {:?}", e);
    } else {
        println!("✅ IPC Debug: Successfully added message to buffer (total: {})", buffer.len());
    }
}

/// Create visual registry from current game state
fn create_visual_registry() -> VisualRegistry {
    let mut registry = VisualRegistry {
        tiles: HashMap::new(),
        entities: HashMap::new(),
        ui_themes: HashMap::new(),
        animations: HashMap::new(),
        sprite_sheets: HashMap::new(),
    };

    // Tile visuals
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

    registry.tiles.insert("tree".to_string(), TileVisual {
        name: "Tree".to_string(),
        color: "#166534".to_string(),
        emoji: Some("🌳".to_string()),
        sprite: None,
        animation: None,
        variant_selector: None,
        blocks_movement: true,
        blocks_sight: false,
    });

    // Entity visuals
    registry.entities.insert("peasant".to_string(), EntityVisual {
        name: "Peasant".to_string(),
        category: "unit".to_string(),
        color: "#8B4513".to_string(),
        emoji: Some("👨‍🌾".to_string()),
        size: (1.0, 1.0),
        sprite: None,
        animations: HashMap::new(),
        attachment_points: vec![],
        color_variations: false,
        visual_states: HashMap::new(),
    });

    registry.entities.insert("berry".to_string(), EntityVisual {
        name: "Berry Bush".to_string(),
        category: "resource".to_string(),
        color: "#dc2626".to_string(),
        emoji: Some("🫐".to_string()),
        size: (1.0, 1.0),
        sprite: None,
        animations: HashMap::new(),
        attachment_points: vec![],
        color_variations: false,
        visual_states: HashMap::new(),
    });

    registry
}

