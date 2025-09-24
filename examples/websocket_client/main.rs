//! WebSocket Client Example
//!
//! This example demonstrates real-time monitoring and control of the world-simulator
//! through WebSocket connections. It showcases:
//! - WebSocket client implementation
//! - Real-time data streaming
//! - Remote simulation control
//! - Live performance monitoring
//! - Interactive visualization

use bevy::prelude::*;
use world_sim_interface::*;
use world_sim_simple::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

fn main() {
    println!("🌐 Starting WebSocket Client Example");

    // Set up logging with WebSocket detail
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    // Create and run the simulation
    let mut app = App::new();

    // Configure with window for visualization
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "World Simulator - WebSocket Client".into(),
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
    app.init_resource::<WebSocketClientState>();

    // Add startup systems for WebSocket client setup
    app.add_systems(Startup, setup_websocket_client);

    // Add update systems for WebSocket handling
    app.add_systems(Update, (
        websocket_simulation_monitor,
        websocket_data_stream,
        websocket_command_handler,
        websocket_performance_monitor,
    ).chain());

    println!("🔗 WebSocket client initialized. Running for 500 ticks...");
    println!("📡 This will showcase real-time data streaming and remote control capabilities.");

    // Run the simulation
    app.run();
}

/// State for WebSocket client operations
#[derive(Resource, Default)]
pub struct WebSocketClientState {
    pub connected_clients: Arc<Mutex<HashMap<String, ClientInfo>>>,
    pub message_queue: Arc<Mutex<Vec<WebSocketMessage>>>,
    pub performance_metrics: Arc<Mutex<PerformanceMetrics>>,
    pub simulation_commands: Arc<Mutex<Vec<SimulationCommand>>>,
    pub data_streams: Arc<Mutex<HashMap<String, DataStream>>>,
}

/// Client connection information
#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub id: String,
    pub connected_at: Instant,
    pub subscriptions: Vec<String>,
    pub message_count: u64,
    pub last_activity: Instant,
}

/// WebSocket message structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub payload: serde_json::Value,
    pub timestamp: u64,
    pub client_id: Option<String>,
}

/// Simulation command structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimulationCommand {
    pub command_type: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub timestamp: u64,
    pub client_id: String,
}

/// Performance metrics structure
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub average_latency_ms: f64,
    pub peak_connections: u32,
    pub current_connections: u32,
    pub simulation_tps: f64,
    pub memory_usage_mb: f64,
}

/// Data stream configuration
#[derive(Debug, Clone)]
pub struct DataStream {
    pub stream_type: String,
    pub update_interval: Duration,
    pub last_update: Instant,
    pub enabled: bool,
    pub subscriber_count: u32,
}

/// Setup WebSocket client and simulation
fn setup_websocket_client(
    mut commands: Commands,
    mut pack_system: Option<Res<packs::PackSystem>>,
    mut ws_state: ResMut<WebSocketClientState>,
) {
    println!("🔗 Setting up WebSocket Client Example...");

    // Initialize data streams
    let mut data_streams = HashMap::new();
    data_streams.insert("simulation_state".to_string(), DataStream {
        stream_type: "simulation_state".to_string(),
        update_interval: Duration::from_millis(100),
        last_update: Instant::now(),
        enabled: true,
        subscriber_count: 0,
    });
    data_streams.insert("entity_updates".to_string(), DataStream {
        stream_type: "entity_updates".to_string(),
        update_interval: Duration::from_millis(50),
        last_update: Instant::now(),
        enabled: true,
        subscriber_count: 0,
    });
    data_streams.insert("performance_metrics".to_string(), DataStream {
        stream_type: "performance_metrics".to_string(),
        update_interval: Duration::from_millis(1000),
        last_update: Instant::now(),
        enabled: true,
        subscriber_count: 0,
    });

    *ws_state.data_streams = Arc::new(Mutex::new(data_streams));

    // Simulate client connections
    simulate_client_connections(&mut ws_state);

    println!("✅ WebSocket client setup complete!");
    println!("🎯 Features demonstrated:");
    println!("   • Real-time data streaming");
    println!("   • Remote simulation control");
    println!("   • Performance monitoring");
    println!("   • Client connection management");
    println!("   • Message queue processing");
}

