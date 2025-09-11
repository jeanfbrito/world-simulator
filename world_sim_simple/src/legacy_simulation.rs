use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldGrid {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub tile_type: String,
    pub passable: bool,
    pub resources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: usize,
    pub agent_type: String,
    pub x: usize,
    pub y: usize,
    pub health: f32,
    pub energy: f32,
    pub state: String,
    pub inventory: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LegacySimulationState {
    pub tick: u64,
    pub agents: Vec<Agent>,
    pub events: Vec<SimulationEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationEvent {
    pub tick: u64,
    pub event_type: String,
    pub description: String,
    pub entity_id: Option<usize>,
    pub position: Option<(usize, usize)>,
}

impl Default for WorldGrid {
    fn default() -> Self {
        let width = 64;
        let height = 64;
        let mut tiles = Vec::with_capacity(width * height);

        for _ in 0..width * height {
            tiles.push(Tile {
                tile_type: "grass".to_string(),
                passable: true,
                resources: vec![],
            });
        }

        Self {
            width,
            height,
            tiles,
        }
    }
}
