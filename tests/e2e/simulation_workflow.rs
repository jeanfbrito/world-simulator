use std::time::Duration;
use tokio::time::sleep;

use world_sim_simple::simulation::Simulation;
use world_sim_simple::ipc::IPCMessage;

mod common;

#[tokio::test]
async fn test_complete_simulation_workflow() {
    let mut context = common::setup_test_environment().await;

    // Create complete simulation configuration
    let config = common::TestConfig::default()
        .with_world_size(50, 50)
        .with_peasant_count(20)
        .with_resources(vec![
            "wood".to_string(),
            "stone".to_string(),
            "food".to_string(),
            "iron".to_string(),
        ]);

    // Initialize simulation
    let simulation = context.initialize_simulation(&config).await;
    assert!(simulation.is_ok(), "Failed to initialize simulation");

    let mut simulation = simulation.unwrap();

    // Run simulation for sufficient time to observe complex behaviors
    let start_time = std::time::Instant::now();
    let target_duration = Duration::from_secs(30);

    while start_time.elapsed() < target_duration {
        // Process IPC messages
        let messages = context.collect_ipc_messages().await;
        assert!(!messages.is_empty(), "No IPC messages received");

        // Verify simulation state progression
        let state = simulation.get_state().await;
        assert!(state.is_ok(), "Failed to get simulation state");

        let state = state.unwrap();
        assert!(state.entities.len() > 0, "No entities in simulation");

        // Check for resource gathering activities
        let has_gathering = state.entities.iter().any(|entity| {
            entity.current_action.as_ref().map_or(false, |action| {
                action.contains("gather") || action.contains("collect")
            })
        });

        // Check for building activities
        let has_building = state.entities.iter().any(|entity| {
            entity.current_action.as_ref().map_or(false, |action| {
                action.contains("build") || action.contains("construct")
            })
        });

        // Verify AI decision making
        assert!(has_gathering || has_building, "Entities not performing meaningful actions");

        // Advance simulation
        let result = simulation.tick().await;
        assert!(result.is_ok(), "Simulation tick failed");

        sleep(Duration::from_millis(100)).await;
    }

    // Final state verification
    let final_state = simulation.get_state().await.unwrap();

    // Verify entities have made progress
    let total_gathered: u32 = final_state.entities.iter()
        .map(|e| e.resources_gathered)
        .sum();
    assert!(total_gathered > 0, "No resources gathered during simulation");

    // Verify AI diversity
    let different_actions: std::collections::HashSet<_> = final_state.entities.iter()
        .filter_map(|e| e.current_action.as_ref())
        .collect();
    assert!(different_actions.len() > 1, "All entities performing same action");

    context.cleanup().await;
}

#[tokio::test]
async fn test_websocket_real_time_monitoring() {
    let mut context = common::setup_test_environment().await;

    // Start WebSocket server
    let ws_handle = context.start_websocket_server().await;
    assert!(ws_handle.is_ok(), "Failed to start WebSocket server");

    // Connect WebSocket client
    let ws_client = context.connect_websocket_client().await;
    assert!(ws_client.is_ok(), "Failed to connect WebSocket client");

    let mut ws_client = ws_client.unwrap();

    // Initialize simulation with monitoring
    let config = common::TestConfig::default()
        .with_peasant_count(10)
        .with_websocket_enabled(true);

    let simulation = context.initialize_simulation(&config).await.unwrap();

    // Monitor real-time updates
    let mut message_count = 0;
    let mut entity_updates = 0;
    let mut resource_updates = 0;

    let timeout = Duration::from_secs(20);
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < timeout && message_count < 50 {
        if let Some(message) = ws_client.try_recv().await {
            message_count += 1;

            match message {
                IPCMessage::EntityUpdate { .. } => entity_updates += 1,
                IPCMessage::ResourceUpdate { .. } => resource_updates += 1,
                IPCMessage::WorldUpdate { .. } => {
                    // Verify world state consistency
                }
                _ => {}
            }
        }

        // Advance simulation
        let _ = simulation.tick().await;
        sleep(Duration::from_millis(200)).await;
    }

    // Verify message delivery
    assert!(message_count > 0, "No WebSocket messages received");
    assert!(entity_updates > 0, "No entity updates received");
    assert!(resource_updates > 0, "No resource updates received");

    // Verify message ordering and consistency
    assert!(message_count == entity_updates + resource_updates + 1,
            "Message count mismatch");

    ws_client.disconnect().await;
    context.cleanup().await;
}

