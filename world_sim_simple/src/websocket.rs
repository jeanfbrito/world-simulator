use bevy::prelude::*;
use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_tungstenite::{accept_async, tungstenite::Message};

// Messages from client to server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Connect {
        client_id: String,
    },
    Disconnect {
        client_id: String,
    },
    Command {
        action: String,
        data: serde_json::Value,
    },
    SetTile {
        x: usize,
        y: usize,
        tile_type: String,
    },
    SpawnWorker {
        x: usize,
        y: usize,
    },
    PlayPause,
    SetSpeed {
        speed: f32,
    },
    GenerateMap {
        map_type: String,
    },
    RespawnPeasants,
    RequestState,
    Ping,
    // Debug messages
    DebugGetSystemInfo,
    DebugGetComponents,
    DebugValidateState,
}

// Messages from server to client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Welcome {
        client_id: String,
    },
    GameState {
        state: GameStateSnapshot,
    },
    TileUpdate {
        x: usize,
        y: usize,
        tile_type: String,
    },
    EntityUpdate {
        entities: Vec<EntityData>,
    },
    TickUpdate {
        tick: u32,
    },
    Error {
        message: String,
    },
    // Debug responses
    DebugInfo {
        info: serde_json::Value,
    },
    ValidationResult {
        valid: bool,
        details: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateSnapshot {
    pub tick: u32,
    pub running: bool,
    pub speed: f32,
    pub map_size: usize,
    pub tiles: Vec<Vec<String>>,
    pub entities: Vec<EntityData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityData {
    pub id: String,
    pub entity_type: String,
    pub x: usize,
    pub y: usize,
    pub data: HashMap<String, serde_json::Value>,
}

// Resource to store WebSocket connections
#[derive(Resource, Clone)]
pub struct WebSocketConnections {
    pub sender: mpsc::UnboundedSender<ServerMessage>,
}

// Resource to receive client messages
#[derive(Resource, Clone)]
pub struct ClientMessageQueue {
    pub messages: Arc<Mutex<Vec<ClientMessage>>>,
}

impl Default for ClientMessageQueue {
    fn default() -> Self {
        Self {
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

// Plugin to add WebSocket functionality
pub struct WebSocketPlugin;

impl Plugin for WebSocketPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = mpsc::unbounded_channel::<ServerMessage>();
        let message_queue = ClientMessageQueue::default();

        app.insert_resource(WebSocketConnections { sender: tx.clone() })
            .insert_resource(message_queue.clone())
            .add_systems(Startup, start_websocket_server)
            .add_systems(
                Update,
                (
                    process_client_messages,
                    broadcast_game_state.run_if(should_broadcast),
                ),
            );

        // Start WebSocket server in background
        let msg_queue = message_queue.messages.clone();
        let tx_clone = tx.clone();
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                run_websocket_server(tx_clone, rx, msg_queue).await;
            });
        });
    }
}

fn start_websocket_server() {
    info!("WebSocket server starting on ws://localhost:8080");
}

