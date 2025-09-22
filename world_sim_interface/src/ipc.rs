//! Inter-Process Communication (IPC) protocol definitions
//!
//! This module defines the message format and types used for communication
//! between the headless World Simulator and the Sim Viewer via stdout/stdin.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::{EntitySnapshot, Position, Tick, EntityId};

/// Current IPC protocol version
pub const IPC_PROTOCOL_VERSION: u32 = 1;

/// Core IPC message structure with versioning and sequencing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcMessage {
    /// Protocol version for compatibility
    pub version: u32,

    /// Unix timestamp when message was created
    pub timestamp: u64,

    /// Sequence number for message ordering and loss detection
    pub seq_num: u64,

    /// The actual message payload
    pub payload: MessagePayload,
}

impl IpcMessage {
    /// Create a new IPC message with automatic timestamp and sequence
    pub fn new(seq_num: u64, payload: MessagePayload) -> Self {
        Self {
            version: IPC_PROTOCOL_VERSION,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            seq_num,
            payload,
        }
    }

    /// Validate that this message meets protocol requirements
    pub fn validate(&self) -> Result<(), IpcError> {
        if self.version != IPC_PROTOCOL_VERSION {
            return Err(IpcError::VersionMismatch {
                expected: IPC_PROTOCOL_VERSION,
                received: self.version,
            });
        }

        // Validate timestamp is not too far in the future (5 minute tolerance)
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        if self.timestamp > current_time + 300 {
            return Err(IpcError::InvalidTimestamp {
                timestamp: self.timestamp,
                current_time,
            });
        }

        Ok(())
    }
}

/// Different types of messages that can be sent via IPC
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum MessagePayload {
    /// Complete game state snapshot
    GameState(GameStateData),

    /// Individual entity updates for efficiency
    EntityUpdate(EntityUpdateData),

    /// Pack definitions and visual metadata
    PackDefinitions(PackDefinitionsData),

    /// Tick update for synchronization
    TickUpdate(TickUpdateData),

    /// Command from viewer to simulator
    Command(CommandData),

    /// Heartbeat for connection health
    Heartbeat(HeartbeatData),

    /// Error information
    Error(ErrorData),

    /// System status information
    SystemStatus(SystemStatusData),
}

/// Complete game state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateData {
    /// Current simulation tick
    pub tick: Tick,

    /// World dimensions
    pub world_size: (usize, usize),

    /// All entities in the world
    pub entities: Vec<EntitySnapshot>,

    /// Global state information
    pub global_state: GlobalStateData,

    /// Optional: Only changed entities since last update
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed_entities: Option<Vec<EntityId>>,
}

/// Global simulation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalStateData {
    /// Simulation speed multiplier
    pub simulation_speed: f32,

    /// Whether simulation is running
    pub is_running: bool,

    /// Total elapsed simulation time
    pub elapsed_time: f32,

    /// Resource totals
    pub resources: HashMap<String, u32>,

    /// Population statistics
    pub population: PopulationData,
}

/// Population statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationData {
    /// Total population count
    pub total: u32,

    /// Population by entity type
    pub by_type: HashMap<String, u32>,

    /// Population by position (region-based)
    pub by_region: HashMap<String, u32>,
}

/// Individual entity update for efficient delta updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityUpdateData {
    /// Entity ID
    pub entity_id: EntityId,

    /// Update type
    pub update_type: EntityUpdateType,

    /// Entity data (for changed/added entities)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity: Option<EntitySnapshot>,

    /// Position change (if applicable)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
}

/// Types of entity updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityUpdateType {
    /// Entity was added to the world
    Added,

    /// Entity was modified
    Changed,

    /// Entity was removed from the world
    Removed,

    /// Entity moved to a new position
    Moved,
}

/// Pack definitions and visual metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackDefinitionsData {
    /// List of loaded packs
    pub packs: Vec<PackMetadata>,

    /// Visual definitions registry
    pub visual_registry: VisualRegistry,

    /// Load order information
    pub load_order: Vec<String>,
}

