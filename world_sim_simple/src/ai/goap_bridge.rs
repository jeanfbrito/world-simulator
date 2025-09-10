use bevy::prelude::*;
use crate::ai::{Task, TaskType, TaskPriority, TaskStatus, TaskSystem, GoapAction, ActionPlan};
use crate::debug::{DebugSystem, DebugLevel};

/// Bridges GOAP actions to the existing task system
pub fn goap_to_task_bridge_system(
    mut commands: Commands,
    mut agents: Query<(Entity, &ActionPlan), Changed<ActionPlan>>,
    mut task_system: ResMut<TaskSystem>,
    debug: Res<DebugSystem>,
) {
    for (entity, plan) in agents.iter_mut() {
        if let Some(action) = plan.current_action() {
            debug.log(
                DebugLevel::Info,
                "GOAP_ACTION",
                &format!("🎯 Executing GOAP action: {}", action.name)
            );
            
            // Convert GOAP action to task type and priority
            let (task_type, priority) = match action.name.as_str() {
                "move_to_resource" => (TaskType::Move, TaskPriority::Normal),
                "harvest_resource" => (TaskType::Harvest, TaskPriority::Normal),
                "move_to_storage" => (TaskType::Move, TaskPriority::Normal),
                "store_resources" => (TaskType::Deliver, TaskPriority::Normal),
                "gather_food" => (TaskType::Harvest, TaskPriority::High),
                "eat_food" => (TaskType::Move, TaskPriority::High),
                "rest" => (TaskType::Move, TaskPriority::High),
                "build_structure" => (TaskType::Build, TaskPriority::Normal),
                "build_house" => (TaskType::Build, TaskPriority::High), // High priority for shelter
                "cut_wood" => (TaskType::Harvest, TaskPriority::High), // Need wood for house
                "quarry_stone" => (TaskType::Harvest, TaskPriority::High), // Need stone for house
                "get_wood_from_stockpile" => (TaskType::Move, TaskPriority::Normal),
                "get_stone_from_stockpile" => (TaskType::Move, TaskPriority::Normal),
                _ => {
                    debug.log(
                        DebugLevel::Debug,
                        "GOAP_BRIDGE",
                        &format!("Unknown action: {}", action.name)
                    );
                    continue;
                }
            };
            
            // Create task using TaskSystem
            let mut task = task_system.create_task(task_type);
            task.priority = priority;
            task.target_entity = Some(entity);
            
            // Add task to system
            task_system.add_task(task.clone());
            
            // Also add as component to entity
            commands.entity(entity).insert(task);
            
            debug.log(
                DebugLevel::Info,
                "GOAP_TASK",
                &format!("✓ Created {:?} task (priority: {:?}) from action: {}", task_type, priority, action.name)
            );
        }
    }
}

/// System to update needs over time (hunger, energy)
pub fn update_needs_system(
    mut workers: Query<(
        &mut crate::components::IsHungry,
        &mut crate::components::HasEnergy,
        &crate::components::IsWorking,
        &crate::components::WorkerTag,
    )>,
    time: Res<Time>,
    debug: Res<DebugSystem>,
) {
    let delta = time.delta_secs() as f64;
    
    for (mut hunger, mut energy, is_working, _) in workers.iter_mut() {
        // Increase hunger over time (much slower, realistic rate)
        let hunger_rate = if is_working.0 { 0.005 } else { 0.003 }; // 0.5% per second when working, 0.3% when idle
        hunger.0 = (hunger.0 + delta * hunger_rate).min(1.0);
        
        // Decrease energy over time (slower, more realistic)
        let energy_rate = if is_working.0 { 0.015 } else { 0.005 }; // 1.5% per second when working, 0.5% when idle
        energy.0 = (energy.0 - delta * energy_rate).max(0.0);
        
        // Log critical states
        if hunger.0 > 0.8 {
            debug.log(
                DebugLevel::Info,
                "NEEDS_CRITICAL",
                "⚠️ Worker is very hungry! (>80%)"
            );
        }
        
        if energy.0 < 0.2 {
            debug.log(
                DebugLevel::Info,
                "NEEDS_CRITICAL",
                "⚠️ Worker is exhausted! (<20% energy)"
            );
        }
        
        // Log periodic needs update (every 10 seconds)
        static mut LAST_LOG: f64 = 0.0;
        unsafe {
            if delta > 0.0 && LAST_LOG > 10.0 {
                debug.log(
                    DebugLevel::Debug,
                    "NEEDS_UPDATE",
                    &format!("Worker needs - Hunger: {:.0}%, Energy: {:.0}%",
                        hunger.0 * 100.0, energy.0 * 100.0)
                );
                LAST_LOG = 0.0;
            } else {
                LAST_LOG += delta;
            }
        }
    }
}