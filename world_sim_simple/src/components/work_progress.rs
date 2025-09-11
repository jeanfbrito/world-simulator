/// Tick-based work progress system
/// 
/// This module implements work progress using integer counters,
/// following the tick-based simulation architecture. All work
/// (building, crafting, gathering) uses the same counter system.

use bevy::prelude::*;
use crate::simulation::*;

/// Work progress tracked with tick-based counters
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct WorkProgress {
    /// Current work type being performed
    pub work_type: Option<WorkType>,
    
    /// Progress counter (0 to MAX_WORK_PROGRESS)
    pub progress_counter: u32,
    
    /// Total ticks required to complete current work
    pub required_ticks: u32,
    
    /// Work speed modifier (1.0 = normal, 2.0 = twice as fast)
    pub speed_modifier: f32,
    
    /// Target entity we're working on (building, resource node, etc.)
    pub target_entity: Option<Entity>,
    
    /// Whether work is currently active
    pub is_working: bool,
    
    /// Whether work can be interrupted
    pub interruptible: bool,
}

impl WorkProgress {
    pub fn new() -> Self {
        Self {
            work_type: None,
            progress_counter: 0,
            required_ticks: 0,
            speed_modifier: 1.0,
            target_entity: None,
            is_working: false,
            interruptible: true,
        }
    }
    
    /// Start a new work task
    pub fn start_work(
        &mut self, 
        work_type: WorkType, 
        required_ticks: u32,
        target: Option<Entity>
    ) {
        self.work_type = Some(work_type);
        self.progress_counter = 0;
        self.required_ticks = required_ticks;
        self.target_entity = target;
        self.is_working = true;
    }
    
    /// Update work progress (called on ticks)
    /// Returns true if work was completed this tick
    pub fn tick_update(&mut self) -> bool {
        if !self.is_working || self.required_ticks == 0 {
            return false;
        }
        
        // Calculate progress increment based on speed modifier
        let base_increment = MAX_WORK_PROGRESS / self.required_ticks.max(1);
        let actual_increment = (base_increment as f32 * self.speed_modifier) as u32;
        
        self.progress_counter += actual_increment;
        
        // Check if work is complete
        if self.progress_counter >= MAX_WORK_PROGRESS {
            // Don't clear work_type yet - the work system needs to read it first!
            // Just mark as not working
            self.is_working = false;
            return true;
        }
        
        false
    }
    
    /// Complete the current work
    pub fn complete_work(&mut self) {
        self.work_type = None;
        self.progress_counter = 0;
        self.required_ticks = 0;
        self.target_entity = None;
        self.is_working = false;
    }
    
    /// Cancel current work
    pub fn cancel_work(&mut self) {
        if self.interruptible {
            self.complete_work();
        }
    }
    
    /// Get progress as a float (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        if self.required_ticks == 0 {
            return 0.0;
        }
        (self.progress_counter as f32 / MAX_WORK_PROGRESS as f32).clamp(0.0, 1.0)
    }
    
    /// Get remaining ticks
    pub fn remaining_ticks(&self) -> u32 {
        if !self.is_working {
            return 0;
        }
        
        let progress_per_tick = MAX_WORK_PROGRESS / self.required_ticks.max(1);
        let remaining_progress = MAX_WORK_PROGRESS.saturating_sub(self.progress_counter);
        
        (remaining_progress / progress_per_tick.max(1)) + 1
    }
    
    /// Apply a temporary speed boost
    pub fn apply_speed_modifier(&mut self, modifier: f32) {
        self.speed_modifier *= modifier;
    }
    
    /// Reset speed modifier
    pub fn reset_speed_modifier(&mut self) {
        self.speed_modifier = 1.0;
    }
}

/// Types of work that can be performed
#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum WorkType {
    /// Gathering resources from the environment
    Gathering(ResourceWork),
    
    /// Constructing buildings
    Building(BuildingWork),
    
    /// Crafting items
    Crafting(CraftingWork),
    
    /// Farming/harvesting
    Farming(FarmingWork),
    
    /// Mining underground
    Mining(MiningWork),
    
    /// Research/learning
    Research(ResearchWork),
    
    /// Repairing damaged structures
    Repair(RepairWork),
    
    /// Generic work
    Generic(String),
}

/// Resource gathering work details
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct ResourceWork {
    pub resource_type: crate::resources::ResourceType,
    pub amount: u32,
    pub tool_bonus: f32,
}

/// Building construction work details
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct BuildingWork {
    pub building_type: String, // TODO: Replace with BuildingType when available
    pub construction_phase: ConstructionPhase,
    pub material_delivered: bool,
}

/// Construction phases for buildings
#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum ConstructionPhase {
    Foundation,
    Framework,
    Walls,
    Roof,
    Interior,
    Finishing,
}