/// Metadata about a loaded pack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackMetadata {
    /// Pack identifier
    pub id: String,

    /// Human-readable pack name
    pub name: String,

    /// Pack version
    pub version: String,

    /// Pack author
    pub author: String,

    /// Pack description
    pub description: String,

    /// Dependencies on other packs
    pub dependencies: Vec<String>,

    /// Pack features/capabilities
    pub features: Vec<String>,

    /// Load priority
    pub priority: i32,

    /// Whether this pack supports hot-reload
    pub supports_hot_reload: bool,
}

/// Visual registry containing all visual definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualRegistry {
    /// Tile visual definitions
    pub tiles: HashMap<String, TileVisual>,

    /// Entity visual definitions
    pub entities: HashMap<String, EntityVisual>,

    /// UI theme definitions
    pub ui_themes: HashMap<String, UITheme>,

    /// Animation definitions
    pub animations: HashMap<String, AnimationData>,

    /// Sprite sheet mappings
    pub sprite_sheets: HashMap<String, SpriteSheet>,
}

/// Visual definition for a tile type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileVisual {
    /// Display name
    pub name: String,

    /// Base color (hex format)
    pub color: String,

    /// Sprite identifier (if any)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sprite: Option<String>,

    /// Fallback emoji for text-based rendering
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,

    /// Animation data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animation: Option<String>,

    /// Variant selector for visual diversity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant_selector: Option<VariantSelector>,

    /// Whether this tile blocks movement
    pub blocks_movement: bool,

    /// Whether this tile blocks sight
    pub blocks_sight: bool,
}

/// Visual definition for an entity type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityVisual {
    /// Display name
    pub name: String,

    /// Entity category
    pub category: String,

    /// Base color (hex format)
    pub color: String,

    /// Sprite identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sprite: Option<String>,

    /// Fallback emoji for text-based rendering
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,

    /// Size in tiles
    pub size: (f32, f32),

    /// Animation set mapping
    pub animations: HashMap<String, String>,

    /// Whether to use color variations
    #[serde(default)]
    pub color_variations: bool,

    /// Equipment/attachment points
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachment_points: Vec<String>,

    /// Visual states (e.g., health, status effects)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub visual_states: HashMap<String, String>,
}

/// UI theme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UITheme {
    /// Theme name
    pub name: String,

    /// Color scheme
    pub colors: HashMap<String, String>,

    /// Font family
    pub font_family: String,

    /// UI element styles
    pub elements: HashMap<String, UIElementStyle>,

    /// Icon mappings
    pub icons: HashMap<String, String>,
}

/// Individual UI element style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIElementStyle {
    /// Background color
    pub background: String,

    /// Border color
    pub border: String,

    /// Text color
    pub text_color: String,

    /// Font size
    pub font_size: u32,

    /// Border radius
    pub border_radius: u32,

    /// Sprite or image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sprite: Option<String>,
}

/// Animation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationData {
    /// Animation name
    pub name: String,

    /// Frame count
    pub frame_count: u32,

    /// Frame duration in milliseconds
    pub frame_duration: u32,

    /// Whether animation loops
    pub loops: bool,

    /// Frame sprite names
    pub frames: Vec<String>,
}

/// Variant selector for visual diversity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantSelector {
    /// Base sprite name
    pub base: String,

    /// Variant suffixes
    pub variants: Vec<String>,

    /// Selection strategy
    pub strategy: VariantStrategy,
}

/// Strategy for selecting variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariantStrategy {
    /// Random selection
    Random,

    /// Based on position (hash-based)
    Positional,

    /// Based on entity ID
    Deterministic,

    /// Round-robin selection
    RoundRobin,
}

/// Sprite sheet definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteSheet {
    /// Sprite sheet name
    pub name: String,

    /// Image file path
    pub file_path: String,

    /// Tile size in pixels
    pub tile_size: (u32, u32),

    /// Sprite layout (columns, rows)
    pub layout: (u32, u32),

    /// Individual sprite mappings
    pub sprites: HashMap<String, (u32, u32)>,
}

/// Tick update for synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickUpdateData {
    /// Current tick number
    pub tick: Tick,

    /// Simulation time elapsed
    pub elapsed_time: f32,

    /// Performance metrics
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performance: Option<PerformanceData>,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceData {
    /// Tick duration in milliseconds
    pub tick_duration_ms: f32,

    /// Entities processed this tick
    pub entities_processed: u32,

    /// Memory usage in MB
    pub memory_usage_mb: f32,

    /// System load percentage
    pub system_load_percent: f32,
}

