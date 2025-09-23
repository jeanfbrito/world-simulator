use bevy::prelude::*;
use crate::components::{
    UnitMind, UnitInventory, GridPosition, GridMovement, WorkProgress, UnitTag,
    NameComponent,
};
use crate::resources::ResourceType;

/// System to update unit's state of mind based on their current activity
pub fn update_unit_mind_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut UnitMind,
            Option<&crate::ai::bevy_dogoap_impl::Satiety>,
            Option<&UnitInventory>,
            Option<&GridPosition>,
            Option<&GridMovement>,
            Option<&WorkProgress>,
            Option<&NameComponent>,
        ),
        With<UnitTag>,
    >,
) {
    for (
        entity,
        mut mind,
        needs,
        inventory,
        position,
        movement,
        work_progress,
        name,
    ) in query.iter_mut()
    {
        // Determine what the unit should be thinking about
        let new_mind = determine_unit_mind(
            needs,
            inventory,
            position,
            movement,
            work_progress,
            );

        // Always update mind state to ensure it's current
        // (previously we were only updating on discriminant change which missed state updates)
        if *mind != new_mind {
            // Debug log the state change
            if let Some(n) = name {
                println!(
                    "[MIND] {} changed from {} to {}",
                    n.name,
                    mind.description(),
                    new_mind.description()
                );
            }
            *mind = new_mind;
        }
    }
}

fn determine_unit_mind(
    needs: Option<&crate::ai::bevy_dogoap_impl::Satiety>,
    inventory: Option<&UnitInventory>,
    position: Option<&GridPosition>,
    movement: Option<&GridMovement>,
    work_progress: Option<&WorkProgress>,
) -> UnitMind {
    // Priority 1: Check if unit is working on something
    // (Since WorkProgress detection seems to not be working properly)
    
    // Priority 1: Check if actively working
    if let Some(work) = work_progress {
        if work.is_working || (work.work_type.is_some() && work.progress() > 0.0) {
            if let Some(work_type) = &work.work_type {
                use crate::components::WorkType;
                return match work_type {
                    WorkType::Gathering(res) => UnitMind::Gathering {
                        resource: format!("{:?}", res.resource_type).to_lowercase(),
                    },
                    WorkType::Building(_) => UnitMind::Building {
                        structure: "structure".to_string(),
                    },
                    _ => UnitMind::Working {
                        task: format!("{:?}", work_type).to_lowercase(),
                    },
                };
            }
        }
    }

    
    // Priority 2: Check critical needs
    if let Some(needs) = needs {
        // Very hungry - actively looking for food (satiety < 30 = hunger > 0.7 in old system)
        if needs.0 < 30.0 {
            // Check if we have food
            if let Some(inv) = inventory {
                if inv.has_item(ResourceType::Berries, 1) {
                    return UnitMind::Eating;
                }
            }
            return UnitMind::SearchingForFood;
        }

        // Very tired - but energy recovers when idle now
        // No need for explicit resting state
    }

    // Priority 3: Check if we're moving
    if let Some(move_comp) = movement {
        if move_comp.is_moving {
            // Try to determine destination based on context
            if let Some(needs) = needs {
                if needs.0 < 50.0 {  // satiety < 50 = hunger > 0.5 in old system
                    return UnitMind::SearchingForFood;
                }
            }
            return UnitMind::Wandering;
        }
    }


    // Priority 4: Random idle states based on needs
    if let Some(needs) = needs {
        if needs.0 >= 30.0 && needs.0 < 60.0 {  // satiety 30-60 = hunger 0.4-0.7 in old system
            return UnitMind::LookingAround; // Mildly hungry, looking for opportunities
        }
    }

    // Default state
    UnitMind::Idle
}

/// System to add UnitMind component to entities that don't have it
pub fn ensure_unit_mind_system(
    mut commands: Commands,
    query: Query<Entity, (With<UnitTag>, Without<UnitMind>)>,
) {
    for entity in query.iter() {
        println!("[MIND] Adding UnitMind component to entity {:?}", entity);
        commands.entity(entity).insert(UnitMind::default());
    }
}


/// System to log mind state changes for debugging
pub fn log_mind_changes_system(
    query: Query<(&crate::components::NameComponent, &UnitMind), Changed<UnitMind>>,
) {
    for (name, mind) in query.iter() {
        println!(
            "[MIND] {} is now: {}",
            name.name,
            mind.description()
        );
    }
}