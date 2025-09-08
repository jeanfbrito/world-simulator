//! World state resource

use bevy_ecs::prelude::*;
use world_sim_interface::{WorldConfig, Season, Weather};

/// Global world state
#[derive(Resource, Debug, Clone)]
pub struct WorldState {
    pub config: WorldConfig,
    pub current_tick: u64,
    pub season: Season,
    pub weather: Weather,
    pub game_speed: f32,
    pub entity_counter: u64,
}

impl WorldState {
    pub fn new(config: WorldConfig) -> Self {
        Self {
            config,
            current_tick: 0,
            season: Season::Spring,
            weather: Weather::Clear,
            game_speed: 1.0,
            entity_counter: 1,
        }
    }
    
    pub fn next_entity_id(&mut self) -> u64 {
        let id = self.entity_counter;
        self.entity_counter += 1;
        id
    }
    
    pub fn advance_tick(&mut self) {
        self.current_tick += 1;
        
        // Update season every 100 ticks
        if self.config.seasons_enabled && self.current_tick % 100 == 0 {
            self.season = match self.season {
                Season::Spring => Season::Summer,
                Season::Summer => Season::Autumn,
                Season::Autumn => Season::Winter,
                Season::Winter => Season::Spring,
            };
        }
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new(WorldConfig::default())
    }
}