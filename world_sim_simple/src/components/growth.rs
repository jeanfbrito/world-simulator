use crate::resources::ResourceType;
/// Growth and lifecycle system for various resources
///
/// Supports different growth patterns:
/// - Trees: grow from saplings to mature, regrow after cutting
/// - Fruits: ripen over time on bushes/trees
/// - Crops: grow through defined stages
/// - Ores: can optionally replenish based on game settings
/// - Animals: reproduce and maintain populations
use bevy::prelude::*;

/// Different growth behaviors for resources
#[derive(Component, Clone, Debug, Reflect, PartialEq)]
pub enum GrowthPattern {
    /// Trees that grow from sapling to mature
    TreeGrowth {
        current_stage: TreeStage,
        ticks_in_stage: u32,
        growth_time_per_stage: u32,
    },

    /// Fruits/berries that ripen on plants
    FruitRipening {
        ripe_amount: u32,
        unripe_amount: u32,
        ripening_rate: u32,     // How many ripen per interval
        ripening_interval: u32, // Ticks between ripening
        ticks_since_ripening: u32,
    },

    /// Crops that grow through stages
    CropGrowth {
        current_stage: CropStage,
        ticks_in_stage: u32,
        stage_durations: Vec<u32>, // Ticks per stage
    },

    /// Rocks/ores that may respawn
    MineralRespawn {
        respawns: bool,              // Game config: do minerals respawn?
        respawn_chance: f32,         // 0.0 to 1.0
        respawn_check_interval: u32, // How often to check
        ticks_since_check: u32,
    },

    /// Simple regeneration (backwards compatibility)
    SimpleRegeneration {
        regeneration_rate: u32,
        regeneration_interval: u32,
        ticks_since_regen: u32,
    },
}

#[derive(Clone, Debug, Reflect, PartialEq)]
pub enum TreeStage {
    Sapling,
    Young,
    Mature,
    Old,
    Stump, // After being cut
}

#[derive(Clone, Debug, Reflect, PartialEq)]
pub enum CropStage {
    Planted,
    Sprouting,
    Growing,
    Flowering,
    Ripe,
    Harvested,
}

/// Component for resources that grow/regenerate
#[derive(Component, Clone, Debug, Reflect)]
pub struct GrowingResource {
    pub resource_type: ResourceType,
    pub current_amount: u32,
    pub max_amount: u32,
    pub harvestable_amount: u32, // How much can be harvested right now
    pub growth_pattern: GrowthPattern,

    // When depleted, how does it recover?
    pub depletion_behavior: DepletionBehavior,
    pub ticks_since_depletion: u32,

    // Environmental factors
    pub growth_multiplier: f32, // Affected by soil, weather, etc.
    pub seasonal_factor: f32,   // Some things don't grow in winter
}

#[derive(Clone, Debug, Reflect, PartialEq)]
pub enum DepletionBehavior {
    /// Regrows from stump/roots (trees)
    RegrowFromBase { regrow_time: u32 },

    /// Dies and needs replanting (crops)
    RequiresReplanting,

    /// Slowly replenishes (berry bushes)
    GradualReplenishment { rate: u32, interval: u32 },

    /// May or may not respawn (ores)
    ChanceRespawn { chance: f32, check_interval: u32 },

    /// Never regenerates (non-renewable)
    NonRenewable,
}

impl GrowingResource {
    /// Create a tree that grows through stages
    pub fn tree(wood_amount: u32) -> Self {
        Self {
            resource_type: ResourceType::Wood,
            current_amount: wood_amount,
            max_amount: wood_amount,
            harvestable_amount: wood_amount,
            growth_pattern: GrowthPattern::TreeGrowth {
                current_stage: TreeStage::Mature,
                ticks_in_stage: 0,
                growth_time_per_stage: 500, // 50 seconds per stage
            },
            depletion_behavior: DepletionBehavior::RegrowFromBase {
                regrow_time: 3000, // 5 minutes to regrow
            },
            ticks_since_depletion: 0,
            growth_multiplier: 1.0,
            seasonal_factor: 1.0,
        }
    }

    /// Create a fruit bush that ripens berries
    pub fn fruit_bush(initial_ripe: u32, max_fruit: u32) -> Self {
        Self {
            resource_type: ResourceType::Berries, // Default to berries for fruit
            current_amount: initial_ripe,
            max_amount: max_fruit,
            harvestable_amount: initial_ripe,
            growth_pattern: GrowthPattern::FruitRipening {
                ripe_amount: initial_ripe,
                unripe_amount: 0,
                ripening_rate: 1,        // Slower ripening - 1 berry at a time
                ripening_interval: 400,  // Every 40 seconds (much slower regrowth)
                ticks_since_ripening: 0,
            },
            depletion_behavior: DepletionBehavior::GradualReplenishment {
                rate: 1,
                interval: 300,  // Double the time - every 30 seconds (was 150 ticks/15 seconds)
            },
            ticks_since_depletion: 0,
            growth_multiplier: 1.0,
            seasonal_factor: 1.0,
        }
    }

