use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt};
use tokio::io::{AsyncBufRead, AsyncBufReadExt, BufReader};
use world_sim_interface::ipc::{IpcMessage, MessagePayload, PackDefinitionsData};
use tracing::{info, error, warn, debug};
use clap::Parser;
mod pack_loader;
use pack_loader::{load_pack_for_ipc, PackLoaderError};

/// Sim Viewer - IPC to WebSocket bridge
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IPC input file (stdin if not specified)
    #[arg(short, long)]
    ipc_file: Option<String>,

    /// WebSocket server port
    #[arg(short, long, default_value = "8080")]
    port: u16,

    /// Path to pack files directory
    #[arg(long)]
    pack_path: Option<String>,

    /// Enable verbose logging
    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

#[derive(Clone)]
pub struct ViewerState {
    clients: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>,
    latest_state: Arc<RwLock<Option<MessagePayload>>>,
    pack_path: Option<String>,
}

impl ViewerState {
    pub fn new(pack_path: Option<String>) -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            latest_state: Arc::new(RwLock::new(None)),
            pack_path,
        }
    }

    async fn add_client(&self, id: String, sender: mpsc::UnboundedSender<Message>) {
        let mut clients = self.clients.write().await;
        clients.insert(id.clone(), sender);
        info!("Client {} connected. Total clients: {}", id, clients.len());
    }

    async fn remove_client(&self, id: String) {
        let mut clients = self.clients.write().await;
        clients.remove(&id);
        info!("Client {} disconnected. Total clients: {}", id, clients.len());
    }

    async fn broadcast_to_clients(&self, message: Message) {
        let clients = self.clients.read().await;
        let mut disconnected = Vec::new();

        for (id, sender) in clients.iter() {
            if sender.send(message.clone()).is_err() {
                disconnected.push(id.clone());
            }
        }

        // Remove disconnected clients
        drop(clients);
        if !disconnected.is_empty() {
            let mut clients = self.clients.write().await;
            for id in disconnected {
                clients.remove(&id);
                warn!("Removed disconnected client: {}", id);
            }
        }
    }

    async fn update_state(&self, payload: MessagePayload) {
        let mut latest_state = self.latest_state.write().await;
        *latest_state = Some(payload.clone());
        drop(latest_state);

        // Broadcast to all WebSocket clients
        let json_message = serde_json::to_string(&payload)
            .unwrap_or_else(|e| {
                error!("Failed to serialize payload: {}", e);
                r#"{"error": "Failed to serialize state"}"#.to_string()
            });

        self.broadcast_to_clients(Message::Text(json_message)).await;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(if args.verbose { tracing::Level::DEBUG } else { tracing::Level::INFO })
        .init();

    let state = ViewerState::new(args.pack_path.clone());

    // Start IPC reader
    let ipc_state = state.clone();
    tokio::spawn(async move {
        if let Err(e) = read_ipc_messages(args.ipc_file, ipc_state).await {
            error!("IPC reader error: {}", e);
        }
    });

    // Start WebSocket server
    start_websocket_server(args.port, state).await?;

    Ok(())
}

async fn read_ipc_messages(
    ipc_file: Option<String>,
    state: ViewerState,
) -> anyhow::Result<()> {
    let mut reader: Box<dyn AsyncBufRead + Unpin + Send> = if let Some(file_path) = ipc_file {
        // Read from file
        let file = tokio::fs::File::open(file_path).await?;
        Box::new(BufReader::new(file))
    } else {
        // Read from stdin
        Box::new(BufReader::new(tokio::io::stdin()))
    };

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line).await {
            Ok(0) => break, // EOF
            Ok(_) => {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    process_ipc_line(trimmed, &state).await;
                }
            }
            Err(e) => {
                error!("Error reading IPC input: {}", e);
                break;
            }
        }
    }

    Ok(())
}

