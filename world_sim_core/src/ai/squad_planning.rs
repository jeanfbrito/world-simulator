//! Squad-based planning for shared goals and coordinated behavior

use bevy_ecs::prelude::*;
use world_sim_interface::Position;
use bevy_dogoap::prelude::*;
use std::collections::HashMap;
use crate::components::*;

/// Represents a squad of workers with shared goals
#[derive(Component)]
pub struct Squad {
    pub id: u32,
    pub members: Vec<Entity>,
    pub leader: Option<Entity>,
    pub shared_goal: Option<SquadGoal>,
    pub formation: Formation,
    pub morale: f32,
}

/// Squad goals that can be shared
#[derive(Clone, Debug)]
pub enum SquadGoal {
    HarvestArea { center: Position, radius: f32 },
    ConstructBuilding { building_type: world_sim_interface::BuildingType, position: Position },
    DefendLocation { position: Position },
    PatrolRoute { waypoints: Vec<Position> },
    GatherResources { resource_type: world_sim_interface::ResourceType, quota: u32 },
}

/// Formation types for squad movement
#[derive(Clone, Debug)]
pub enum Formation {
    Line,
    Column,
    Circle,
    Wedge,
    Scatter,
}

/// Component for squad members
#[derive(Component)]
pub struct SquadMember {
    pub squad_id: u32,
    pub role: SquadRole,
    pub position_in_formation: usize,
}

#[derive(Clone, Debug)]
pub enum SquadRole {
    Leader,
    Worker,
    Scout,
    Guard,
}

/// Resource managing all squads
#[derive(Resource, Default)]
pub struct SquadManager {
    squads: HashMap<u32, Squad>,
    next_squad_id: u32,
}

impl SquadManager {
    /// Create a new squad
    pub fn create_squad(&mut self, members: Vec<Entity>) -> u32 {
        let squad_id = self.next_squad_id;
        self.next_squad_id += 1;
        
        let leader = members.first().copied();
        
        let squad = Squad {
            id: squad_id,
            members: members.clone(),
            leader,
            shared_goal: None,
            formation: Formation::Scatter,
            morale: 1.0,
        };
        
        self.squads.insert(squad_id, squad);
        squad_id
    }
    
    /// Assign a goal to a squad
    pub fn assign_goal(&mut self, squad_id: u32, goal: SquadGoal) {
        if let Some(squad) = self.squads.get_mut(&squad_id) {
            squad.shared_goal = Some(goal);
        }
    }
    
    /// Get squad by ID
    pub fn get_squad(&self, squad_id: u32) -> Option<&Squad> {
        self.squads.get(&squad_id)
    }
    
    /// Update squad morale based on success/failure
    pub fn update_morale(&mut self, squad_id: u32, delta: f32) {
        if let Some(squad) = self.squads.get_mut(&squad_id) {
            squad.morale = (squad.morale + delta).clamp(0.0, 1.0);
        }
    }
}

/// System that forms squads from nearby workers
pub fn form_squads_system(
    mut commands: Commands,
    mut squad_manager: ResMut<SquadManager>,
    workers: Query<(Entity, &PositionComponent), (With<WorkerComponent>, Without<SquadMember>)>,
    spatial_grid: Res<super::social_alerts::SpatialGrid>,
) {
    let mut processed = std::collections::HashSet::new();
    
    for (entity, pos) in workers.iter() {
        if processed.contains(&entity) {
            continue;
        }
        
        // Find nearby workers without squads
        let nearby = spatial_grid.get_entities_in_range(pos, 20.0);
        let mut squad_members = vec![entity];
        processed.insert(entity);
        
        for nearby_entity in nearby {
            if processed.contains(&nearby_entity) {
                continue;
            }
            
            if let Ok((worker_entity, _)) = workers.get(nearby_entity) {
                squad_members.push(worker_entity);
                processed.insert(worker_entity);
                
                // Limit squad size
                if squad_members.len() >= 5 {
                    break;
                }
            }
        }
        
        // Form squad if we have enough members
        if squad_members.len() >= 3 {
            let squad_id = squad_manager.create_squad(squad_members.clone());
            
            // Assign squad member components
            for (i, member) in squad_members.iter().enumerate() {
                let role = if i == 0 { SquadRole::Leader } else { SquadRole::Worker };
                
                commands.entity(*member).insert(SquadMember {
                    squad_id,
                    role: role.clone(),
                    position_in_formation: i,
                });
                
                // Leader gets shared planner
                if i == 0 {
                    commands.entity(*member).insert(SharedPlanner {
                        squad_id,
                        is_leader: true,
                    });
                }
            }
            
            println!("Formed squad {} with {} members", squad_id, squad_members.len());
        }
    }
}

/// Component for shared GOAP planning
#[derive(Component)]
pub struct SharedPlanner {
    pub squad_id: u32,
    pub is_leader: bool,
}

/// System that shares GOAP plans within squads
pub fn share_squad_plans_system(
    squad_manager: Res<SquadManager>,
    mut planners: Query<(&SharedPlanner, &SquadMember, &mut Planner)>,
) {
    // Leaders share their plans with squad members
    let mut squad_plans: HashMap<u32, Goal> = HashMap::new();
    
    // First pass: collect leader plans
    for (shared, member, planner) in planners.iter() {
        if shared.is_leader {
            if let Some(goal) = &planner.current_goal {
                squad_plans.insert(member.squad_id, goal.clone());
            }
        }
    }
    
    // Second pass: apply plans to members
    for (shared, member, mut planner) in planners.iter_mut() {
        if !shared.is_leader {
            if let Some(squad_goal) = squad_plans.get(&member.squad_id) {
                // Members adopt simplified version of leader's goal
                if planner.current_goal.is_none() {
                    planner.current_goal = Some(squad_goal.clone());
                }
            }
        }
    }
}

