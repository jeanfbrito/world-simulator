//! Common utilities and helpers for integration tests
//!
//! This module provides shared functionality used across different
//! test modules including test setup, teardown, and helper functions.

use world_sim_interface::*;
use world_sim_simple::*;

/// Test configuration structure
pub struct TestConfig {
    pub world_size: usize,
    pub num_units: usize,
    pub simulation_ticks: u32,
    pub enable_debugging: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            world_size: 32, // Smaller world for faster testing
            num_units: 5,
            simulation_ticks: 100,
            enable_debugging: false,
        }
    }
}

/// Test context that holds simulation state and provides cleanup
pub struct TestContext {
    pub config: TestConfig,
    pub app: App,
    pub simulation_state: SimulationState,
}

impl TestContext {
    /// Create a new test context with the given configuration
    pub fn new(config: TestConfig) -> Self {
        let mut app = App::new();
        
        // Initialize the app with minimal plugins for testing
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: None,
            exit_condition: bevy::window::ExitCondition::DontExit,
            close_when_requested: false,
        }));
        
        // Add core simulation plugins
        app.add_plugins(simulation::TickSimulationPlugin);
        app.add_plugins(ComponentsPlugin);
        app.add_plugins(PackSystemPlugin);
        app.add_plugins(WorldPlugin);
        app.add_plugins(SimPlugin);
        app.add_plugins(TilemapPlugin);
        app.add_plugins(ResourcesPlugin);
        app.add_plugins(BuildingsPlugin);
        app.add_plugins(CraftingPlugin);
        app.add_plugins(AIPlugin);
        app.add_plugins(SaveLoadPlugin);
        app.add_plugins(PerformancePlugin);
        app.add_plugins(SystemsPlugin);
        
        // Initialize resources
        app.init_resource::<WorldMap>();
        app.init_resource::<SimulationState>();
        
        // Run startup systems
        app.update();
        
        let simulation_state = app.world().resource::<SimulationState>().clone();
        
        Self {
            config,
            app,
            simulation_state,
        }
    }
    
    /// Run the simulation for a specified number of ticks
    pub fn run_simulation_ticks(&mut self, ticks: u32) {
        for _ in 0..ticks {
            self.app.update();
        }
    }
    
    /// Get the current simulation tick
    pub fn current_tick(&self) -> u32 {
        self.simulation_state.tick
    }
    
    /// Check if simulation is running
    pub fn is_running(&self) -> bool {
        self.simulation_state.running
    }
    
    /// Clean up the test context
    pub fn cleanup(&mut self) {
        // Perform any necessary cleanup
        // This will be called when the test context is dropped
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        self.cleanup();
    }
}

/// Helper function to create a basic test world
pub fn create_test_world(config: &TestConfig) -> TestContext {
    TestContext::new(config.clone())
}

/// Helper function to count entities with a specific component
pub fn count_entities_with_component<T: Component>(world: &World) -> usize {
    world.query::<&T>().iter(world).count()
}

/// Helper function to get entity by name (if NameComponent exists)
pub fn find_entity_by_name(world: &World, name: &str) -> Option<Entity> {
    world.query::<(&NameComponent, Entity)>()
        .iter(world)
        .find(|(name_comp, _)| name_comp.0 == name)
        .map(|(_, entity)| entity)
}

/// Helper function to validate world state
pub fn validate_world_state(world: &World) -> Result<(), String> {
    // Check that essential resources exist
    let resource_count = count_entities_with_component::<ResourceNode>(world);
    if resource_count == 0 {
        return Err("No resources found in world".to_string());
    }
    
    // Check that units exist
    let unit_count = count_entities_with_component::<UnitTag>(world);
    if unit_count == 0 {
        return Err("No units found in world".to_string());
    }
    
    // Check that simulation state is valid
    // TODO: Add more validation logic
    
    Ok(())
}

/// Helper function to wait for a condition to be true
pub fn wait_for_condition<F>(world: &mut World, condition: F, max_ticks: u32) -> bool
where
    F: Fn(&World) -> bool,
{
    for _ in 0..max_ticks {
        if condition(world) {
            return true;
        }
        // Update the world once
        let mut app = App::new();
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: None,
            exit_condition: bevy::window::ExitCondition::DontExit,
            close_when_requested: false,
        }));
        app.update();
    }
    false
}

/// Helper function to create test entities
pub fn create_test_entities(world: &mut World, count: usize) -> Vec<Entity> {
    let mut entities = Vec::new();
    
    for i in 0..count {
        let entity = world.spawn((
            NameComponent(format!("TestEntity_{}", i)),
            UnitTag,
            UnitStats::default(),
            PositionComponent::default(),
        )).id();
        entities.push(entity);
    }
    
    entities
}

/// Helper function to create test resources
pub fn create_test_resources(world: &mut World, positions: &[(usize, usize)]) -> Vec<Entity> {
    let mut entities = Vec::new();
    
    for &(x, y) in positions {
        let entity = world.spawn((
            NameComponent("TestResource".to_string()),
            ResourceNode {
                resource_type: ResourceType::Wood,
                amount: 100,
                max_amount: 100,
                regeneration_rate: 0.1,
                last_harvest_time: 0,
            },
            PositionComponent { x: x as f32, y: y as f32 },
        )).id();
        entities.push(entity);
    }
    
    entities
}

/// Macro for easier test setup
#[macro_export]
macro_rules! setup_test {
    () => {
        let config = TestConfig::default();
        let mut ctx = TestContext::new(config);
    };
    
    ($config:expr) => {
        let mut ctx = TestContext::new($config);
    };
}

/// Macro for asserting entity count
#[macro_export]
macro_rules! assert_entity_count {
    ($world:expr, $component:ty, $expected:expr) => {
        let count = $crate::count_entities_with_component::<$component>($world);
        assert_eq!(count, $expected, "Expected {} entities with component {}, found {}", $expected, stringify!($component), count);
    };
}

/// Macro for asserting simulation state
#[macro_export]
macro_rules! assert_simulation_running {
    ($ctx:expr) => {
        assert!($ctx.is_running(), "Simulation should be running");
    };
}

/// Macro for asserting simulation tick
#[macro_export]
macro_rules! assert_tick_count {
    ($ctx:expr, $expected:expr) => {
        assert_eq!($ctx.current_tick(), $expected, "Expected tick count {}, found {}", $expected, $ctx.current_tick());
    };
}

/// Test result type for convenience
pub type TestResult = Result<(), String>;

/// Helper function to run a test with setup and teardown
pub fn run_test<F>(test_func: F) -> TestResult
where
    F: FnOnce(&mut TestContext) -> TestResult,
{
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);
    
    let result = test_func(&mut ctx);
    
    ctx.cleanup();
    result
}