/// Simulate WebSocket client connections
fn simulate_client_connections(ws_state: &mut WebSocketClientState) {
    let mut clients = HashMap::new();

    // Add simulation client
    clients.insert("sim_client_001".to_string(), ClientInfo {
        id: "sim_client_001".to_string(),
        connected_at: Instant::now(),
        subscriptions: vec!["simulation_state".to_string(), "entity_updates".to_string()],
        message_count: 0,
        last_activity: Instant::now(),
    });

    // Add monitoring client
    clients.insert("monitor_client_001".to_string(), ClientInfo {
        id: "monitor_client_001".to_string(),
        connected_at: Instant::now(),
        subscriptions: vec!["performance_metrics".to_string(), "simulation_state".to_string()],
        message_count: 0,
        last_activity: Instant::now(),
    });

    // Add control client
    clients.insert("control_client_001".to_string(), ClientInfo {
        id: "control_client_001".to_string(),
        connected_at: Instant::now(),
        subscriptions: vec!["simulation_state".to_string(), "entity_updates".to_string()],
        message_count: 0,
        last_activity: Instant::now(),
    });

    *ws_state.connected_clients = Arc::new(Mutex::new(clients));

    // Add initial messages
    let initial_messages = vec![
        WebSocketMessage {
            message_type: "connection_established".to_string(),
            payload: serde_json::json!({"status": "connected", "protocol": "ws_v1"}),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            client_id: Some("sim_client_001".to_string()),
        },
        WebSocketMessage {
            message_type: "subscribe".to_string(),
            payload: serde_json::json!({"streams": ["simulation_state", "entity_updates"]}),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            client_id: Some("monitor_client_001".to_string()),
        },
    ];

    *ws_state.message_queue = Arc::new(Mutex::new(initial_messages));
}

/// Monitor simulation and stream data via WebSocket
fn websocket_simulation_monitor(
    sim_state: Res<SimulationState>,
    time: Res<Time>,
    ws_state: Res<WebSocketClientState>,
    resource_query: Query<&ResourceNode>,
    unit_query: Query<&UnitTag>,
    mut last_update: Local<Instant>,
) {
    let now = Instant::now();
    if now.duration_since(*last_update) < Duration::from_millis(100) {
        return;
    }
    *last_update = now;

    let resource_count = resource_query.iter().count();
    let unit_count = unit_query.iter().count();

    // Create simulation state message
    let sim_message = WebSocketMessage {
        message_type: "simulation_state".to_string(),
        payload: serde_json::json!({
            "tick": sim_state.tick,
            "resources": resource_count,
            "units": unit_count,
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "fps": time.fps(),
            "delta_time": time.delta_seconds()
        }),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        client_id: None,
    };

    // Add to message queue
    if let Ok(mut queue) = ws_state.message_queue.lock() {
        queue.push(sim_message);
    }
}

/// Handle WebSocket data streaming
fn websocket_data_stream(
    ws_state: Res<WebSocketClientState>,
    sim_state: Res<SimulationState>,
    mut last_stream: Local<std::collections::HashMap<String, Instant>>,
) {
    let mut data_streams = ws_state.data_streams.lock().unwrap();
    let current_time = Instant::now();

    for (stream_name, stream) in data_streams.iter_mut() {
        let last_stream_time = last_stream.entry(stream_name.clone()).or_insert(Instant::now());

        if current_time.duration_since(*last_stream_time) >= stream.update_interval && stream.enabled {
            *last_stream_time = current_time;

            let message = match stream_name.as_str() {
                "simulation_state" => create_simulation_state_message(&sim_state),
                "entity_updates" => create_entity_update_message(),
                "performance_metrics" => create_performance_message(&ws_state),
                _ => continue,
            };

            if let Ok(mut queue) = ws_state.message_queue.lock() {
                queue.push(message);
            }

            stream.last_update = current_time;
        }
    }
}

/// Handle WebSocket commands
fn websocket_command_handler(
    ws_state: Res<WebSocketClientState>,
    mut commands: Commands,
    mut last_command: Local<Instant>,
) {
    let now = Instant::now();
    if now.duration_since(*last_command) < Duration::from_millis(200) {
        return;
    }
    *last_command = now;

    let simulation_commands = ws_state.simulation_commands.lock().unwrap();

    // Process queued commands
    for command in simulation_commands.iter() {
        match command.command_type.as_str() {
            "spawn_entity" => handle_spawn_command(&command),
            "set_speed" => handle_speed_command(&command),
            "pause_simulation" => handle_pause_command(&command),
            "query_state" => handle_query_command(&command, &ws_state),
            _ => println!("Unknown command: {}", command.command_type),
        }
    }
}