/// System for coordinated squad movement
pub fn squad_movement_system(
    squad_manager: Res<SquadManager>,
    mut members: Query<(&SquadMember, &mut MovementComponent, &mut PositionComponent)>,
) {
    for squad in squad_manager.squads.values() {
        if let Some(leader_entity) = squad.leader {
            // Get leader position
            if let Ok((_, _, leader_pos)) = members.get(leader_entity) {
                let leader_pos = leader_pos.clone();
                
                // Move other members in formation
                for (i, member_entity) in squad.members.iter().enumerate() {
                    if *member_entity == leader_entity {
                        continue;
                    }
                    
                    if let Ok((member, mut movement, mut pos)) = members.get_mut(*member_entity) {
                        let offset = calculate_formation_offset(&squad.formation, i, squad.members.len());
                        
                        let target = Position {
                            x: leader_pos.x() + offset.0,
                            y: leader_pos.y() + offset.1,
                        };
                        
                        // Move toward formation position
                        if pos.distance_to(&leader_pos) > 15.0 {
                            // Too far, catch up quickly
                            movement.speed = 7.0;
                            movement.set_target(target);
                        } else if pos.distance_to(&PositionComponent::new(target.x, target.y)) > 2.0 {
                            // Maintain formation
                            movement.speed = 5.0;
                            movement.set_target(target);
                        }
                    }
                }
            }
        }
    }
}

/// Calculate formation offset for a squad member
fn calculate_formation_offset(formation: &Formation, index: usize, total: usize) -> (i32, i32) {
    match formation {
        Formation::Line => {
            let spacing = 3;
            let offset = (index as i32 - total as i32 / 2) * spacing;
            (offset, 0)
        }
        Formation::Column => {
            let spacing = 3;
            (0, index as i32 * spacing)
        }
        Formation::Circle => {
            let angle = (index as f32 / total as f32) * std::f32::consts::TAU;
            let radius = 5.0 + total as f32;
            (
                (angle.cos() * radius) as i32,
                (angle.sin() * radius) as i32,
            )
        }
        Formation::Wedge => {
            let row = (index as f32).sqrt() as i32;
            let col = index as i32 - row * row;
            (col * 3 - row * 3 / 2, row * 3)
        }
        Formation::Scatter => {
            // Random-ish scatter based on index
            let x = ((index * 7) % 10) as i32 - 5;
            let y = ((index * 13) % 10) as i32 - 5;
            (x, y)
        }
    }
}

/// System that handles squad goal execution
pub fn execute_squad_goals_system(
    mut squad_manager: ResMut<SquadManager>,
    mut commands: Commands,
    members: Query<(Entity, &SquadMember, &PositionComponent, &InventoryComponent)>,
) {
    for squad in squad_manager.squads.values_mut() {
        if let Some(goal) = &squad.shared_goal {
            match goal {
                SquadGoal::HarvestArea { center, radius } => {
                    // Check if squad is in harvest area
                    let mut in_area_count = 0;
                    for member_entity in &squad.members {
                        if let Ok((_, _, pos, _)) = members.get(*member_entity) {
                            if pos.distance_to(&PositionComponent::new(center.x, center.y)) <= *radius {
                                in_area_count += 1;
                            }
                        }
                    }
                    
                    // All members in area, start harvesting
                    if in_area_count == squad.members.len() {
                        for member_entity in &squad.members {
                            commands.entity(*member_entity).insert(HarvestWoodAction);
                        }
                    }
                }
                
                SquadGoal::GatherResources { resource_type, quota } => {
                    // Check total gathered
                    let mut total_gathered = 0u32;
                    for member_entity in &squad.members {
                        if let Ok((_, _, _, inventory)) = members.get(*member_entity) {
                            total_gathered += inventory.get_resource_amount(*resource_type);
                        }
                    }
                    
                    if total_gathered >= *quota {
                        // Goal achieved!
                        squad.shared_goal = None;
                        squad.morale += 0.1;
                        println!("Squad {} achieved gathering goal!", squad.id);
                    }
                }
                
                _ => {}
            }
        }
    }
}

/// System that promotes cooperation within squads
pub fn squad_cooperation_system(
    members: Query<(&SquadMember, &IsHungry, &HasFood, &PositionComponent)>,
    mut commands: Commands,
) {
    // Group members by squad
    let mut squads: HashMap<u32, Vec<(Entity, f64, u32, PositionComponent)>> = HashMap::new();
    
    for (member, hunger, food, pos) in members.iter() {
        squads.entry(member.squad_id)
            .or_insert_with(Vec::new)
            .push((Entity::PLACEHOLDER, hunger.0, food.0, pos.clone()));
    }
    
    // Within each squad, share resources
    for (squad_id, members) in squads {
        let total_food: u32 = members.iter().map(|(_, _, food, _)| food).sum();
        let avg_hunger: f64 = members.iter().map(|(_, hunger, _, _)| hunger).sum::<f64>() / members.len() as f64;
        
        // If someone is very hungry and squad has food, share
        for (entity, hunger, food, pos) in &members {
            if *hunger > 70.0 && avg_hunger < 50.0 && total_food > members.len() as u32 {
                // Trigger food sharing
                println!("Squad {} sharing food with hungry member", squad_id);
            }
        }
    }
}