use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents the current state of mind/activity of a unit
/// This is what the unit is currently focused on or thinking about
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub enum UnitMind {
    /// Just standing around, no particular goal
    Idle,
    
    /// Thinking about what to do next
    Thinking,
    
    /// Looking around the area, exploring
    LookingAround,
    
    /// Actively searching for food sources
    SearchingForFood,
    
    /// Walking towards a specific destination
    GoingThere { destination: String },
    
    /// Gathering resources (berries, wood, etc)
    Gathering { resource: String },
    
    /// Working on a specific task
    Working { task: String },
    
    /// Eating food to satisfy hunger
    Eating,
    
    /// Resting to recover energy
    Resting,
    
    /// Fleeing from danger
    Fleeing,
    
    /// Going home for shelter
    GoingHome,
    
    /// Storing items in storage
    Storing,
    
    /// Retrieving items from storage
    Retrieving,
    
    /// Building something
    Building { structure: String },
    
    /// Contemplating life
    ContemplatingLife,
    
    /// Wandering aimlessly
    Wandering,
    
    /// Custom state for special behaviors
    Custom(String),
}

impl Default for UnitMind {
    fn default() -> Self {
        Self::Idle
    }
}

impl UnitMind {
    /// Get a human-readable description of the current state
    pub fn description(&self) -> String {
        match self {
            Self::Idle => "idle".to_string(),
            Self::Thinking => "thinking".to_string(),
            Self::LookingAround => "looking around".to_string(),
            Self::SearchingForFood => "searching for food".to_string(),
            Self::GoingThere { destination } => format!("going to {}", destination),
            Self::Gathering { resource } => format!("gathering {}", resource),
            Self::Working { task } => format!("working on {}", task),
            Self::Eating => "eating".to_string(),
            Self::Resting => "resting".to_string(),
            Self::Fleeing => "fleeing".to_string(),
            Self::GoingHome => "going home".to_string(),
            Self::Storing => "storing items".to_string(),
            Self::Retrieving => "retrieving items".to_string(),
            Self::Building { structure } => format!("building {}", structure),
            Self::ContemplatingLife => "contemplating life".to_string(),
            Self::Wandering => "wandering".to_string(),
            Self::Custom(desc) => desc.clone(),
        }
    }
    
    /// Get a short action name for display
    pub fn action_name(&self) -> &str {
        match self {
            Self::Idle => "idle",
            Self::Thinking => "thinking",
            Self::LookingAround => "exploring",
            Self::SearchingForFood => "searching_food",
            Self::GoingThere { .. } => "walking",
            Self::Gathering { .. } => "gathering",
            Self::Working { .. } => "working",
            Self::Eating => "eating",
            Self::Resting => "resting",
            Self::Fleeing => "fleeing",
            Self::GoingHome => "going_home",
            Self::Storing => "storing",
            Self::Retrieving => "retrieving",
            Self::Building { .. } => "building",
            Self::ContemplatingLife => "contemplating",
            Self::Wandering => "wandering",
            Self::Custom(_) => "custom",
        }
    }
    
    /// Check if the unit is actively doing something (not idle or thinking)
    pub fn is_active(&self) -> bool {
        !matches!(self, Self::Idle | Self::Thinking | Self::LookingAround | Self::ContemplatingLife)
    }
    
    /// Check if the unit is moving
    pub fn is_moving(&self) -> bool {
        matches!(self, 
            Self::GoingThere { .. } | 
            Self::SearchingForFood | 
            Self::Fleeing | 
            Self::GoingHome |
            Self::LookingAround |
            Self::Wandering
        )
    }
}