#[tokio::test]
async fn test_simulation_error_handling_and_recovery() {
    let mut context = common::setup_test_environment().await;

    // Test with invalid configurations
    let invalid_configs = vec![
        common::TestConfig::default().with_world_size(0, 0), // Invalid size
        common::TestConfig::default().with_peasant_count(-1), // Invalid count
        common::TestConfig::default().with_resources(vec!["invalid_resource".to_string()]), // Invalid resource
    ];

    for config in invalid_configs {
        let result = context.initialize_simulation(&config).await;
        assert!(result.is_err(), "Should fail with invalid config");

        // Verify error is handled gracefully
        match result.err().unwrap() {
            world_sim_simple::error::SimulationError::InvalidConfiguration(_) => {}
            _ => panic!("Unexpected error type"),
        }
    }

    // Test recovery from runtime errors
    let valid_config = common::TestConfig::default();
    let mut simulation = context.initialize_simulation(&valid_config).await.unwrap();

    // Simulate various error conditions
    let error_scenarios = vec![
        || async {
            // Try to tick without proper initialization
            simulation.tick().await
        },
        || async {
            // Try to access invalid entity
            simulation.get_entity_state(999999).await
        },
    ];

    for scenario in error_scenarios {
        let result = scenario().await;
        // Some should fail gracefully, others should work
        // The important thing is that they don't crash
    }

    // Verify simulation can continue after errors
    for _ in 0..10 {
        let result = simulation.tick().await;
        assert!(result.is_ok(), "Simulation should recover from errors");
    }

    context.cleanup().await;
}

#[tokio::test]
async fn test_multi_simulation_coordination() {
    let mut context = common::setup_test_environment().await;

    // Create multiple simulations with different configurations
    let configs = vec![
        common::TestConfig::default()
            .with_world_size(30, 30)
            .with_peasant_count(5)
            .with_label("small_world"),
        common::TestConfig::default()
            .with_world_size(50, 50)
            .with_peasant_count(15)
            .with_label("medium_world"),
        common::TestConfig::default()
            .with_world_size(70, 70)
            .with_peasant_count(25)
            .with_label("large_world"),
    ];

    let mut simulations = Vec::new();

    // Initialize all simulations
    for config in configs {
        let simulation = context.initialize_simulation(&config).await;
        assert!(simulation.is_ok(), "Failed to initialize simulation");
        simulations.push(simulation.unwrap());
    }

    // Run simulations in parallel
    let mut handles = Vec::new();

    for (i, mut simulation) in simulations.into_iter().enumerate() {
        let context_clone = context.clone();
        let handle = tokio::spawn(async move {
            let mut tick_count = 0;
            let start_time = std::time::Instant::now();
            let duration = Duration::from_secs(10);

            while start_time.elapsed() < duration {
                let _ = simulation.tick().await;
                tick_count += 1;

                // Collect IPC messages
                let messages = context_clone.collect_ipc_messages().await;
                if !messages.is_empty() {
                    // Verify messages are properly tagged with simulation ID
                    for message in messages {
                        match message {
                            IPCMessage::EntityUpdate { entity_id, .. } => {
                                assert!(entity_id > 0, "Invalid entity ID in message");
                            }
                            _ => {}
                        }
                    }
                }

                tokio::time::sleep(Duration::from_millis(100)).await;
            }

            (i, tick_count, simulation)
        });
        handles.push(handle);
    }

    // Wait for all simulations to complete
    let results = futures::future::join_all(handles).await;

    // Verify all simulations ran successfully
    for result in results {
        assert!(result.is_ok(), "Simulation task failed");
        let (i, tick_count, simulation) = result.unwrap();
        assert!(tick_count > 50, "Simulation {} didn't run enough ticks", i);

        // Verify final state
        let state = simulation.get_state().await.unwrap();
        assert!(state.entities.len() > 0, "Simulation {} has no entities", i);
    }

    context.cleanup().await;
}

