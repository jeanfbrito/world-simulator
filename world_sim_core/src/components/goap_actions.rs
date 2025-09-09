//! GOAP action components for worker behaviors

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use bevy_dogoap::prelude::*;

// Basic survival actions

/// Action to eat food and reduce hunger
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct EatAction;

/// Action to rest and restore energy
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct RestAction;

/// Action to sleep at home for full energy restoration
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct SleepAction;

// Resource gathering actions

/// Action to harvest wood from trees
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct HarvestWoodAction;

/// Action to gather food from bushes or farms
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct GatherFoodAction;

/// Action to mine stone from quarries
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct MineStoneAction;

// Storage and inventory actions

/// Action to store resources at storage building
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct StoreResourcesAction;

/// Action to retrieve resources from storage
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct RetrieveResourcesAction;

/// Action to drop excess inventory
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct DropInventoryAction;

// Building and construction actions

/// Action to construct a house
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct BuildHouseAction;

/// Action to construct storage building
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct BuildStorageAction;

/// Action to construct a farm
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct BuildFarmAction;

/// Action to repair damaged buildings
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct RepairBuildingAction;

// Movement and navigation actions

/// Action to go to the nearest resource node
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct GoToResourceAction;

/// Action to go to storage building
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct GoToStorageAction;

/// Action to go home
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct GoToHomeAction;

/// Action to go to construction site
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct GoToConstructionAction;

/// Action to explore and find new resources
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct ExploreAction;

// Social and settlement actions

/// Action to help another worker
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct HelpWorkerAction;

/// Action to defend settlement from threats
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct DefendAction;

/// Action to trade resources with merchant
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct TradeAction;

// Production and crafting actions

/// Action to process raw resources into refined materials
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct ProcessResourcesAction;

/// Action to craft tools or items
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct CraftItemAction;

/// Action to farm/plant crops
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct FarmAction;

// Idle and waiting actions

/// Action to idle when no tasks available
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct IdleAction;

/// Action to wait for a specific condition
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct WaitAction;

// Complex multi-step actions

/// Action to complete a full harvest cycle (go to resource, harvest, return to storage)
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct CompleteHarvestCycleAction;

/// Action to complete a construction project
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct CompleteConstructionAction;