/// Command from viewer to simulator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandData {
    /// Command identifier
    pub command_id: String,

    /// Command type
    pub command_type: String,

    /// Command parameters
    pub parameters: HashMap<String, serde_json::Value>,

    /// Whether command requires acknowledgment
    pub requires_ack: bool,
}

/// Heartbeat for connection health monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatData {
    /// Sender identifier
    pub sender: String,

    /// Timestamp when heartbeat was sent
    pub sent_at: u64,

    /// Optional performance metrics
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<PerformanceData>,
}

/// Error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorData {
    /// Error code
    pub code: String,

    /// Human-readable error message
    pub message: String,

    /// Error severity
    pub severity: ErrorSeverity,

    /// Additional error context
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub context: HashMap<String, String>,
}

/// Error severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Informational message
    Info,

    /// Warning that doesn't stop operation
    Warning,

    /// Error that may affect functionality
    Error,

    /// Critical error that stops operation
    Critical,
}

/// System status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusData {
    /// System status
    pub status: SystemStatus,

    /// System capabilities
    pub capabilities: Vec<String>,

    /// System version
    pub version: String,

    /// Uptime in seconds
    pub uptime_seconds: u64,

    /// Active connections count
    pub active_connections: u32,
}

/// System status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemStatus {
    /// System is running normally
    Running,

    /// System is paused
    Paused,

    /// System is shutting down
    ShuttingDown,

    /// System is in error state
    Error,

    /// System is initializing
    Initializing,
}

/// IPC error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcError {
    /// Protocol version mismatch
    VersionMismatch { expected: u32, received: u32 },

    /// Invalid timestamp
    InvalidTimestamp { timestamp: u64, current_time: u64 },

    /// Invalid message format
    InvalidFormat { message: String },

    /// Sequence number mismatch or gap
    SequenceError { expected: u64, received: u64 },

    /// Connection lost
    ConnectionLost,

    /// Serialization/deserialization error
    SerializationError { message: String },

    /// Command execution error
    CommandError { command: String, error: String },
}

impl std::fmt::Display for IpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpcError::VersionMismatch { expected, received } => {
                write!(f, "Version mismatch: expected {}, received {}", expected, received)
            }
            IpcError::InvalidTimestamp { timestamp, current_time } => {
                write!(f, "Invalid timestamp: {}, current time: {}", timestamp, current_time)
            }
            IpcError::InvalidFormat { message } => {
                write!(f, "Invalid message format: {}", message)
            }
            IpcError::SequenceError { expected, received } => {
                write!(f, "Sequence error: expected {}, received {}", expected, received)
            }
            IpcError::ConnectionLost => {
                write!(f, "Connection lost")
            }
            IpcError::SerializationError { message } => {
                write!(f, "Serialization error: {}", message)
            }
            IpcError::CommandError { command, error } => {
                write!(f, "Command error: {}: {}", command, error)
            }
        }
    }
}

impl std::error::Error for IpcError {}

/// Helper trait for creating typed messages
pub trait MessageFactory {
    fn create_game_state(&mut self, tick: Tick, world_size: (usize, usize), entities: Vec<EntitySnapshot>) -> IpcMessage;
    fn create_entity_update(&mut self, entity_id: EntityId, update_type: EntityUpdateType, entity: Option<EntitySnapshot>) -> IpcMessage;
    fn create_pack_definitions(&mut self, packs: Vec<PackMetadata>, visual_registry: VisualRegistry) -> IpcMessage;
    fn create_tick_update(&mut self, tick: Tick, elapsed_time: f32) -> IpcMessage;
    fn create_command(&mut self, command_id: String, command_type: String, parameters: HashMap<String, serde_json::Value>) -> IpcMessage;
    fn create_heartbeat(&mut self, sender: String) -> IpcMessage;
    fn create_error(&mut self, code: String, message: String, severity: ErrorSeverity) -> IpcMessage;
}

/// Default message factory implementation
pub struct DefaultMessageFactory {
    sequence_counter: u64,
}