/// Crafting work details
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct CraftingWork {
    pub recipe_id: String,
    pub output_count: u32,
    pub quality_modifier: f32,
}

/// Farming work details
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct FarmingWork {
    pub action: FarmAction,
    pub crop_type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum FarmAction {
    Plowing,
    Planting,
    Watering,
    Weeding,
    Harvesting,
}

/// Mining work details
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct MiningWork {
    pub depth: u32,
    pub ore_type: Option<String>,
    pub tunnel_direction: Option<Direction>,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

/// Research work details
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct ResearchWork {
    pub technology: String,
    pub research_points: u32,
}

/// Repair work details
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct RepairWork {
    pub damage_amount: f32,
    pub materials_needed: Vec<(crate::resources::ResourceType, u32)>,
}

/// Work speed configuration for different work types
#[derive(Component, Clone, Debug, Reflect)]
pub struct WorkSpeed {
    /// Base ticks for gathering (affected by tool quality)
    pub gathering_ticks: u32,
    
    /// Base ticks for building (affected by skill)
    pub building_ticks: u32,
    
    /// Base ticks for crafting (affected by workshop quality)
    pub crafting_ticks: u32,
    
    /// Base ticks for farming
    pub farming_ticks: u32,
    
    /// Base ticks for mining
    pub mining_ticks: u32,
    
    /// Global work speed modifier
    pub global_modifier: f32,
}

impl Default for WorkSpeed {
    fn default() -> Self {
        Self {
            gathering_ticks: GATHER_TICKS_BASE,
            building_ticks: BUILD_TICKS_BASE,
            crafting_ticks: CRAFT_TICKS_BASE,
            farming_ticks: FARM_TICKS_BASE,
            mining_ticks: MINE_TICKS_BASE,
            global_modifier: 1.0,
        }
    }
}

impl WorkSpeed {
    /// Get ticks required for a specific work type
    pub fn get_ticks_for(&self, work_type: &WorkType) -> u32 {
        let base_ticks = match work_type {
            WorkType::Gathering(_) => self.gathering_ticks,
            WorkType::Building(_) => self.building_ticks,
            WorkType::Crafting(_) => self.crafting_ticks,
            WorkType::Farming(_) => self.farming_ticks,
            WorkType::Mining(_) => self.mining_ticks,
            WorkType::Research(_) => 100, // Fixed for research
            WorkType::Repair(_) => 50,    // Fixed for repairs
            WorkType::Generic(_) => 30,   // Default for generic
        };
        
        ((base_ticks as f32) / self.global_modifier.max(0.1)) as u32
    }
    
    /// Apply skill-based modifier
    pub fn apply_skill_modifier(&mut self, skill_level: f32) {
        // Higher skill = faster work
        self.global_modifier = 1.0 + (skill_level - 1.0) * 0.1;
    }
}

// Work tick constants (base values)
pub const GATHER_TICKS_BASE: u32 = 10;   // 1 second at 10 TPS
pub const BUILD_TICKS_BASE: u32 = 50;    // 5 seconds
pub const CRAFT_TICKS_BASE: u32 = 30;    // 3 seconds
pub const FARM_TICKS_BASE: u32 = 20;     // 2 seconds
pub const MINE_TICKS_BASE: u32 = 40;     // 4 seconds

/// Work queue for managing multiple tasks
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct WorkQueue {
    /// Queue of work to be done
    pub tasks: Vec<QueuedWork>,
    
    /// Whether to loop the queue
    pub repeat: bool,
    
    /// Maximum queue size
    pub max_size: usize,
}

#[derive(Clone, Debug, Reflect)]
pub struct QueuedWork {
    pub work_type: WorkType,
    pub required_ticks: u32,
    pub target_entity: Option<Entity>,
    pub priority: i32,
}

impl WorkQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            tasks: Vec::new(),
            repeat: false,
            max_size,
        }
    }
    
    /// Add work to the queue
    pub fn enqueue(&mut self, work: QueuedWork) -> bool {
        if self.tasks.len() >= self.max_size {
            return false;
        }
        
        self.tasks.push(work);
        self.tasks.sort_by_key(|w| -w.priority); // Higher priority first
        true
    }
    
    /// Get the next work item
    pub fn dequeue(&mut self) -> Option<QueuedWork> {
        if self.tasks.is_empty() {
            return None;
        }
        
        let work = self.tasks.remove(0);
        
        if self.repeat {
            // Re-add to end of queue
            self.tasks.push(work.clone());
        }
        
        Some(work)
    }
    
    /// Clear all queued work
    pub fn clear(&mut self) {
        self.tasks.clear();
    }
    
    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}