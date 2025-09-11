/// Plant growth types inspired by Dwarf Fortress
///
/// Generic system that supports any kind of plant produce:
/// - Fruits (apples, oranges, berries)
/// - Vegetables (peppers, tomatoes, squash)
/// - Nuts (walnuts, almonds)
/// - Seeds/Grains (wheat, barley)
/// - Leaves (lettuce, cabbage)
/// - Roots (carrots, potatoes)
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Generic plant growth that can be harvested
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum PlantGrowth {
    // Tree/Bush fruits
    Fruit, // Generic fruit (apple, orange, berry, etc.)
    Nut,   // Tree nuts (walnut, almond, chestnut)

    // Crop yields
    Vegetable, // Generic vegetable (pepper, tomato, squash)
    Grain,     // Seeds/grains (wheat, barley, rice)
    Leaf,      // Edible leaves (lettuce, cabbage, spinach)
    Root,      // Root vegetables (carrot, potato, turnip)
    Pod,       // Pods (beans, peas)

    // Special
    Flower, // Edible flowers or for crafting
    Herb,   // Herbs for cooking/medicine
    Fiber,  // Cotton, flax for textiles
}

impl PlantGrowth {
    /// Can this be eaten raw?
    pub fn is_edible_raw(&self) -> bool {
        match self {
            PlantGrowth::Fruit => true,
            PlantGrowth::Nut => true,
            PlantGrowth::Vegetable => false, // Most need cooking
            PlantGrowth::Grain => false,     // Needs milling/cooking
            PlantGrowth::Leaf => true,
            PlantGrowth::Root => false, // Needs cooking
            PlantGrowth::Pod => false,  // Needs cooking
            PlantGrowth::Flower => true,
            PlantGrowth::Herb => true,
            PlantGrowth::Fiber => false, // Not food
        }
    }

    /// Nutrition value when consumed
    pub fn nutrition_value(&self) -> u32 {
        match self {
            PlantGrowth::Fruit => 20, // Good nutrition
            PlantGrowth::Nut => 30,   // High energy
            PlantGrowth::Vegetable => 25,
            PlantGrowth::Grain => 35, // Staple food
            PlantGrowth::Leaf => 10,  // Low calories
            PlantGrowth::Root => 30,  // Filling
            PlantGrowth::Pod => 25,
            PlantGrowth::Flower => 5, // Minimal nutrition
            PlantGrowth::Herb => 5,
            PlantGrowth::Fiber => 0, // Not food
        }
    }

    /// Can be brewed into alcohol?
    pub fn is_brewable(&self) -> bool {
        match self {
            PlantGrowth::Fruit => true,     // Wine, cider
            PlantGrowth::Grain => true,     // Beer, whiskey
            PlantGrowth::Root => true,      // Potato vodka
            PlantGrowth::Vegetable => true, // Some can be fermented
            _ => false,
        }
    }

    /// Processing required before use
    pub fn processing_required(&self) -> ProcessingType {
        match self {
            PlantGrowth::Grain => ProcessingType::Milling,
            PlantGrowth::Fiber => ProcessingType::Threshing,
            PlantGrowth::Pod => ProcessingType::Shelling,
            _ => ProcessingType::None,
        }
    }
}

/// How plant material needs to be processed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum ProcessingType {
    None,      // Can be used directly
    Milling,   // Needs quern/millstone (grain → flour)
    Threshing, // Separate seeds/fiber (flax → linen thread)
    Shelling,  // Remove shells (peas → shelled peas)
    Pressing,  // Extract oil/juice (olives → oil)
    Cooking,   // Must be cooked to eat
}

/// Specific plant produce with metadata
#[derive(Component, Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct PlantProduce {
    pub growth_type: PlantGrowth,
    pub specific_name: String, // "Apple", "Carrot", "Wheat"
    pub stack_size: u32,       // How many in this stack
    pub quality: ProduceQuality,
    pub days_until_rot: Option<u32>, // Some things don't rot
    pub season_harvested: Season,
}

