use bevy::prelude::*;
use crate::components::{
    UnitMind, UnitNeedsV2, UnitInventory, GridPosition, WorkProgress, WorkerTag,
};
use crate::ai::{ActionPlan, GoapAction};
use crate::resources::ResourceType;

/// System to update unit's state of mind based on their current activity
pub fn update_unit_mind_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut UnitMind,
            Option<&UnitNeedsV2>,
            Option<&UnitInventory>,
            Option<&GridPosition>,
            Option<&WorkProgress>,
            Option<&ActionPlan>,
        ),
        With<WorkerTag>,
    >,
) {
    for (
        entity,
        mut mind,
        needs,
        inventory,
        position,
        work_progress,
        action_plan,
    ) in query.iter_mut()
    {
        // Determine what the unit should be thinking about
        let new_mind = determine_unit_mind(
            needs,
            inventory,
            position,
            work_progress,
            action_plan,
        );

        // Update if changed
        if !matches!((&*mind, &new_mind), (UnitMind::Custom(a), UnitMind::Custom(b)) if a == b)
            && std::mem::discriminant(&*mind) != std::mem::discriminant(&new_mind)
        {
            *mind = new_mind;
        }
    }
}

fn determine_unit_mind(
    needs: Option<&UnitNeedsV2>,
    inventory: Option<&UnitInventory>,
    position: Option<&GridPosition>,
    work_progress: Option<&WorkProgress>,
    action_plan: Option<&ActionPlan>,
) -> UnitMind {
    // Priority 1: Check if actively working
    if let Some(work) = work_progress {
        if work.is_working {
            return match work.work_type.as_str() {
                "gather_food" => UnitMind::Gathering {
                    resource: "berries".to_string(),
                },
                "gather_wood" => UnitMind::Gathering {
                    resource: "wood".to_string(),
                },
                "gather_stone" => UnitMind::Gathering {
                    resource: "stone".to_string(),
                },
                "building" => UnitMind::Building {
                    structure: "structure".to_string(),
                },
                _ => UnitMind::Working {
                    task: work.work_type.clone(),
                },
            };
        }
    }

    // Priority 2: Check if executing a plan
    if let Some(plan) = action_plan {
        if !plan.actions.is_empty() && plan.current_index < plan.actions.len() {
            let current_action = &plan.actions[plan.current_index];
            return match current_action.name.as_str() {
                "move_to_resource" | "move_to_berries" | "move_to_tree" => {
                    UnitMind::GoingThere {
                        destination: "resource".to_string(),
                    }
                }
                "gather_food" => UnitMind::Gathering {
                    resource: "berries".to_string(),
                },
                "gather_wood" => UnitMind::Gathering {
                    resource: "wood".to_string(),
                },
                "gather_stone" => UnitMind::Gathering {
                    resource: "stone".to_string(),
                },
                "store_resources" => UnitMind::Storing,
                "eat_food" => UnitMind::Eating,
                "rest" => UnitMind::Resting,
                "go_home" => UnitMind::GoingHome,
                _ => UnitMind::Working {
                    task: current_action.name.clone(),
                },
            };
        } else if plan.is_complete() {
            return UnitMind::Thinking; // Plan complete, thinking about next step
        }
    }

    // Priority 3: Check critical needs
    if let Some(needs) = needs {
        // Very hungry - actively looking for food
        if needs.hunger() > 0.7 {
            // Check if we have food
            if let Some(inv) = inventory {
                if inv.has_item(ResourceType::Food, 1) {
                    return UnitMind::Eating;
                }
            }
            return UnitMind::SearchingForFood;
        }

        // Very tired - need rest
        if needs.energy() < 0.2 {
            return UnitMind::Resting;
        }
    }

    // Priority 4: Check if we're moving
    if let Some(pos) = position {
        if pos.is_moving() {
            // Try to determine destination based on context
            if let Some(needs) = needs {
                if needs.hunger() > 0.5 {
                    return UnitMind::SearchingForFood;
                }
            }
            return UnitMind::Wandering;
        }
    }

    // Priority 5: Check if thinking (no plan)
    if action_plan.is_none() {
        return UnitMind::Thinking;
    }

    // Priority 6: Random idle states based on needs
    if let Some(needs) = needs {
        if needs.hunger() > 0.4 && needs.hunger() < 0.7 {
            return UnitMind::LookingAround; // Mildly hungry, looking for opportunities
        }
    }

    // Default state
    UnitMind::Idle
}

/// System to add UnitMind component to entities that don't have it
pub fn ensure_unit_mind_system(
    mut commands: Commands,
    query: Query<Entity, (With<WorkerTag>, Without<UnitMind>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(UnitMind::default());
    }
}

/// System to log mind state changes for debugging
pub fn log_mind_changes_system(
    query: Query<(&crate::components::NameComponent, &UnitMind), Changed<UnitMind>>,
) {
    for (name, mind) in query.iter() {
        debug!(
            "[MIND] {} is now: {}",
            name.0,
            mind.description()
        );
    }
}