impl DefaultMessageFactory {
    pub fn new() -> Self {
        Self { sequence_counter: 0 }
    }

    fn next_sequence(&mut self) -> u64 {
        self.sequence_counter += 1;
        self.sequence_counter
    }
}

impl MessageFactory for DefaultMessageFactory {
    fn create_game_state(&mut self, tick: Tick, world_size: (usize, usize), entities: Vec<EntitySnapshot>) -> IpcMessage {
        IpcMessage::new(
            self.next_sequence(),
            MessagePayload::GameState(GameStateData {
                tick,
                world_size,
                entities,
                global_state: GlobalStateData {
                    simulation_speed: 1.0,
                    is_running: true,
                    elapsed_time: 0.0,
                    resources: HashMap::new(),
                    population: PopulationData {
                        total: 0,
                        by_type: HashMap::new(),
                        by_region: HashMap::new(),
                    },
                },
                changed_entities: None,
            }),
        )
    }

    fn create_entity_update(&mut self, entity_id: EntityId, update_type: EntityUpdateType, entity: Option<EntitySnapshot>) -> IpcMessage {
        IpcMessage::new(
            self.next_sequence(),
            MessagePayload::EntityUpdate(EntityUpdateData {
                entity_id,
                update_type,
                entity,
                position: None,
            }),
        )
    }

    fn create_pack_definitions(&mut self, packs: Vec<PackMetadata>, visual_registry: VisualRegistry) -> IpcMessage {
        IpcMessage::new(
            self.next_sequence(),
            MessagePayload::PackDefinitions(PackDefinitionsData {
                packs: packs.clone(),
                visual_registry,
                load_order: packs.iter().map(|p| p.id.clone()).collect(),
            }),
        )
    }

    fn create_tick_update(&mut self, tick: Tick, elapsed_time: f32) -> IpcMessage {
        IpcMessage::new(
            self.next_sequence(),
            MessagePayload::TickUpdate(TickUpdateData {
                tick,
                elapsed_time,
                performance: None,
            }),
        )
    }

    fn create_command(&mut self, command_id: String, command_type: String, parameters: HashMap<String, serde_json::Value>) -> IpcMessage {
        IpcMessage::new(
            self.next_sequence(),
            MessagePayload::Command(CommandData {
                command_id,
                command_type,
                parameters,
                requires_ack: true,
            }),
        )
    }

    fn create_heartbeat(&mut self, sender: String) -> IpcMessage {
        IpcMessage::new(
            self.next_sequence(),
            MessagePayload::Heartbeat(HeartbeatData {
                sender,
                sent_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                metrics: None,
            }),
        )
    }

    fn create_error(&mut self, code: String, message: String, severity: ErrorSeverity) -> IpcMessage {
        IpcMessage::new(
            self.next_sequence(),
            MessagePayload::Error(ErrorData {
                code,
                message,
                severity,
                context: HashMap::new(),
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let mut factory = DefaultMessageFactory::new();

        let msg = factory.create_game_state(1, (64, 64), Vec::new());
        assert_eq!(msg.seq_num, 1);
        assert_eq!(msg.version, IPC_PROTOCOL_VERSION);

        let msg2 = factory.create_game_state(2, (64, 64), Vec::new());
        assert_eq!(msg2.seq_num, 2);
    }

    #[test]
    fn test_message_validation() {
        let msg = IpcMessage::new(1, MessagePayload::Heartbeat(HeartbeatData {
            sender: "test".to_string(),
            sent_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            metrics: None,
        }));

        assert!(msg.validate().is_ok());

        let mut invalid_msg = msg.clone();
        invalid_msg.version = 999;
        assert!(matches!(invalid_msg.validate(), Err(IpcError::VersionMismatch { .. })));
    }

    #[test]
    fn test_serialization() {
        let msg = IpcMessage::new(1, MessagePayload::Heartbeat(HeartbeatData {
            sender: "test".to_string(),
            sent_at: 1234567890,
            metrics: None,
        }));

        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: IpcMessage = serde_json::from_str(&json).unwrap();

        assert_eq!(msg.seq_num, deserialized.seq_num);
        assert_eq!(msg.version, deserialized.version);
    }
}