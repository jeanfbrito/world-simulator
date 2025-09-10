// Minimal test to verify bevy_dogoap is working
use bevy::prelude::*;
use bevy_dogoap::prelude::*;

// Test datum component
#[derive(Component, Clone, DatumComponent)]
pub struct TestHunger(pub f64);

// Test action component  
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
#[reflect(Component)]
pub struct TestEatAction;

pub fn test_dogoap_setup() {
    // Try to use the generated methods
    let _goal = Goal::from_reqs(&[
        TestHunger::is_less(30.0),
    ]);
    
    let _action = TestEatAction::new()
        .add_mutator(TestHunger::set(100.0));
        
    println!("bevy_dogoap test compiled successfully!");
}