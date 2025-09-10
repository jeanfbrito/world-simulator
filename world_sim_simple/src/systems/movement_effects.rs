/// System to update movement effects based on unit status
/// 
/// This system updates movement speed modifiers based on
/// energy levels, inventory weight, injuries, etc.

use bevy::prelude::*;
use crate::components::{
    MovementEffects, MovementSpeed, UnitNeedsV2, UnitInventory,
    PeasantTag, NameComponent
};
use crate::SimulationState;

/// Updates movement effects based on unit needs and inventory
pub fn update_movement_effects_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(
        &mut MovementEffects,
        &UnitNeedsV2,
        &UnitInventory,
        &NameComponent,
    ), With<PeasantTag>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    // Only update on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    // Only update every 10 ticks to reduce computation
    if sim_state.tick % 10 != 0 {
        return;
    }
    
    for (mut effects, needs, inventory, name) in units.iter_mut() {
        let old_modifier = effects.get_total_modifier();
        
        // Update exhaustion based on energy
        effects.update_exhaustion(needs.energy());
        
        // Update encumbrance based on inventory weight
        let weight_ratio = inventory.current_weight / inventory.max_weight;
        effects.update_encumbrance(weight_ratio);
        
        // Apply additional effects based on conditions
        if needs.is_starving() {
            effects.slowed = Some(0.7);  // 70% speed when starving
        } else {
            effects.slowed = None;
        }
        
        let new_modifier = effects.get_total_modifier();
        
        // Log significant speed changes
        if (old_modifier - new_modifier).abs() > 0.1 {
            debug.log(
                DebugLevel::Debug,
                "MOVEMENT",
                &format!("{} movement speed changed: {:.0}% → {:.0}%",
                    name.name,
                    old_modifier * 100.0,
                    new_modifier * 100.0
                )
            );
        }
    }
}

/// System to add movement components to units that don't have them
pub fn add_movement_components_system(
    mut commands: Commands,
    query: Query<Entity, (With<PeasantTag>, Without<MovementSpeed>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert((
            MovementSpeed::default(),
            MovementEffects::default(),
        ));
        
        println!("Added movement components to entity {:?}", entity);
    }
}

/// System to configure different movement speeds for different unit types
pub fn configure_unit_speeds_system(
    mut commands: Commands,
    query: Query<(Entity, &NameComponent), Added<MovementSpeed>>,
) {
    for (entity, name) in query.iter() {
        // Example: Make some units faster or slower based on their ID
        let speed = if name.name.contains("1") {
            MovementSpeed::fast()  // Peasant 1 is a scout
        } else if name.name.contains("5") {
            MovementSpeed::slow()  // Peasant 5 carries heavy equipment
        } else {
            MovementSpeed::default()
        };
        
        commands.entity(entity).insert(speed);
        
        println!("Configured movement speed for {}", name.name);
    }
}