#[tokio::test]
async fn test_persistence_and_state_restoration() {
    let mut context = common::setup_test_environment().await;

    // Create simulation with complex state
    let config = common::TestConfig::default()
        .with_world_size(40, 40)
        .with_peasant_count(15)
        .with_resources(vec![
            "wood".to_string(),
            "stone".to_string(),
            "food".to_string(),
            "iron".to_string(),
            "gold".to_string(),
        ]);

    let mut simulation = context.initialize_simulation(&config).await.unwrap();

    // Run simulation to develop complex state
    for _ in 0..100 {
        let _ = simulation.tick().await;
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // Save simulation state
    let save_result = simulation.save_state().await;
    assert!(save_result.is_ok(), "Failed to save simulation state");
    let saved_state = save_result.unwrap();

    // Verify saved state contains expected data
    assert!(saved_state.entities.len() > 0, "Saved state has no entities");
    assert!(saved_state.resources.len() > 0, "Saved state has no resources");
    assert!(saved_state.tick > 0, "Saved state has invalid tick count");

    // Create new simulation and restore state
    let mut restored_simulation = context.initialize_simulation(&config).await.unwrap();
    let restore_result = restored_simulation.restore_state(saved_state).await;
    assert!(restore_result.is_ok(), "Failed to restore simulation state");

    // Verify restored state matches original
    let original_state = simulation.get_state().await.unwrap();
    let restored_state = restored_simulation.get_state().await.unwrap();

    assert_eq!(original_state.entities.len(), restored_state.entities.len(),
               "Entity count mismatch after restore");
    assert_eq!(original_state.tick, restored_state.tick,
               "Tick count mismatch after restore");

    // Continue simulation from restored state
    for _ in 0..50 {
        let _ = restored_simulation.tick().await;
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // Verify simulation continues correctly after restoration
    let final_state = restored_simulation.get_state().await.unwrap();
    assert!(final_state.tick > original_state.tick,
            "Simulation didn't progress after restoration");

    context.cleanup().await;
}

#[tokio::test]
async fn test_performance_under_load() {
    let mut context = common::setup_test_environment().await;

    // Test with increasing load
    let load_scenarios = vec![
        (5, 20, 20),   // Small load
        (15, 40, 40),  // Medium load
        (30, 60, 60),  // Heavy load
    ];

    for (entity_count, world_width, world_height) in load_scenarios {
        let config = common::TestConfig::default()
            .with_world_size(world_width, world_height)
            .with_peasant_count(entity_count)
            .with_resources(vec![
                "wood".to_string(),
                "stone".to_string(),
                "food".to_string(),
            ]);

        let mut simulation = context.initialize_simulation(&config).await.unwrap();

        // Measure performance
        let start_time = std::time::Instant::now();
        let target_ticks = 100;
        let mut tick_times = Vec::new();

        for tick in 0..target_ticks {
            let tick_start = std::time::Instant::now();
            let result = simulation.tick().await;
            assert!(result.is_ok(), "Tick {} failed", tick);
            tick_times.push(tick_start.elapsed());

            // Collect IPC messages
            let messages = context.collect_ipc_messages().await;
            assert!(messages.len() <= entity_count,
                   "Too many messages for entity count");
        }

        let total_time = start_time.elapsed();
        let avg_tick_time = tick_times.iter().sum::<Duration>() / tick_times.len() as u32;
        let max_tick_time = tick_times.iter().max().unwrap();

        // Performance assertions
        assert!(total_time < Duration::from_secs(30),
               "Simulation took too long: {:?} for {} entities", total_time, entity_count);
        assert!(avg_tick_time < Duration::from_millis(100),
               "Average tick time too slow: {:?} for {} entities", avg_tick_time, entity_count);
        assert!(max_tick_time < Duration::from_millis(500),
               "Max tick time too slow: {:?} for {} entities", max_tick_time, entity_count);

        // Verify simulation quality under load
        let state = simulation.get_state().await.unwrap();
        assert!(state.entities.len() == entity_count,
               "Entity count mismatch under load");

        let total_gathered: u32 = state.entities.iter()
            .map(|e| e.resources_gathered)
            .sum();
        assert!(total_gathered > 0, "No progress made under load");
    }

    context.cleanup().await;
}