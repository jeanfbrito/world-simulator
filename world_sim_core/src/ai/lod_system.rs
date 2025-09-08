//! Level of Detail (LOD) AI system - reduces complexity based on importance/distance

use bevy_ecs::prelude::*;
use crate::components::*;

/// AI complexity level based on importance
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum AIComplexity {
    /// Full hybrid AI - GOAP + Utility + all features
    Full,
    /// Utility AI only - reactive behaviors  
    Reactive,
    /// Simple scripted behaviors
    Simple,
    /// Minimal processing - basic needs only
    Minimal,
    /// Dormant - almost no processing
    Dormant,
}

/// Component that tracks entity importance for LOD
#[derive(Component)]
pub struct LODComponent {
    pub complexity: AIComplexity,
    pub importance_score: f32,
    pub distance_to_player: f32,
    pub last_full_update: f32,
    pub update_frequency: f32, // How often to update (seconds)
}

impl Default for LODComponent {
    fn default() -> Self {
        Self {
            complexity: AIComplexity::Simple,
            importance_score: 0.5,
            distance_to_player: 100.0,
            last_full_update: 0.0,
            update_frequency: 1.0,
        }
    }
}

/// Calculate importance score for an entity
pub fn calculate_importance(
    distance_to_player: f32,
    is_in_combat: bool,
    is_player_ally: bool,
    has_valuable_items: bool,
    is_doing_important_task: bool,
) -> f32 {
    let mut score = 0.0;
    
    // Distance is primary factor (inverse)
    if distance_to_player < 20.0 {
        score += 1.0;
    } else if distance_to_player < 50.0 {
        score += 0.7;
    } else if distance_to_player < 100.0 {
        score += 0.4;
    } else if distance_to_player < 200.0 {
        score += 0.2;
    }
    
    // Modifiers
    if is_in_combat { score += 0.5; }
    if is_player_ally { score += 0.3; }
    if has_valuable_items { score += 0.2; }
    if is_doing_important_task { score += 0.2; }
    
    score.min(1.0)
}

/// System that updates LOD levels based on importance
pub fn update_lod_system(
    mut query: Query<(
        Entity,
        &mut LODComponent,
        &PositionComponent,
        Option<&super::priority_queue::InCombat>,
        Option<&PlayerAlly>,
        Option<&InventoryComponent>,
        Option<&WorkerComponent>,
    )>,
    player: Query<&PositionComponent, With<PlayerControlled>>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();
    
    // Get player position (if exists)
    let player_pos = player.get_single().ok();
    
    for (entity, mut lod, pos, combat, ally, inventory, worker) in query.iter_mut() {
        // Skip if not time to update
        if current_time - lod.last_full_update < lod.update_frequency {
            continue;
        }
        
        // Calculate distance to player
        lod.distance_to_player = if let Some(p_pos) = player_pos {
            pos.distance_to(p_pos)
        } else {
            100.0 // Default medium distance
        };
        
        // Calculate importance
        let has_valuable = inventory.map(|inv| inv.total_value() > 50).unwrap_or(false);
        let doing_important = worker.map(|w| 
            matches!(w.state, world_sim_interface::WorkerState::Working)
        ).unwrap_or(false);
        
        lod.importance_score = calculate_importance(
            lod.distance_to_player,
            combat.is_some(),
            ally.is_some(),
            has_valuable,
            doing_important,
        );
        
        // Determine complexity level
        let new_complexity = match lod.importance_score {
            s if s > 0.8 => AIComplexity::Full,
            s if s > 0.6 => AIComplexity::Reactive,
            s if s > 0.4 => AIComplexity::Simple,
            s if s > 0.2 => AIComplexity::Minimal,
            _ => AIComplexity::Dormant,
        };
        
        // Update if changed
        if new_complexity != lod.complexity {
            lod.complexity = new_complexity;
            update_ai_components(entity, new_complexity, &mut query.commands());
        }
        
        // Set update frequency based on complexity
        lod.update_frequency = match lod.complexity {
            AIComplexity::Full => 0.1,      // 10 updates/sec
            AIComplexity::Reactive => 0.25,  // 4 updates/sec
            AIComplexity::Simple => 0.5,     // 2 updates/sec
            AIComplexity::Minimal => 1.0,    // 1 update/sec
            AIComplexity::Dormant => 5.0,    // 1 update/5 sec
        };
        
        lod.last_full_update = current_time;
    }
}

