//! Game configuration resource

use bevy_ecs::prelude::*;
use std::time::Duration;

/// Game configuration settings
#[derive(Resource, Debug, Clone)]
pub struct GameConfig {
    pub tick_rate: Duration,
    pub harvest_time: Duration,
    pub build_time: Duration,
    pub worker_speed: f32,
    pub resource_regeneration_rate: f32,
    pub max_entities: usize,
    pub autosave_interval: Option<Duration>,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            tick_rate: Duration::from_millis(100),
            harvest_time: Duration::from_secs(3),
            build_time: Duration::from_secs(10),
            worker_speed: 1.0,
            resource_regeneration_rate: 0.1,
            max_entities: 10000,
            autosave_interval: Some(Duration::from_secs(300)),
        }
    }
}