    /// Create a stone deposit (may or may not regenerate based on config)
    pub fn stone_deposit(amount: u32, regenerates: bool) -> Self {
        Self {
            resource_type: ResourceType::Stone,
            current_amount: amount,
            max_amount: amount,
            harvestable_amount: amount,
            growth_pattern: GrowthPattern::MineralRespawn {
                respawns: regenerates,
                respawn_chance: if regenerates { 0.1 } else { 0.0 },
                respawn_check_interval: 6000, // Check every 10 minutes
                ticks_since_check: 0,
            },
            depletion_behavior: if regenerates {
                DepletionBehavior::ChanceRespawn {
                    chance: 0.1,
                    check_interval: 6000,
                }
            } else {
                DepletionBehavior::NonRenewable
            },
            ticks_since_depletion: 0,
            growth_multiplier: 1.0,
            seasonal_factor: 1.0, // Stones don't care about seasons
        }
    }

    /// Update growth for one tick
    pub fn tick_update(&mut self) -> GrowthUpdate {
        let mut update = GrowthUpdate::NoChange;

        // Handle depletion recovery first
        if self.current_amount == 0 {
            self.ticks_since_depletion += 1;

            match &self.depletion_behavior {
                DepletionBehavior::RegrowFromBase { regrow_time } => {
                    if self.ticks_since_depletion >= *regrow_time {
                        // Tree regrows from stump
                        if let GrowthPattern::TreeGrowth { current_stage, .. } =
                            &mut self.growth_pattern
                        {
                            *current_stage = TreeStage::Sapling;
                            self.current_amount = self.max_amount / 4; // Young tree has less wood
                            self.harvestable_amount = 0; // Can't harvest saplings
                            self.ticks_since_depletion = 0;
                            update = GrowthUpdate::Regrown;
                        }
                    }
                }
                DepletionBehavior::GradualReplenishment { rate, interval } => {
                    // Wait for first interval before starting regeneration (don't regenerate at tick 0)
                    if self.ticks_since_depletion > 0 && self.ticks_since_depletion % interval == 0 {
                        self.current_amount = (self.current_amount + rate).min(self.max_amount);
                        self.harvestable_amount = self.current_amount;
                        if self.current_amount > 0 {
                            update = GrowthUpdate::Replenished(self.current_amount);
                        }
                    }
                }
                _ => {}
            }

            return update;
        }

        // Handle normal growth patterns
        match &mut self.growth_pattern {
            GrowthPattern::TreeGrowth {
                current_stage,
                ticks_in_stage,
                growth_time_per_stage,
            } => {
                *ticks_in_stage += 1;
                if *ticks_in_stage >= *growth_time_per_stage {
                    *ticks_in_stage = 0;
                    let new_stage = match current_stage {
                        TreeStage::Sapling => TreeStage::Young,
                        TreeStage::Young => TreeStage::Mature,
                        TreeStage::Mature => TreeStage::Old,
                        _ => return update,
                    };
                    *current_stage = new_stage.clone();

                    // Update harvestable amount based on stage
                    self.harvestable_amount = match &new_stage {
                        TreeStage::Sapling => 0,
                        TreeStage::Young => self.max_amount / 2,
                        TreeStage::Mature => self.max_amount,
                        TreeStage::Old => self.max_amount,
                        TreeStage::Stump => 0,
                    };

                    update = GrowthUpdate::StageChanged(format!("{:?}", new_stage));
                }
            }

            GrowthPattern::FruitRipening {
                ripe_amount,
                unripe_amount,
                ripening_rate,
                ripening_interval,
                ticks_since_ripening,
            } => {
                *ticks_since_ripening += 1;
                if *ticks_since_ripening >= *ripening_interval && *ripe_amount < self.max_amount {
                    *ticks_since_ripening = 0;
                    let ripened = (*ripening_rate).min(self.max_amount - *ripe_amount);
                    *ripe_amount += ripened;
                    self.current_amount = *ripe_amount;
                    self.harvestable_amount = *ripe_amount;
                    update = GrowthUpdate::Ripened(ripened);
                }
            }

            GrowthPattern::SimpleRegeneration {
                regeneration_rate,
                regeneration_interval,
                ticks_since_regen,
            } => {
                *ticks_since_regen += 1;
                if *ticks_since_regen >= *regeneration_interval
                    && self.current_amount < self.max_amount
                {
                    *ticks_since_regen = 0;
                    let regenerated =
                        (*regeneration_rate).min(self.max_amount - self.current_amount);
                    self.current_amount += regenerated;
                    self.harvestable_amount = self.current_amount;
                    update = GrowthUpdate::Regenerated(regenerated);
                }
            }

            _ => {}
        }

        update
    }

    /// Harvest resources from this node
    pub fn harvest(&mut self, amount: u32) -> u32 {
        let harvested = amount.min(self.harvestable_amount);
        self.harvestable_amount -= harvested;
        self.current_amount -= harvested;

        // Mark as depleted if empty
        if self.current_amount == 0 {
            self.ticks_since_depletion = 0;

            // Trees become stumps
            if let GrowthPattern::TreeGrowth { current_stage, .. } = &mut self.growth_pattern {
                *current_stage = TreeStage::Stump;
            }
        }

        harvested
    }
}

/// Result of a growth update
#[derive(Debug, Clone)]
pub enum GrowthUpdate {
    NoChange,
    Regenerated(u32),
    Ripened(u32),
    Replenished(u32),
    StageChanged(String),
    Regrown,
}

/// Event fired when a resource grows/changes
#[derive(Event, Debug, Clone)]
pub struct ResourceGrowthEvent {
    pub entity: Entity,
    pub resource_type: ResourceType,
    pub update: GrowthUpdate,
    pub new_amount: u32,
}

/// Tag component for resources that grow
#[derive(Component, Debug, Clone, Reflect)]
pub struct GrowthEnabledTag;