async fn run_websocket_server(
    tx: mpsc::UnboundedSender<ServerMessage>,
    mut rx: mpsc::UnboundedReceiver<ServerMessage>,
    message_queue: Arc<Mutex<Vec<ClientMessage>>>,
) {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("WebSocket server listening on ws://localhost:8080");

    let clients: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<Message>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let clients_clone = clients.clone();

    // Broadcast task
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let clients = clients_clone.lock().unwrap();
            let msg_json = serde_json::to_string(&msg).unwrap();

            for (_, client_tx) in clients.iter() {
                let _ = client_tx.send(Message::Text(msg_json.clone()));
            }
        }
    });

    // Accept connections
    while let Ok((stream, addr)) = listener.accept().await {
        let clients = clients.clone();
        let message_queue = message_queue.clone();
        let tx = tx.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, addr, clients, message_queue, tx).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: std::net::SocketAddr,
    clients: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<Message>>>>,
    message_queue: Arc<Mutex<Vec<ClientMessage>>>,
    server_tx: mpsc::UnboundedSender<ServerMessage>,
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let (client_tx, mut client_rx) = mpsc::unbounded_channel::<Message>();

    let client_id = format!("client_{}", uuid::Uuid::new_v4());
    println!(
        "New WebSocket connection from {} with id {}",
        addr, client_id
    );

    // Store client sender
    {
        let mut clients = clients.lock().unwrap();
        clients.insert(client_id.clone(), client_tx);
    }

    // Send welcome message
    let welcome = ServerMessage::Welcome {
        client_id: client_id.clone(),
    };
    ws_sender
        .send(Message::Text(serde_json::to_string(&welcome)?))
        .await?;

    // Send initial game state
    {
        let mut queue = message_queue.lock().unwrap();
        queue.push(ClientMessage::RequestState);
    }

    // Spawn task to send messages to client
    let send_task = tokio::spawn(async move {
        while let Some(msg) = client_rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Receive messages from client
    while let Some(msg) = ws_receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    match &client_msg {
                        ClientMessage::Ping => {
                            // Respond to ping immediately to keep connection alive
                            // No need to process ping in game loop
                        }
                        _ => {
                            let mut queue = message_queue.lock().unwrap();
                            queue.push(client_msg);
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            Err(_) => break,
            _ => {}
        }
    }

    // Clean up
    send_task.abort();
    {
        let mut clients = clients.lock().unwrap();
        clients.remove(&client_id);
    }

    println!("Client {} disconnected", client_id);
    Ok(())
}

// System to process client messages
fn process_client_messages(
    message_queue: ResMut<ClientMessageQueue>,
    mut sim_state: ResMut<crate::SimulationState>,
    mut world_map: ResMut<crate::WorldMap>,
    mut commands: Commands,
    connections: Res<WebSocketConnections>,
    peasant_query: Query<Entity, With<crate::components::UnitTag>>,
) {
    let mut messages = message_queue.messages.lock().unwrap();

    for msg in messages.drain(..) {
        match msg {
            ClientMessage::PlayPause => {
                sim_state.running = !sim_state.running;
            }
            ClientMessage::SetSpeed { speed } => {
                sim_state.speed = speed;
            }
            ClientMessage::SetTile { x, y, tile_type } => {
                if x < crate::MAP_SIZE && y < crate::MAP_SIZE {
                    world_map.tiles[y][x] = parse_tile_type(&tile_type);
                }
            }
            ClientMessage::GenerateMap { map_type } => {
                generate_map(&mut world_map, &map_type);
            }
            ClientMessage::RespawnPeasants => {
                // Despawn all existing peasants
                for entity in peasant_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                
                // Respawn 5 new peasants
                let mut rng = rand::thread_rng();
                let mut spawned = 0;
                
                while spawned < 5 {
                    let x = rng.gen_range(20..44);
                    let y = rng.gen_range(20..44);
                    
                    // Check if tile is walkable
                    if world_map.tiles[y][x].is_walkable() {
                        crate::spawning::spawn_peasant(&mut commands, spawned + 1, x, y);
                        spawned += 1;
                        println!("[RESPAWN] Spawned peasant {} at ({}, {})", spawned, x, y);
                    }
                }
                
                println!("[RESPAWN] Completed respawning 5 peasants");
                
                // Trigger state update
                sim_state.set_changed();
            }
            ClientMessage::RequestState => {
                // Trigger immediate state broadcast
                sim_state.set_changed();
            }
            ClientMessage::DebugGetSystemInfo => {
                let info = serde_json::json!({
                    "tick": sim_state.tick,
                    "running": sim_state.running,
                    "speed": sim_state.speed,
                    "map_size": crate::MAP_SIZE,
                    "worker_count": 5, // TODO: Query actual count
                });
                let msg = ServerMessage::DebugInfo { info };
                let _ = connections.sender.send(msg);
            }
            ClientMessage::DebugGetComponents => {
                // Will be expanded as we add components
                let info = serde_json::json!({
                    "components": ["Worker", "TileEntity", "WorldMap"],
                    "systems": ["simulation_system", "render_map_system", "ui_system"],
                });
                let msg = ServerMessage::DebugInfo { info };
                let _ = connections.sender.send(msg);
            }
            ClientMessage::DebugValidateState => {
                // Validate system state
                let mut valid = true;
                let mut details = Vec::new();

                if world_map.tiles.len() != crate::MAP_SIZE {
                    valid = false;
                    details.push("Invalid map height");
                }

                for row in world_map.tiles.iter() {
                    if row.len() != crate::MAP_SIZE {
                        valid = false;
                        details.push("Invalid map width");
                        break;
                    }
                }

                let msg = ServerMessage::ValidationResult {
                    valid,
                    details: details.join(", "),
                };
                let _ = connections.sender.send(msg);
            }
            _ => {}
        }
    }
}

fn parse_tile_type(tile_type: &str) -> crate::TileType {
    match tile_type {
        "grass" => crate::TileType::Grass,
        "stone" => crate::TileType::Stone,
        "sand" => crate::TileType::Sand,
        "water" => crate::TileType::Water,
        "deep-water" => crate::TileType::DeepWater,
        "tree" => crate::TileType::Tree,
        "ore" => crate::TileType::Ore,
        "berry" => crate::TileType::Berry,
        "wall" => crate::TileType::Wall,
        "floor" => crate::TileType::Floor,
        "door" => crate::TileType::Door,
        "storage" => crate::TileType::Storage,
        "workshop" => crate::TileType::Workshop,
        "blocked" => crate::TileType::Blocked,
        _ => crate::TileType::Empty,
    }
}

fn generate_map(world_map: &mut crate::WorldMap, map_type: &str) {
    match map_type {
        "island" => {
            // Generate island map
            let center = crate::MAP_SIZE / 2;
            for y in 0..crate::MAP_SIZE {
                for x in 0..crate::MAP_SIZE {
                    let dist = ((x as f32 - center as f32).powi(2)
                        + (y as f32 - center as f32).powi(2))
                    .sqrt();
                    let max_dist = center as f32;

                    if dist > max_dist * 0.9 {
                        world_map.tiles[y][x] = crate::TileType::DeepWater;
                    } else if dist > max_dist * 0.75 {
                        world_map.tiles[y][x] = crate::TileType::Water;
                    } else if dist > max_dist * 0.6 {
                        world_map.tiles[y][x] = crate::TileType::Sand;
                    } else {
                        world_map.tiles[y][x] = crate::TileType::Grass;
                    }
                }
            }
        }
        "forest" => {
            // Fill with grass and trees
            for y in 0..crate::MAP_SIZE {
                for x in 0..crate::MAP_SIZE {
                    world_map.tiles[y][x] = if rand::random::<f32>() > 0.7 {
                        crate::TileType::Tree
                    } else {
                        crate::TileType::Grass
                    };
                }
            }
        }
        _ => {
            // Random map
            let types = [
                crate::TileType::Grass,
                crate::TileType::Stone,
                crate::TileType::Sand,
                crate::TileType::Water,
            ];
            for y in 0..crate::MAP_SIZE {
                for x in 0..crate::MAP_SIZE {
                    world_map.tiles[y][x] = types[rand::random::<usize>() % types.len()];
                }
            }
        }
    }
}

// System to broadcast game state
fn broadcast_game_state(
    connections: Res<WebSocketConnections>,
    mut sim_state: ResMut<crate::SimulationState>,
    world_map: Res<crate::WorldMap>,
    workers: Query<
        (
            &crate::components::NameComponent,
            &crate::components::HealthComponent,
            &crate::TileEntity,
            &crate::components::PositionComponent,
            Option<&crate::components::UnitNeedsV2>,
            Option<&crate::components::UnitInventory>,
            Option<&crate::ai::ActionPlan>,
            Option<&crate::components::WorkProgress>,
            Option<&crate::components::UnitMind>,  // Add UnitMind component
        ),
        With<crate::components::UnitTag>,
    >,
    berry_bushes: Query<
        (&crate::TileEntity, &crate::components::ResourceNode),
        With<crate::ai::BerryBushTag>,
    >,
    trees: Query<
        &crate::TileEntity,
        With<crate::ai::TreeTag>,
    >,
) {
    let mut entities = Vec::new();

    // Track previous positions to detect movement
    use std::collections::HashMap as StdHashMap;
    static mut PREV_POSITIONS: Option<StdHashMap<String, (usize, usize)>> = None;
    unsafe {
        if PREV_POSITIONS.is_none() {
            PREV_POSITIONS = Some(StdHashMap::new());
        }
    }
    
    for (name, health, tile, position, needs, inventory, plan, work, mind) in workers.iter() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), serde_json::json!(name.display_name));
        data.insert("health".to_string(), serde_json::json!(health.current));
        
        // Add needs data if available (energy comes from here now)
        if let Some(needs) = needs {
            data.insert("hunger".to_string(), serde_json::json!(needs.hunger() * 100.0)); // Convert to percentage
            data.insert("energy".to_string(), serde_json::json!(needs.energy() * 100.0)); // Energy from needs
            data.insert("morale".to_string(), serde_json::json!(needs.morale() * 100.0));
            data.insert("shelter".to_string(), serde_json::json!(needs.has_shelter));
        } else {
            // Fallback values if no needs component
            data.insert("hunger".to_string(), serde_json::json!(0.0));
            data.insert("energy".to_string(), serde_json::json!(100.0));
            data.insert("morale".to_string(), serde_json::json!(100.0));
            data.insert("shelter".to_string(), serde_json::json!(false));
        }
        
        // Detect movement
        let current_pos = (tile.x, tile.y);
        let is_moving = unsafe {
            let prev_map = PREV_POSITIONS.as_mut().unwrap();
            let worker_id = format!("worker_{}", name.name);
            let prev_pos = prev_map.get(&worker_id).copied();
            prev_map.insert(worker_id, current_pos);
            
            if let Some(prev) = prev_pos {
                prev != current_pos
            } else {
                false
            }
        };
        
        // Get status and action from UnitMind component - single source of truth
        let (status, action) = if let Some(mind) = mind {
            // Status is based on movement
            let status = if mind.is_moving() { "walking" } else { "standing" };
            // Action is the mind state description
            let action = mind.description();
            (status, action)
        } else {
            // Fallback if no UnitMind component
            ("standing", "idle".to_string())
        };
        
        data.insert("status".to_string(), serde_json::json!(status));
        data.insert("action".to_string(), serde_json::json!(action.clone()));
        data.insert("mind_state".to_string(), serde_json::json!(action)); // Add mind_state for viewer compatibility
        
        // Add inventory data if available
        if let Some(inv) = inventory {
            let inventory_items: Vec<String> = inv.items.iter()
                .map(|(resource, count)| format!("{:?}:{}", resource, count))
                .collect();
            data.insert("inventory".to_string(), serde_json::json!(inventory_items));
            data.insert("inventory_weight".to_string(), serde_json::json!(inv.current_weight));
        }

        entities.push(EntityData {
            id: format!("worker_{}", name.name),
            entity_type: "worker".to_string(),
            x: tile.x,
            y: tile.y,
            data,
        });
    }
    
    // Add berry bushes to entities
    for (tile, resource) in berry_bushes.iter() {
        let mut data = HashMap::new();
        data.insert("amount".to_string(), serde_json::json!(resource.amount));
        data.insert("max_amount".to_string(), serde_json::json!(resource.max_amount));
        entities.push(EntityData {
            id: format!("berry_{}_{}", tile.x, tile.y),
            entity_type: "berry".to_string(),
            x: tile.x,
            y: tile.y,
            data,
        });
    }
    
    // Add trees to entities
    for tile in trees.iter() {
        entities.push(EntityData {
            id: format!("tree_{}_{}", tile.x, tile.y),
            entity_type: "tree".to_string(),
            x: tile.x,
            y: tile.y,
            data: HashMap::new(),
        });
    }

    let tiles: Vec<Vec<String>> = world_map
        .tiles
        .iter()
        .map(|row| {
            row.iter()
                .map(|t| format!("{:?}", t).to_lowercase())
                .collect()
        })
        .collect();

    let snapshot = GameStateSnapshot {
        tick: sim_state.tick,
        running: sim_state.running,
        speed: sim_state.speed,
        map_size: crate::MAP_SIZE,
        tiles,
        entities,
    };

    let msg = ServerMessage::GameState { state: snapshot };
    let _ = connections.sender.send(msg);

    // Clear changed flag after broadcasting
    sim_state.clear_changed();
}

fn should_broadcast(time: Res<Time>, sim_state: Res<crate::SimulationState>) -> bool {
    // Broadcast every 100ms or when simulation state changes
    use std::sync::atomic::{AtomicU32, Ordering};
    static LAST_BROADCAST_MS: AtomicU32 = AtomicU32::new(0);

    let elapsed_ms = (time.elapsed_secs() * 1000.0) as u32;
    let last_ms = LAST_BROADCAST_MS.load(Ordering::Relaxed);

    if elapsed_ms - last_ms > 100 || sim_state.is_changed() {
        LAST_BROADCAST_MS.store(elapsed_ms, Ordering::Relaxed);
        true
    } else {
        false
    }
}
