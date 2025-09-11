use super::{ActionPlan, GoapAction, WorldState};
use crate::components::NameComponent;
use bevy::prelude::*;
use colored::Colorize;

/// Enhanced AI debug logging system
pub struct AIDebugLogger;

impl AIDebugLogger {
    /// Log agent state with name and key details
    pub fn log_agent_state(
        entity: Entity,
        name: Option<&NameComponent>,
        state: &WorldState,
        goal: &str,
    ) {
        let agent_name = name
            .map(|n| n.name.clone())
            .unwrap_or_else(|| format!("Agent {:?}", entity));

        // Extract key state values
        let hunger = state
            .get("is_hungry")
            .map(|v| format!("{:?}", v))
            .unwrap_or("N/A".to_string());
        let energy = state
            .get("has_energy")
            .map(|v| format!("{:?}", v))
            .unwrap_or("N/A".to_string());
        let wood = state
            .get("has_wood")
            .map(|v| format!("{:?}", v))
            .unwrap_or("0".to_string());
        let food = state
            .get("has_food")
            .map(|v| format!("{:?}", v))
            .unwrap_or("0".to_string());
        let at_location = if state
            .get("at_resource")
            .map(|v| matches!(v, super::StateValue::Bool(true)))
            .unwrap_or(false)
        {
            "Resource"
        } else if state
            .get("at_storage")
            .map(|v| matches!(v, super::StateValue::Bool(true)))
            .unwrap_or(false)
        {
            "Storage"
        } else if state
            .get("at_home")
            .map(|v| matches!(v, super::StateValue::Bool(true)))
            .unwrap_or(false)
        {
            "Home"
        } else {
            "Wilderness"
        };

        println!(
            "{} {} | Goal: {} | Hunger: {} | Energy: {} | Wood: {} | Food: {} | Location: {}",
            "👤".blue(),
            agent_name.cyan(),
            goal.yellow(),
            hunger.red(),
            energy.green(),
            wood.white(),
            food.magenta(),
            at_location.bright_blue()
        );
    }

    /// Log why planning failed
    pub fn log_planning_failure(
        entity: Entity,
        name: Option<&NameComponent>,
        goal: &str,
        available_actions: &[&GoapAction],
        state: &WorldState,
    ) {
        let agent_name = name
            .map(|n| n.name.clone())
            .unwrap_or_else(|| format!("Agent {:?}", entity));

        println!(
            "{} {} failed to plan for goal: {}",
            "❌".red(),
            agent_name.cyan(),
            goal.yellow()
        );

        // Show which actions were considered but couldn't be used
        let mut valid_actions = Vec::new();
        let mut invalid_actions = Vec::new();

        for action in available_actions {
            if action.is_valid(state) {
                valid_actions.push(action.name.as_str());
            } else {
                invalid_actions.push(action.name.as_str());
            }
        }

        if !valid_actions.is_empty() {
            println!(
                "  {} Valid actions: {}",
                "✓".green(),
                valid_actions.join(", ").green()
            );
        }

        if !invalid_actions.is_empty() {
            println!(
                "  {} Invalid actions: {}",
                "✗".red(),
                invalid_actions.join(", ").bright_black()
            );
        }

        // Show what's missing for the goal
        println!("  {} Goal requirements not met", "ℹ".blue());
    }

    /// Log successful plan creation
    pub fn log_plan_created(entity: Entity, name: Option<&NameComponent>, plan: &ActionPlan) {
        let agent_name = name
            .map(|n| n.name.clone())
            .unwrap_or_else(|| format!("Agent {:?}", entity));

        let action_names: Vec<&str> = plan.actions.iter().map(|a| a.name.as_str()).collect();

        let total_cost: f32 = plan.actions.iter().map(|a| a.cost).sum();
        println!(
            "{} {} created plan: [{}] (cost: {:.1})",
            "📋".green(),
            agent_name.cyan(),
            action_names.join(" → ").yellow(),
            total_cost.to_string().bright_black()
        );
    }

    /// Log action execution
    pub fn log_action_execution(
        entity: Entity,
        name: Option<&NameComponent>,
        action: &str,
        progress: f32,
    ) {
        let agent_name = name
            .map(|n| n.name.clone())
            .unwrap_or_else(|| format!("Agent {:?}", entity));

        let progress_bar = create_progress_bar(progress);

        println!(
            "{} {} executing: {} {}",
            "⚙".yellow(),
            agent_name.cyan(),
            action.green(),
            progress_bar
        );
    }

    /// Log tick summary
    pub fn log_tick_summary(
        tick: u32,
        active_agents: usize,
        planning_agents: usize,
        executing_agents: usize,
        idle_agents: usize,
    ) {
        println!(
            "\n{} {} | Agents: {} total | {} planning | {} working | {} idle",
            "🕐".bright_blue(),
            format!("TICK {}", tick).bright_blue().bold(),
            active_agents.to_string().white(),
            planning_agents.to_string().yellow(),
            executing_agents.to_string().green(),
            idle_agents.to_string().bright_black()
        );
    }

    /// Log resource summary
    pub fn log_resource_summary(
        wood_available: usize,
        stone_available: usize,
        food_available: usize,
        trees: usize,
        rocks: usize,
        berries: usize,
    ) {
        println!(
            "{} Resources | Storage: {}🪵 {}⛏ {}🍖 | World: {}🌲 {}🪨 {}🫐",
            "📦".yellow(),
            wood_available,
            stone_available,
            food_available,
            trees,
            rocks,
            berries
        );
    }
}

fn create_progress_bar(progress: f32) -> String {
    let width = 10;
    let filled = (progress * width as f32) as usize;
    let empty = width - filled;

    format!(
        "[{}{}] {:.0}%",
        "█".repeat(filled).green(),
        "░".repeat(empty).bright_black(),
        progress * 100.0
    )
}

/// System to provide enhanced debug output
pub fn enhanced_debug_system(
    sim_state: Res<crate::SimulationState>,
    agents: Query<(Entity, Option<&NameComponent>, &WorldState), With<crate::ai::WorkerAI>>,
    plans: Query<&ActionPlan>,
) {
    if !sim_state.just_ticked {
        return;
    }

    let total = agents.iter().count();
    let planning = agents
        .iter()
        .filter(|(e, _, _)| plans.get(*e).is_err())
        .count();
    let executing = agents
        .iter()
        .filter(|(e, _, _)| plans.get(*e).is_ok())
        .count();
    let idle = total - planning - executing;

    AIDebugLogger::log_tick_summary(sim_state.tick, total, planning, executing, idle);
}