pub async fn process_ipc_line(line: &str, state: &ViewerState) {
    // Skip lines that don't look like JSON IPC messages
    if !line.starts_with("{\"version\":") {
        return;
    }

    match serde_json::from_str::<IpcMessage>(line) {
        Ok(ipc_message) => {
            match ipc_message.payload {
                MessagePayload::GameState(game_state) => {
                    info!("Received game state for tick {}", game_state.tick);
                    state.update_state(MessagePayload::GameState(game_state)).await;
                }
                MessagePayload::PackDefinitions(pack_defs) => {
                    info!("Received pack definitions from engine");

                    // Load actual pack files and enhance the definitions
                    if let Some(pack_path) = &state.pack_path {
                        match load_pack_definitions_from_path(pack_path, pack_defs.clone()).await {
                            Ok(enhanced_pack_defs) => {
                                info!("Successfully loaded and enhanced pack definitions");
                                state.update_state(MessagePayload::PackDefinitions(enhanced_pack_defs)).await;
                            }
                            Err(e) => {
                                error!("Failed to load pack definitions: {}", e);
                                // Fall back to original pack definitions
                                state.update_state(MessagePayload::PackDefinitions(pack_defs)).await;
                            }
                        }
                    } else {
                        warn!("No pack path provided, using engine pack definitions as-is");
                        state.update_state(MessagePayload::PackDefinitions(pack_defs)).await;
                    }
                }
                MessagePayload::Heartbeat(heartbeat) => {
                    info!("Received heartbeat from {}", heartbeat.sender);
                    state.update_state(MessagePayload::Heartbeat(heartbeat)).await;
                }
                _ => {
                    debug!("Received unhandled IPC message type");
                }
            }
        }
        Err(e) => {
            warn!("Failed to parse IPC message: {}", e);
        }
    }
}

/// Load pack definitions from file system and enhance engine data
async fn load_pack_definitions_from_path(
    pack_path: &str,
    engine_pack_defs: PackDefinitionsData,
) -> Result<PackDefinitionsData, PackLoaderError> {
    info!("Loading pack definitions from: {}", pack_path);

    // Try to load pack files to get enhanced visual definitions
    if let Ok((enhanced_metadata, enhanced_visual_registry)) = load_pack_for_ipc(pack_path) {
        info!("Successfully loaded pack files, enhancing visual definitions");

        // Use enhanced visual registry but keep engine metadata
        let enhanced_pack_defs = PackDefinitionsData {
            packs: vec![enhanced_metadata],
            visual_registry: enhanced_visual_registry,
            load_order: engine_pack_defs.load_order,
        };

        Ok(enhanced_pack_defs)
    } else {
        // If pack loading fails, return original engine data with a warning
        warn!("Failed to load pack files, using engine definitions");
        Ok(engine_pack_defs)
    }
}

async fn start_websocket_server(port: u16, state: ViewerState) -> anyhow::Result<()> {
    use tokio::net::TcpListener;

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    info!("WebSocket server listening on ws://localhost:{}", port);

    while let Ok((stream, addr)) = listener.accept().await {
        let client_id = uuid::Uuid::new_v4().to_string();
        let state = state.clone();

        tokio::spawn(async move {
            match accept_async(stream).await {
                Ok(ws_stream) => {
                    info!("WebSocket connection established from {}", addr);
                    handle_websocket_connection(client_id, ws_stream, state).await;
                }
                Err(e) => {
                    error!("Failed to accept WebSocket connection: {}", e);
                }
            }
        });
    }

    Ok(())
}

async fn handle_websocket_connection(
    client_id: String,
    ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    state: ViewerState,
) {
    let (mut sender, mut receiver) = ws_stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Add client to state
    state.add_client(client_id.clone(), tx).await;

    // Send latest state if available
    {
        let latest_state = state.latest_state.read().await;
        if let Some(payload) = latest_state.as_ref() {
            if let Ok(json) = serde_json::to_string(payload) {
                let _ = state.broadcast_to_clients(Message::Text(json)).await;
            }
        }
    }

    // Handle incoming messages from WebSocket client
    let receive_task = async {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(Message::Text(text)) => {
                    debug!("Received text message from {}: {}", client_id, text);
                    // Handle client commands here
                }
                Ok(Message::Close(_)) => {
                    info!("WebSocket close received from {}", client_id);
                    break;
                }
                Err(e) => {
                    error!("WebSocket error from {}: {}", client_id, e);
                    break;
                }
                _ => {}
            }
        }
    };

    // Handle outgoing messages to WebSocket client
    let send_task = async {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    };

    // Run both tasks concurrently
    tokio::select! {
        _ = receive_task => {},
        _ = send_task => {},
    }

    // Remove client when connection is closed
    state.remove_client(client_id).await;
    info!("WebSocket connection closed");
}