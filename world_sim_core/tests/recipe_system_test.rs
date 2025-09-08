//! Test recipe processing functionality
//! This test MUST fail first (TDD)

use world_sim_core::SimulationEngine;
use world_sim_interface::{
    WorldConfig, EntityType, Position, EngineCommand,
    BuildingType, ResourceType, Recipe, RecipeId
};
use std::collections::HashMap;

#[test]
fn test_recipe_registration() {
    let mut engine = SimulationEngine::new();
    
    // Register custom recipe
    let plank_recipe = Recipe {
        id: RecipeId::from("wood_to_planks"),
        name: "Wood to Planks".to_string(),
        inputs: vec![(ResourceType::Wood, 2)].into_iter().collect(),
        outputs: vec![(ResourceType::Planks, 3)].into_iter().collect(),
        duration_ticks: 5,
        required_building: Some(BuildingType::Sawmill),
    };
    
    engine.register_recipe(plank_recipe.clone());
    
    let recipes = engine.get_recipes();
    assert!(recipes.contains(&plank_recipe), "Recipe should be registered");
}

#[test]
fn test_recipe_execution_in_building() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 30,
        height: 30,
        starting_workers: 2,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    // Register recipe
    let bread_recipe = Recipe {
        id: RecipeId::from("wheat_to_bread"),
        name: "Bake Bread".to_string(),
        inputs: vec![(ResourceType::Wheat, 3)].into_iter().collect(),
        outputs: vec![(ResourceType::Food, 5)].into_iter().collect(),
        duration_ticks: 10,
        required_building: Some(BuildingType::Bakery),
    };
    
    engine.register_recipe(bread_recipe.clone());
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Build bakery
    let give_resources = EngineCommand::GiveResources {
        entity_id: worker.id,
        resources: vec![
            (ResourceType::Wood, 10),
            (ResourceType::Stone, 5),
        ].into_iter().collect(),
    };
    engine.execute_command(give_resources);
    
    let build_cmd = EngineCommand::Build {
        builder_id: worker.id,
        building_type: BuildingType::Bakery,
        position: Position::new(15, 15),
    };
    engine.execute_command(build_cmd);
    
    // Let construction complete
    for _ in 0..20 {
        engine.tick();
    }
    
    let snapshot = engine.snapshot();
    let bakery = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Building(BuildingType::Bakery)))
        .expect("Bakery should exist");
    
    // Give wheat to bakery
    let give_wheat = EngineCommand::GiveResources {
        entity_id: bakery.id,
        resources: vec![(ResourceType::Wheat, 6)].into_iter().collect(),
    };
    engine.execute_command(give_wheat);
    
    // Start recipe
    let start_recipe = EngineCommand::StartRecipe {
        building_id: bakery.id,
        recipe_id: bread_recipe.id.clone(),
    };
    
    let result = engine.execute_command(start_recipe);
    assert!(result.success, "Should start recipe with resources");
    
    // Let recipe complete
    for _ in 0..15 {
        engine.tick();
    }
    
    // Check bread was produced
    let snapshot = engine.snapshot();
    let bakery_after = snapshot.entities
        .iter()
        .find(|e| e.id == bakery.id)
        .unwrap();
    
    let output = bakery_after.components
        .get("output")
        .expect("Bakery should have output");
    
    let food_count: u32 = output
        .get("food")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    
    assert_eq!(food_count, 5, "Should produce 5 food from recipe");
}

#[test]
fn test_recipe_requires_inputs() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig::default();
    engine.new_world(config).unwrap();
    
    // Register recipe
    let tool_recipe = Recipe {
        id: RecipeId::from("make_tools"),
        name: "Make Tools".to_string(),
        inputs: vec![
            (ResourceType::Wood, 1),
            (ResourceType::Stone, 2),
        ].into_iter().collect(),
        outputs: vec![(ResourceType::Tools, 1)].into_iter().collect(),
        duration_ticks: 8,
        required_building: Some(BuildingType::Workshop),
    };
    
    engine.register_recipe(tool_recipe.clone());
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Build workshop
    let give_resources = EngineCommand::GiveResources {
        entity_id: worker.id,
        resources: vec![(ResourceType::Wood, 20)].into_iter().collect(),
    };
    engine.execute_command(give_resources);
    
    let build_cmd = EngineCommand::Build {
        builder_id: worker.id,
        building_type: BuildingType::Workshop,
        position: Position::new(10, 10),
    };
    engine.execute_command(build_cmd);
    
    // Let construction complete
    for _ in 0..20 {
        engine.tick();
    }
    
    let snapshot = engine.snapshot();
    let workshop = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Building(BuildingType::Workshop)))
        .expect("Workshop should exist");
    
    // Try to start recipe without inputs
    let start_recipe = EngineCommand::StartRecipe {
        building_id: workshop.id,
        recipe_id: tool_recipe.id.clone(),
    };
    
    let result = engine.execute_command(start_recipe);
    assert!(!result.success, "Should fail without required inputs");
    
    // Give partial inputs
    let give_partial = EngineCommand::GiveResources {
        entity_id: workshop.id,
        resources: vec![(ResourceType::Wood, 1)].into_iter().collect(),
    };
    engine.execute_command(give_partial);
    
    let start_recipe2 = EngineCommand::StartRecipe {
        building_id: workshop.id,
        recipe_id: tool_recipe.id.clone(),
    };
    
    let result2 = engine.execute_command(start_recipe2);
    assert!(!result2.success, "Should fail with partial inputs");
    
    // Give remaining inputs
    let give_remaining = EngineCommand::GiveResources {
        entity_id: workshop.id,
        resources: vec![(ResourceType::Stone, 2)].into_iter().collect(),
    };
    engine.execute_command(give_remaining);
    
    let start_recipe3 = EngineCommand::StartRecipe {
        building_id: workshop.id,
        recipe_id: tool_recipe.id.clone(),
    };
    
    let result3 = engine.execute_command(start_recipe3);
    assert!(result3.success, "Should succeed with all inputs");
}

