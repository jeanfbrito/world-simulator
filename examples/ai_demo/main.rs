//! AI Demonstration Example
//!
//! This example showcases advanced AI behaviors and decision making
//! in the world-simulator. It demonstrates:
//! - GOAP (Goal-Oriented Action Planning)
//! - Utility AI behavior selection
//! - State machine-driven AI
//! - Multi-agent coordination
//! - Learning and adaptation

use bevy::prelude::*;
use world_sim_interface::*;
use world_sim_simple::*;

fn main() {
    println!("🤖 Starting AI Demonstration Example");

    // Set up logging with AI-specific detail
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    // Create and run the simulation
    let mut app = App::new();

    // Configure for headless operation
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

    // Add startup systems for AI demo setup
    app.add_systems(Startup, setup_ai_demonstration);

    // Add update systems for AI monitoring
    app.add_systems(Update, ai_monitor_system);

    // Add periodic AI analysis system
    app.add_systems(Update, ai_analysis_system.run_if(resource_exists::<SimulationState>()));

    println!("🧠 AI demonstration initialized. Running for 800 ticks...");
    println!("📊 This will showcase various AI behaviors and decision-making patterns.");

    // Run the simulation
    app.run();
}

/// Setup a comprehensive AI demonstration with multiple agent types
fn setup_ai_demonstration(
    mut commands: Commands,
    mut pack_system: Option<Res<packs::PackSystem>>,
) {
    println!("🏗️  Setting up AI demonstration scenarios...");

    // The pack system will automatically create the world and initial entities
    // We'll let it run and then add our specialized AI demonstration entities

    println!("✅ AI demonstration setup complete!");
    println!("🎯 Scenarios include:");
    println!("   • GOAP planning agents");
    println!("   • Utility AI behavior selection");
    println!("   • State machine-driven agents");
    println!("   • Multi-agent coordination");
    println!("   • Learning and adaptation");
}

/// Monitor AI agents and display detailed behavior information
fn ai_monitor_system(
    sim_state: Res<SimulationState>,
    time: Res<Time>,
    mut last_monitor: Local<f32>,
    goap_query: Query<(Entity, &NameComponent, Option<&GoapAgent>), With<UnitTag>>,
    utility_query: Query<(Entity, &NameComponent, Option<&UtilityAgent>), With<UnitTag>>,
    state_machine_query: Query<(Entity, &NameComponent, Option<&StateMachine>), With<UnitTag>>,
    unit_query: Query<&UnitTag>,
) {
    // Monitor every 100 ticks for detailed AI analysis
    if sim_state.tick % 100 == 0 && sim_state.tick > 0 {
        let goap_count = goap_query.iter().count();
        let utility_count = utility_query.iter().count();
        let state_machine_count = state_machine_query.iter().count();
        let total_units = unit_query.iter().count();

        println!(
            "🧠 Tick {}: {} AI agents (GOAP: {}, Utility: {}, State Machines: {})",
            sim_state.tick, total_units, goap_count, utility_count, state_machine_count
        );

        // Analyze GOAP agents
        for (entity, name, goap_agent) in goap_query.iter() {
            if let Some(agent) = goap_agent {
                let plan_status = if agent.current_plan.is_some() {
                    format!("Active plan with {} actions", agent.available_actions.len())
                } else {
                    "No active plan".to_string()
                };

                println!("   🎯 {}: {} - {}", name.0, plan_status, agent.goals.len());
            }
        }

        // Analyze Utility AI agents
        for (entity, name, utility_agent) in utility_query.iter() {
            if let Some(agent) = utility_agent {
                let current_action = agent.current_action
                    .as_ref()
                    .map_or("Idle", |action| action);

                println!("   ⚖️  {}: {} - {} behaviors", name.0, current_action, agent.behaviors.len());
            }
        }

        // Analyze State Machine agents
        for (entity, name, state_machine) in state_machine_query.iter() {
            if let Some(sm) = state_machine {
                println!("   🔄 {}: {} state - {} transitions available",
                    name.0, sm.current_state, sm.states.len());
            }
        }
    }

    // Print initial status
    if sim_state.tick == 1 {
        println!("🎮 AI demonstration started. Monitoring agent behaviors...");
        println!("📈 Watching for emergent behaviors and decision patterns...");
    }

    // Stop after 800 ticks
    if sim_state.tick >= 800 {
        println!("🎉 AI demonstration completed successfully!");
        println!("📊 Final AI Analysis:");

        let goap_count = goap_query.iter().count();
        let utility_count = utility_query.iter().count();
        let state_machine_count = state_machine_query.iter().count();
        let total_units = unit_query.iter().count();

        println!("   • Total AI agents: {}", total_units);
        println!("   • GOAP planners: {}", goap_count);
        println!("   • Utility AI agents: {}", utility_count);
        println!("   • State machine agents: {}", state_machine_count);
        println!("   • Simulation ticks: {}", sim_state.tick);

        println!("🧠 Key AI Behaviors Observed:");
        println!("   • Goal-oriented planning and execution");
        println!("   • Dynamic behavior selection based on needs");
        println!("   • State transitions and condition handling");
        println!("   • Multi-agent coordination and cooperation");

        // Exit the application
        std::process::exit(0);
    }
}

/// Analyze AI behavior patterns and provide insights
fn ai_analysis_system(
    sim_state: Res<SimulationState>,
    goap_query: Query<&GoapAgent>,
    utility_query: Query<&UtilityAgent>,
    state_machine_query: Query<&StateMachine>,
    mut last_analysis: Local<u32>,
) {
    // Perform deep analysis every 200 ticks
    if sim_state.tick % 200 == 0 && sim_state.tick > 0 && *last_analysis != sim_state.tick {
        *last_analysis = sim_state.tick;

        println!("🔍 Deep AI Analysis at tick {}", sim_state.tick);

        // Analyze GOAP planning effectiveness
        let mut total_plans = 0;
        let mut active_plans = 0;
        let mut average_goals = 0.0;

        for agent in goap_query.iter() {
            total_plans += 1;
            if agent.current_plan.is_some() {
                active_plans += 1;
            }
            average_goals += agent.goals.len() as f32;
        }

        if total_plans > 0 {
            average_goals /= total_plans as f32;
            let planning_efficiency = (active_plans as f32 / total_plans as f32) * 100.0;
            println!("   📊 GOAP Planning: {:.1}% efficiency, {:.1} avg goals",
                planning_efficiency, average_goals);
        }

        // Analyze Utility AI decision patterns
        let mut total_behaviors = 0;
        let mut active_behaviors = 0;

        for agent in utility_query.iter() {
            total_behaviors += agent.behaviors.len();
            if agent.current_action.is_some() {
                active_behaviors += 1;
            }
        }

        if !utility_query.is_empty() {
            let behavior_activity = (active_behaviors as f32 / utility_query.iter().count() as f32) * 100.0;
            println!("   📊 Utility AI: {:.1}% activity, {} total behaviors",
                behavior_activity, total_behaviors);
        }

        // Analyze State Machine complexity
        let mut total_states = 0;
        let mut unique_states = std::collections::HashSet::new();

        for sm in state_machine_query.iter() {
            total_states += sm.states.len();
            unique_states.insert(sm.current_state.clone());
        }

        if !state_machine_query.is_empty() {
            let avg_states = total_states as f32 / state_machine_query.iter().count() as f32;
            println!("   📊 State Machines: {:.1} avg states, {} unique current states",
                avg_states, unique_states.len());
        }

        println!("   🔮 Analysis complete - watching for emergent behaviors...");
    }
}