//! Test fixtures and mock data for integration tests
//!
//! This module provides predefined test data and configurations
//! that can be used across different test modules.

use world_sim_interface::*;
use world_sim_simple::*;

/// Test world configurations
pub struct TestWorldConfig {
    pub size: (usize, usize),
    pub terrain: TerrainType,
    pub resources: Vec<ResourceFixture>,
    pub units: Vec<UnitFixture>,
}

impl Default for TestWorldConfig {
    fn default() -> Self {
        Self {
            size: (32, 32),
            terrain: TerrainType::Grassland,
            resources: vec![
                ResourceFixture::wood(10, 10, 100),
                ResourceFixture::wood(20, 20, 100),
                ResourceFixture::stone(15, 15, 50),
            ],
            units: vec![
                UnitFixture::peasant(5, 5),
                UnitFixture::peasant(25, 25),
            ],
        }
    }
}

/// Test resource fixture
pub struct ResourceFixture {
    pub x: usize,
    pub y: usize,
    pub resource_type: ResourceType,
    pub amount: u32,
    pub max_amount: u32,
}

impl ResourceFixture {
    pub fn wood(x: usize, y: usize, amount: u32) -> Self {
        Self {
            x,
            y,
            resource_type: ResourceType::Wood,
            amount,
            max_amount: amount,
        }
    }

    pub fn stone(x: usize, y: usize, amount: u32) -> Self {
        Self {
            x,
            y,
            resource_type: ResourceType::Stone,
            amount,
            max_amount: amount,
        }
    }

    pub fn food(x: usize, y: usize, amount: u32) -> Self {
        Self {
            x,
            y,
            resource_type: ResourceType::Food,
            amount,
            max_amount: amount,
        }
    }
}

/// Test unit fixture
pub struct UnitFixture {
    pub x: usize,
    pub y: usize,
    pub unit_type: String,
    pub stats: UnitStats,
}

impl UnitFixture {
    pub fn peasant(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            unit_type: "peasant".to_string(),
            stats: UnitStats {
                health: 100,
                max_health: 100,
                hunger: 0,
                max_hunger: 100,
                energy: 100,
                max_energy: 100,
                movement_speed: 1.0,
                attack_damage: 10,
                attack_speed: 1.0,
                defense: 5,
            },
        }
    }

    pub fn builder(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            unit_type: "builder".to_string(),
            stats: UnitStats {
                health: 80,
                max_health: 80,
                hunger: 0,
                max_hunger: 100,
                energy: 100,
                max_energy: 100,
                movement_speed: 0.8,
                attack_damage: 5,
                attack_speed: 1.0,
                defense: 3,
            },
        }
    }
}

/// Test simulation scenarios
pub struct TestScenario {
    pub name: String,
    pub description: String,
    pub world_config: TestWorldConfig,
    pub expected_ticks: u32,
    pub expected_outcome: ScenarioOutcome,
}

pub enum ScenarioOutcome {
    UnitsSurvive(usize), // Expected number of surviving units
    ResourcesDepleted(Vec<ResourceType>), // Resources that should be depleted
    BuildingsBuilt(usize), // Expected number of buildings
    SpecificState(String), // Custom state validation
}

/// Predefined test scenarios
pub fn get_test_scenarios() -> Vec<TestScenario> {
    vec![
        TestScenario {
            name: "Basic Survival".to_string(),
            description: "Units should survive basic simulation".to_string(),
            world_config: TestWorldConfig::default(),
            expected_ticks: 100,
            expected_outcome: ScenarioOutcome::UnitsSurvive(2),
        },
        TestScenario {
            name: "Resource Gathering".to_string(),
            description: "Units should gather resources".to_string(),
            world_config: TestWorldConfig {
                resources: vec![
                    ResourceFixture::wood(10, 10, 50),
                    ResourceFixture::stone(15, 15, 30),
                ],
                units: vec![UnitFixture::peasant(5, 5)],
                ..Default::default()
            },
            expected_ticks: 200,
            expected_outcome: ScenarioOutcome::ResourcesDepleted(vec![ResourceType::Wood]),
        },
        TestScenario {
            name: "Unit Movement".to_string(),
            description: "Units should move to targets".to_string(),
            world_config: TestWorldConfig {
                units: vec![
                    UnitFixture::peasant(5, 5),
                    UnitFixture::peasant(25, 25),
                ],
                ..Default::default()
            },
            expected_ticks: 150,
            expected_outcome: ScenarioOutcome::SpecificState("units_moved".to_string()),
        },
    ]
}

/// Test pack configurations
pub fn get_test_pack_configs() -> Vec<&'static str> {
    vec![
        r#"
        pack = {
            name = "test_pack",
            description = "Test pack for integration tests",
            version = "1.0.0",
            author = "Test Suite",
        }

        -- Test units
        units = {
            peasant = {
                display_name = "Test Peasant",
                description = "A test peasant unit",
                health = 100,
                max_health = 100,
                movement_speed = 1.0,
                ticks_per_tile = 2,
                components = {"unit", "movable", "ai_controlled"},
                tags = {"civilian", "worker"},
                ai = {
                    personality = "balanced",
                    needs = {
                        hunger = 0.5,
                        energy = 0.5,
                        social = 0.3,
                        safety = 0.4,
                        purpose = 0.6
                    }
                }
            }
        }

        -- Test resources
        resources = {
            wood = {
                display_name = "Wood",
                description = "Wood resource",
                resource_type = "wood",
                amount = 100,
                max_amount = 100,
                regeneration_rate = 0.1,
                harvest_tool = "axe",
                harvest_amount = 10
            },
            stone = {
                display_name = "Stone",
                description = "Stone resource",
                resource_type = "stone",
                amount = 50,
                max_amount = 50,
                regeneration_rate = 0.05,
                harvest_tool = "pickaxe",
                harvest_amount = 5
            }
        }

        -- Test world
        world = {
            size = {width = 32, height = 32},
            terrain = "grassland",
            climate = "temperate",
            resources = {
                {type = "wood", x = 10, y = 10, amount = 100},
                {type = "stone", x = 15, y = 15, amount = 50}
            },
            units = {
                {type = "peasant", x = 5, y = 5},
                {type = "peasant", x = 25, y = 25}
            }
        }
        "#,
    ]
}

/// Test configuration files
pub fn get_test_configs() -> Vec<&'static str> {
    vec![
        r#"
        [simulation]
        tick_rate = 60
        max_entities = 1000
        world_size = { width = 32, height = 32 }

        [ai]
        goap_enabled = true
        utility_ai_enabled = true
        planning_interval = 10

        [performance]
        spatial_indexing = true
        multithreading = false
        debug_mode = true

        [logging]
        level = "info"
        file_output = true
        console_output = false
        "#,
    ]
}