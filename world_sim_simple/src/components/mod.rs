use bevy::prelude::*;
use colored::*;

pub mod position;
pub mod health;

pub use position::PositionComponent;
pub use health::HealthComponent;

/// Plugin to register all components
pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        println!("{}", "[COMPONENTS] Registering component systems...".green());
        
        app.register_type::<PositionComponent>()
           .register_type::<HealthComponent>();
           
        println!("{}", "[COMPONENTS] ✓ Position component registered".green());
        println!("{}", "[COMPONENTS] ✓ Health component registered".green());
    }
}