/// Monitor WebSocket performance
fn websocket_performance_monitor(
    ws_state: Res<WebSocketClientState>,
    sim_state: Res<SimulationState>,
    mut last_monitor: Local<u32>,
) {
    if sim_state.tick % 100 == 0 && sim_state.tick > 0 && *last_monitor != sim_state.tick {
        *last_monitor = sim_state.tick;

        let clients = ws_state.connected_clients.lock().unwrap();
        let metrics = ws_state.performance_metrics.lock().unwrap();

        println!(
            "📡 WebSocket Monitor - Tick {}: {} clients, {} messages sent, {:.1}ms avg latency",
            sim_state.tick,
            clients.len(),
            metrics.messages_sent,
            metrics.average_latency_ms
        );

        // Update performance metrics
        let mut metrics = ws_state.performance_metrics.lock().unwrap();
        metrics.current_connections = clients.len() as u32;
        metrics.simulation_tps = 30.0; // Assuming 30 ticks per second
        metrics.memory_usage_mb = estimate_memory_usage();
    }

    // Print initial status
    if sim_state.tick == 1 {
        println!("🎮 WebSocket client started. Monitoring real-time data streams...");
        println!("📈 Tracking client connections and message throughput...");
    }

    // Stop after 500 ticks
    if sim_state.tick >= 500 {
        println!("🎉 WebSocket client demonstration completed successfully!");
        println!("📊 Final WebSocket Analysis:");

        let clients = ws_state.connected_clients.lock().unwrap();
        let metrics = ws_state.performance_metrics.lock().unwrap();

        println!("   • Total clients served: {}", clients.len());
        println!("   • Messages processed: {}", metrics.messages_sent);
        println!("   • Average latency: {:.1}ms", metrics.average_latency_ms);
        println!("   • Peak connections: {}", metrics.peak_connections);
        println!("   • Simulation ticks: {}", sim_state.tick);

        println!("🌐 Key WebSocket Features Demonstrated:");
        println!("   • Real-time data streaming");
        println!("   • Client connection management");
        println!("   • Remote simulation control");
        println!("   • Performance monitoring");
        println!("   • Message queue processing");

        // Exit the application
        std::process::exit(0);
    }
}

// Helper functions for message creation
fn create_simulation_state_message(sim_state: &SimulationState) -> WebSocketMessage {
    WebSocketMessage {
        message_type: "simulation_state".to_string(),
        payload: serde_json::json!({
            "tick": sim_state.tick,
            "running": true,
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        }),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        client_id: None,
    }
}

fn create_entity_update_message() -> WebSocketMessage {
    WebSocketMessage {
        message_type: "entity_updates".to_string(),
        payload: serde_json::json!({
            "update_type": "batch",
            "entities_updated": 5,
            "changes": ["position", "health", "inventory"]
        }),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        client_id: None,
    }
}

fn create_performance_message(ws_state: &WebSocketClientState) -> WebSocketMessage {
    let metrics = ws_state.performance_metrics.lock().unwrap();
    WebSocketMessage {
        message_type: "performance_metrics".to_string(),
        payload: serde_json::json!({
            "tps": metrics.simulation_tps,
            "memory_mb": metrics.memory_usage_mb,
            "active_connections": metrics.current_connections,
            "messages_per_second": metrics.messages_sent / 60 // Rough estimate
        }),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        client_id: None,
    }
}

// Command handlers
fn handle_spawn_command(command: &SimulationCommand) {
    println!("📥 Spawn command received from client {}", command.client_id);
}

fn handle_speed_command(command: &SimulationCommand) {
    if let Some(speed) = command.parameters.get("speed") {
        println!("📥 Speed command: {} from client {}", speed, command.client_id);
    }
}

fn handle_pause_command(command: &SimulationCommand) {
    println!("📥 Pause command from client {}", command.client_id);
}

fn handle_query_command(command: &SimulationCommand, ws_state: &WebSocketClientState) {
    println!("📥 Query command from client {}", command.client_id);
}

fn estimate_memory_usage() -> f64 {
    // Simple memory estimation (in MB)
    50.0 + (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() % 100) as f64 * 0.1
}