#[test]
fn test_recipe_queue() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 20,
        height: 20,
        starting_workers: 1,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    // Register quick recipe
    let quick_recipe = Recipe {
        id: RecipeId::from("quick_craft"),
        name: "Quick Craft".to_string(),
        inputs: vec![(ResourceType::Wood, 1)].into_iter().collect(),
        outputs: vec![(ResourceType::Planks, 1)].into_iter().collect(),
        duration_ticks: 2,
        required_building: Some(BuildingType::Workshop),
    };
    
    engine.register_recipe(quick_recipe.clone());
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Build workshop
    let give_resources = EngineCommand::GiveResources {
        entity_id: worker.id,
        resources: vec![(ResourceType::Wood, 30)].into_iter().collect(),
    };
    engine.execute_command(give_resources);
    
    let build_cmd = EngineCommand::Build {
        builder_id: worker.id,
        building_type: BuildingType::Workshop,
        position: Position::new(10, 10),
    };
    engine.execute_command(build_cmd);
    
    // Let construction complete
    for _ in 0..15 {
        engine.tick();
    }
    
    let snapshot = engine.snapshot();
    let workshop = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Building(BuildingType::Workshop)))
        .expect("Workshop should exist");
    
    // Give resources for multiple recipes
    let give_wood = EngineCommand::GiveResources {
        entity_id: workshop.id,
        resources: vec![(ResourceType::Wood, 5)].into_iter().collect(),
    };
    engine.execute_command(give_wood);
    
    // Queue multiple recipes
    for _ in 0..3 {
        let queue_recipe = EngineCommand::StartRecipe {
            building_id: workshop.id,
            recipe_id: quick_recipe.id.clone(),
        };
        engine.execute_command(queue_recipe);
    }
    
    // Let recipes process
    for _ in 0..10 {
        engine.tick();
    }
    
    // Check all recipes completed
    let snapshot = engine.snapshot();
    let workshop_after = snapshot.entities
        .iter()
        .find(|e| e.id == workshop.id)
        .unwrap();
    
    let output = workshop_after.components
        .get("output")
        .expect("Workshop should have output");
    
    let planks: u32 = output
        .get("planks")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    
    assert_eq!(planks, 3, "Should have processed 3 recipes");
}

#[test]
fn test_recipe_with_multiple_outputs() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig::default();
    engine.new_world(config).unwrap();
    
    // Register recipe with multiple outputs
    let butcher_recipe = Recipe {
        id: RecipeId::from("butcher_animal"),
        name: "Butcher Animal".to_string(),
        inputs: vec![(ResourceType::Livestock, 1)].into_iter().collect(),
        outputs: vec![
            (ResourceType::Food, 10),
            (ResourceType::Leather, 2),
        ].into_iter().collect(),
        duration_ticks: 5,
        required_building: Some(BuildingType::Butcher),
    };
    
    engine.register_recipe(butcher_recipe.clone());
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Build butcher shop
    let give_resources = EngineCommand::GiveResources {
        entity_id: worker.id,
        resources: vec![
            (ResourceType::Wood, 15),
            (ResourceType::Stone, 8),
        ].into_iter().collect(),
    };
    engine.execute_command(give_resources);
    
    let build_cmd = EngineCommand::Build {
        builder_id: worker.id,
        building_type: BuildingType::Butcher,
        position: Position::new(12, 12),
    };
    engine.execute_command(build_cmd);
    
    // Let construction complete
    for _ in 0..20 {
        engine.tick();
    }
    
    let snapshot = engine.snapshot();
    let butcher = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Building(BuildingType::Butcher)))
        .expect("Butcher should exist");
    
    // Give livestock
    let give_livestock = EngineCommand::GiveResources {
        entity_id: butcher.id,
        resources: vec![(ResourceType::Livestock, 2)].into_iter().collect(),
    };
    engine.execute_command(give_livestock);
    
    // Process recipe
    let start_recipe = EngineCommand::StartRecipe {
        building_id: butcher.id,
        recipe_id: butcher_recipe.id.clone(),
    };
    engine.execute_command(start_recipe);
    
    // Let recipe complete
    for _ in 0..10 {
        engine.tick();
    }
    
    // Check both outputs produced
    let snapshot = engine.snapshot();
    let butcher_after = snapshot.entities
        .iter()
        .find(|e| e.id == butcher.id)
        .unwrap();
    
    let output = butcher_after.components
        .get("output")
        .expect("Butcher should have output");
    
    let food: u32 = output
        .get("food")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    
    let leather: u32 = output
        .get("leather")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    
    assert_eq!(food, 10, "Should produce 10 food");
    assert_eq!(leather, 2, "Should produce 2 leather");
}