/// Quality levels affect value and nutrition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum ProduceQuality {
    Poor,      // Drought, disease, poor soil
    Normal,    // Standard quality
    Good,      // Well-tended, good conditions
    Excellent, // Perfect conditions, master farmer
}

/// Seasons for growth and harvest
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl PlantProduce {
    /// Create a new fruit produce (generic - could be apple, orange, berry)
    pub fn fruit(name: String, quantity: u32) -> Self {
        Self {
            growth_type: PlantGrowth::Fruit,
            specific_name: name,
            stack_size: quantity,
            quality: ProduceQuality::Normal,
            days_until_rot: Some(7), // Fruits rot in a week
            season_harvested: Season::Summer,
        }
    }

    /// Create grain produce (wheat, barley, etc.)
    pub fn grain(name: String, quantity: u32) -> Self {
        Self {
            growth_type: PlantGrowth::Grain,
            specific_name: name,
            stack_size: quantity,
            quality: ProduceQuality::Normal,
            days_until_rot: None, // Grain stores well
            season_harvested: Season::Autumn,
        }
    }

    /// Create root vegetable (carrot, potato, etc.)
    pub fn root(name: String, quantity: u32) -> Self {
        Self {
            growth_type: PlantGrowth::Root,
            specific_name: name,
            stack_size: quantity,
            quality: ProduceQuality::Normal,
            days_until_rot: Some(30), // Roots last a month
            season_harvested: Season::Autumn,
        }
    }

    /// Get total nutrition from this stack
    pub fn total_nutrition(&self) -> u32 {
        let base = self.growth_type.nutrition_value() * self.stack_size;
        match self.quality {
            ProduceQuality::Poor => base * 75 / 100,
            ProduceQuality::Normal => base,
            ProduceQuality::Good => base * 125 / 100,
            ProduceQuality::Excellent => base * 150 / 100,
        }
    }

    /// Can this be eaten to reduce hunger?
    pub fn is_edible(&self, is_cooked: bool) -> bool {
        if self.growth_type == PlantGrowth::Fiber {
            return false; // Never edible
        }

        self.growth_type.is_edible_raw() || is_cooked
    }
}

/// What a plant can produce at different growth stages
#[derive(Component, Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct PlantYield {
    pub plant_name: String, // "Apple Tree", "Berry Bush", "Wheat"
    pub growth_type: PlantGrowth,
    pub yield_per_harvest: u32,
    pub harvests_per_year: u32,
    pub preferred_season: Season,
    pub requires_replanting: bool, // Crops vs perennials
}

/// Examples of different plant yields
impl PlantYield {
    pub fn apple_tree() -> Self {
        Self {
            plant_name: "Apple Tree".to_string(),
            growth_type: PlantGrowth::Fruit,
            yield_per_harvest: 20,
            harvests_per_year: 1,
            preferred_season: Season::Autumn,
            requires_replanting: false,
        }
    }

    pub fn berry_bush() -> Self {
        Self {
            plant_name: "Berry Bush".to_string(),
            growth_type: PlantGrowth::Fruit,
            yield_per_harvest: 10,
            harvests_per_year: 2, // Summer and autumn
            preferred_season: Season::Summer,
            requires_replanting: false,
        }
    }

    pub fn pepper_plant() -> Self {
        Self {
            plant_name: "Pepper Plant".to_string(),
            growth_type: PlantGrowth::Vegetable,
            yield_per_harvest: 5,
            harvests_per_year: 3, // Long growing season
            preferred_season: Season::Summer,
            requires_replanting: true,
        }
    }

    pub fn wheat_crop() -> Self {
        Self {
            plant_name: "Wheat".to_string(),
            growth_type: PlantGrowth::Grain,
            yield_per_harvest: 30,
            harvests_per_year: 1,
            preferred_season: Season::Autumn,
            requires_replanting: true,
        }
    }

    pub fn carrot_crop() -> Self {
        Self {
            plant_name: "Carrot".to_string(),
            growth_type: PlantGrowth::Root,
            yield_per_harvest: 8,
            harvests_per_year: 2,
            preferred_season: Season::Autumn,
            requires_replanting: true,
        }
    }
}