/// Update entity components based on LOD level
fn update_ai_components(entity: Entity, complexity: AIComplexity, commands: &mut Commands) {
    match complexity {
        AIComplexity::Full => {
            // Enable all AI systems
            commands.entity(entity)
                .insert(EnableGOAP)
                .insert(EnableUtilityAI)
                .insert(EnablePathfinding)
                .remove::<DisableAI>();
        }
        AIComplexity::Reactive => {
            // Utility AI only
            commands.entity(entity)
                .remove::<EnableGOAP>()
                .insert(EnableUtilityAI)
                .insert(EnablePathfinding)
                .remove::<DisableAI>();
        }
        AIComplexity::Simple => {
            // Basic behaviors only
            commands.entity(entity)
                .remove::<EnableGOAP>()
                .remove::<EnableUtilityAI>()
                .insert(EnableSimpleAI)
                .remove::<DisableAI>();
        }
        AIComplexity::Minimal => {
            // Survival needs only
            commands.entity(entity)
                .remove::<EnableGOAP>()
                .remove::<EnableUtilityAI>()
                .remove::<EnablePathfinding>()
                .insert(EnableMinimalAI);
        }
        AIComplexity::Dormant => {
            // Almost no processing
            commands.entity(entity)
                .insert(DisableAI)
                .remove::<EnableGOAP>()
                .remove::<EnableUtilityAI>()
                .remove::<EnablePathfinding>();
        }
    }
}

/// Marker components for AI system enabling
#[derive(Component)]
pub struct EnableGOAP;

#[derive(Component)]
pub struct EnableUtilityAI;

#[derive(Component)]
pub struct EnablePathfinding;

#[derive(Component)]
pub struct EnableSimpleAI;

#[derive(Component)]
pub struct EnableMinimalAI;

#[derive(Component)]
pub struct DisableAI;

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct PlayerAlly;

/// Staggered update system - spreads updates across frames
pub fn staggered_update_system(
    query: Query<(Entity, &LODComponent)>,
    frame_count: Res<FrameCount>,
) {
    let frame = frame_count.0;
    
    for (entity, lod) in query.iter() {
        // Determine update group based on entity ID
        let update_group = entity.index() % 10;
        
        // Update frequency based on LOD
        let should_update = match lod.complexity {
            AIComplexity::Full => true, // Every frame
            AIComplexity::Reactive => frame % 2 == update_group % 2,
            AIComplexity::Simple => frame % 5 == update_group % 5,
            AIComplexity::Minimal => frame % 10 == update_group % 10,
            AIComplexity::Dormant => frame % 50 == update_group % 50,
        };
        
        if should_update {
            // Entity is scheduled for update this frame
            // Actual update happens in respective AI systems
        }
    }
}

/// Resource tracking frame count
#[derive(Resource, Default)]
pub struct FrameCount(pub u64);

/// System to increment frame counter
pub fn increment_frame_count(mut frame_count: ResMut<FrameCount>) {
    frame_count.0 = frame_count.0.wrapping_add(1);
}

/// Extension methods for InventoryComponent
impl InventoryComponent {
    pub fn total_value(&self) -> u32 {
        self.resources.iter()
            .map(|(resource_type, amount)| {
                let value_per_unit = match resource_type {
                    world_sim_interface::ResourceType::Wood => 2,
                    world_sim_interface::ResourceType::Food => 3,
                    world_sim_interface::ResourceType::Stone => 4,
                    _ => 1,
                };
                amount * value_per_unit
            })
            .sum()
    }
}