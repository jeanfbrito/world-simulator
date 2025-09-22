# Pack System File Structure

## Complete Directory Layout

```
assets/packs/
└── dev-world/                          # Pack root directory
    ├── pack.lua                         # Pack metadata and configuration
    ├── README.md                        # Pack documentation
    ├── LICENSE                          # Pack license
    ├── icon.png                         # Pack icon (64x64)
    │
    ├── data/                            # All game data definitions
    │   ├── resources/                   # Natural resources (harvestable/mineable)
    │   │   ├── plants/                  # Plant resources
    │   │   │   ├── berry_bush.lua       # Wild berry bushes
    │   │   │   ├── wheat.lua            # Wheat crops
    │   │   │   ├── cotton.lua           # Cotton plants
    │   │   │   └── trees/               # Tree resources
    │   │   │       ├── oak.lua          # Oak trees
    │   │   │       ├── pine.lua         # Pine trees
    │   │   │       └── birch.lua        # Birch trees
    │   │   │
    │   │   ├── minerals/                # Mineral resources
    │   │   │   ├── stone.lua            # Stone deposits
    │   │   │   ├── iron_ore.lua         # Iron ore veins
    │   │   │   ├── coal.lua             # Coal deposits
    │   │   │   ├── gold_ore.lua         # Gold ore veins
    │   │   │   └── gems/                # Gemstone deposits
    │   │   │       ├── diamond.lua      # Diamond deposits
    │   │   │       └── emerald.lua      # Emerald deposits
    │   │   │
    │   │   └── liquids/                 # Liquid resources
    │   │       ├── water_source.lua     # Water springs
    │   │       └── oil_deposit.lua      # Oil deposits
    │   │
    │   ├── items/                       # All inventory items
    │   │   ├── tools/                   # Tool items
    │   │   │   ├── axes/                # Axe tools
    │   │   │   │   ├── stone_axe.lua
    │   │   │   │   ├── iron_axe.lua
    │   │   │   │   └── steel_axe.lua
    │   │   │   ├── pickaxes/            # Pickaxe tools
    │   │   │   │   ├── wooden_pickaxe.lua
    │   │   │   │   ├── stone_pickaxe.lua
    │   │   │   │   ├── iron_pickaxe.lua
    │   │   │   │   └── diamond_pickaxe.lua
    │   │   │   ├── shovels/             # Shovel tools
    │   │   │   │   ├── wooden_shovel.lua
    │   │   │   │   └── iron_shovel.lua
    │   │   │   └── special/             # Special tools
    │   │   │       ├── hammer.lua
    │   │   │       └── wrench.lua
    │   │   │
    │   │   ├── weapons/                 # Weapon items
    │   │   │   ├── swords/              # Sword weapons
    │   │   │   │   ├── wooden_sword.lua
    │   │   │   │   ├── iron_sword.lua
    │   │   │   │   └── steel_sword.lua
    │   │   │   ├── ranged/              # Ranged weapons
    │   │   │   │   ├── bow.lua
    │   │   │   │   ├── crossbow.lua
    │   │   │   │   └── arrows.lua
    │   │   │   └── shields/             # Shield items
    │   │   │       ├── wooden_shield.lua
    │   │   │       └── iron_shield.lua
    │   │   │
    │   │   ├── armor/                   # Armor items
    │   │   │   ├── helmets/             # Helmet armor
    │   │   │   │   ├── leather_helmet.lua
    │   │   │   │   └── iron_helmet.lua
    │   │   │   ├── chestplates/         # Chestplate armor
    │   │   │   │   ├── leather_chestplate.lua
    │   │   │   │   └── iron_chestplate.lua
    │   │   │   └── boots/               # Boot armor
    │   │   │       ├── leather_boots.lua
    │   │   │       └── iron_boots.lua
    │   │   │
    │   │   ├── food/                    # Food items
    │   │   │   ├── fruits/              # Fruit foods
    │   │   │   │   ├── berries.lua      # Harvested berries
    │   │   │   │   ├── apple.lua        # Apples
    │   │   │   │   └── orange.lua       # Oranges
    │   │   │   ├── vegetables/          # Vegetable foods
    │   │   │   │   ├── carrot.lua       # Carrots
    │   │   │   │   ├── potato.lua       # Potatoes
    │   │   │   │   └── tomato.lua       # Tomatoes
    │   │   │   ├── meat/                # Meat foods
    │   │   │   │   ├── raw_meat.lua     # Raw meat
    │   │   │   │   ├── cooked_meat.lua  # Cooked meat
    │   │   │   │   └── fish.lua         # Fish
    │   │   │   └── prepared/            # Prepared foods
    │   │   │       ├── bread.lua        # Baked bread
    │   │   │       ├── stew.lua         # Vegetable stew
    │   │   │       └── pie.lua          # Fruit pie
    │   │   │
    │   │   ├── materials/               # Crafting materials
    │   │   │   ├── raw/                 # Raw materials
    │   │   │   │   ├── wood.lua         # Wood planks
    │   │   │   │   ├── stone.lua        # Stone blocks
    │   │   │   │   ├── iron_ingot.lua   # Iron ingots
    │   │   │   │   ├── gold_ingot.lua   # Gold ingots
    │   │   │   │   └── plant_fiber.lua  # Plant fibers
    │   │   │   ├── processed/           # Processed materials
    │   │   │   │   ├── steel_ingot.lua  # Steel ingots
    │   │   │   │   ├── cloth.lua        # Cloth material
    │   │   │   │   ├── leather.lua      # Leather material
    │   │   │   │   └── rope.lua         # Rope
    │   │   │   └── components/          # Crafting components
    │   │   │       ├── gear.lua         # Mechanical gear
    │   │   │       ├── circuit.lua      # Electronic circuit
    │   │   │       └── handle.lua       # Tool handle
    │   │   │
    │   │   └── misc/                    # Miscellaneous items
    │   │       ├── containers/          # Container items
    │   │       │   ├── bucket.lua       # Water bucket
    │   │       │   └── chest.lua        # Storage chest
    │   │       └── consumables/         # Consumable items
    │   │           ├── torch.lua        # Light source
    │   │           └── bandage.lua      # Healing item
    │   │
    │   ├── recipes/                     # Crafting recipes
    │   │   ├── tools/                   # Tool recipes
    │   │   │   ├── basic_tools.lua      # Basic tool recipes
    │   │   │   ├── advanced_tools.lua   # Advanced tool recipes
    │   │   │   └── tool_upgrades.lua    # Tool upgrade recipes
    │   │   │
    │   │   ├── weapons/                 # Weapon recipes
    │   │   │   ├── melee_weapons.lua    # Melee weapon recipes
    │   │   │   └── ranged_weapons.lua   # Ranged weapon recipes
    │   │   │
    │   │   ├── armor/                   # Armor recipes
    │   │   │   ├── light_armor.lua      # Light armor recipes
    │   │   │   └── heavy_armor.lua      # Heavy armor recipes
    │   │   │
    │   │   ├── food/                    # Food recipes
    │   │   │   ├── cooking.lua          # Cooking recipes
    │   │   │   ├── baking.lua           # Baking recipes
    │   │   │   └── preservation.lua     # Food preservation
    │   │   │
    │   │   ├── buildings/               # Building recipes
    │   │   │   ├── basic_structures.lua # Basic buildings
    │   │   │   ├── workstations.lua     # Crafting stations
    │   │   │   └── decorations.lua      # Decorative items
    │   │   │
    │   │   └── materials/               # Material processing
    │   │       ├── smelting.lua         # Smelting recipes
    │   │       ├── refining.lua         # Refining recipes
    │   │       └── combining.lua        # Material combinations
    │   │
    │   ├── entities/                    # Game entities
    │   │   ├── units/                   # Unit entities
    │   │   │   ├── workers/             # Worker units
    │   │   │   │   ├── peasant.lua      # Basic worker
    │   │   │   │   ├── miner.lua        # Mining specialist
    │   │   │   │   ├── lumberjack.lua   # Wood cutting specialist
    │   │   │   │   ├── farmer.lua       # Farming specialist
    │   │   │   │   └── builder.lua      # Construction specialist
    │   │   │   │
    │   │   │   ├── military/            # Military units
    │   │   │   │   ├── soldier.lua      # Basic soldier
    │   │   │   │   ├── archer.lua       # Ranged unit
    │   │   │   │   ├── knight.lua       # Heavy unit
    │   │   │   │   └── scout.lua        # Fast unit
    │   │   │   │
    │   │   │   └── special/             # Special units
    │   │   │       ├── merchant.lua     # Trading unit
    │   │   │       ├── healer.lua       # Medical unit
    │   │   │       └── engineer.lua     # Technical unit
    │   │   │
    │   │   ├── buildings/               # Building entities
    │   │   │   ├── housing/             # Housing buildings
    │   │   │   │   ├── tent.lua         # Basic shelter
    │   │   │   │   ├── house.lua        # Standard house
    │   │   │   │   └── mansion.lua      # Large house
    │   │   │   │
    │   │   │   ├── production/          # Production buildings
    │   │   │   │   ├── sawmill.lua      # Wood processing
    │   │   │   │   ├── mine.lua         # Mining facility
    │   │   │   │   ├── forge.lua        # Metal working
    │   │   │   │   ├── farm.lua         # Food production
    │   │   │   │   └── workshop.lua     # General crafting
    │   │   │   │
    │   │   │   ├── storage/             # Storage buildings
    │   │   │   │   ├── warehouse.lua    # General storage
    │   │   │   │   ├── granary.lua      # Food storage
    │   │   │   │   └── armory.lua       # Weapon storage
    │   │   │   │
    │   │   │   ├── military/            # Military buildings
    │   │   │   │   ├── barracks.lua     # Unit training
    │   │   │   │   ├── watchtower.lua   # Defense structure
    │   │   │   │   └── wall.lua         # Defensive wall
    │   │   │   │
    │   │   │   └── special/             # Special buildings
    │   │   │       ├── market.lua       # Trading post
    │   │   │       ├── hospital.lua     # Medical facility
    │   │   │       └── research_lab.lua # Research building
    │   │   │
    │   │   ├── wildlife/                # Wildlife entities
    │   │   │   ├── passive/             # Passive animals
    │   │   │   │   ├── deer.lua         # Deer
    │   │   │   │   ├── rabbit.lua       # Rabbit
    │   │   │   │   └── chicken.lua      # Chicken
    │   │   │   │
    │   │   │   ├── hostile/             # Hostile creatures
    │   │   │   │   ├── wolf.lua         # Wolf
    │   │   │   │   ├── bear.lua         # Bear
    │   │   │   │   └── bandit.lua       # Human enemy
    │   │   │   │
    │   │   │   └── neutral/             # Neutral creatures
    │   │   │       ├── cow.lua          # Cow
    │   │   │       ├── pig.lua          # Pig
    │   │   │       └── horse.lua        # Horse
    │   │   │
    │   │   └── objects/                 # Static objects
    │   │       ├── decorations/         # Decorative objects
    │   │       │   ├── statue.lua       # Statue
    │   │       │   └── fountain.lua     # Fountain
    │   │       └── interactive/         # Interactive objects
    │   │           ├── lever.lua        # Lever switch
    │   │           └── door.lua         # Door
    │   │
    │   ├── world/                       # World generation
    │   │   ├── biomes/                  # Biome definitions
    │   │   │   ├── forest.lua           # Forest biome
    │   │   │   ├── desert.lua           # Desert biome
    │   │   │   ├── tundra.lua           # Tundra biome
    │   │   │   ├── meadow.lua           # Meadow biome
    │   │   │   └── mountain.lua         # Mountain biome
    │   │   │
    │   │   ├── structures/              # Generated structures
    │   │   │   ├── village.lua          # Village generation
    │   │   │   ├── dungeon.lua          # Dungeon generation
    │   │   │   └── ruins.lua            # Ruins generation
    │   │   │
    │   │   ├── terrain/                 # Terrain generation
    │   │   │   ├── heightmap.lua        # Height generation
    │   │   │   ├── rivers.lua           # River generation
    │   │   │   └── caves.lua            # Cave generation
    │   │   │
    │   │   └── config/                  # World config
    │   │       ├── world_size.lua       # World dimensions
    │   │       ├── difficulty.lua       # Game difficulty
    │   │       └── generation_rules.lua # Generation rules
    │   │
    │   ├── behaviors/                   # AI behaviors
    │   │   ├── unit_ai/                 # Unit AI scripts
    │   │   │   ├── worker_ai.lua        # Worker behavior
    │   │   │   ├── combat_ai.lua        # Combat behavior
    │   │   │   └── pathfinding.lua      # Movement AI
    │   │   │
    │   │   └── wildlife_ai/             # Wildlife AI
    │   │       ├── grazing.lua          # Grazing behavior
    │   │       ├── hunting.lua          # Hunting behavior
    │   │       └── fleeing.lua          # Escape behavior
    │   │
    │   └── quests/                      # Quest definitions
    │       ├── main_quests/             # Main storyline
    │       │   ├── tutorial.lua         # Tutorial quests
    │       │   └── chapter1.lua         # First chapter
    │       │
    │       └── side_quests/             # Side quests
    │           ├── gathering.lua        # Gathering quests
    │           └── exploration.lua      # Exploration quests
    │
    ├── scripts/                          # Lua scripts and utilities
    │   ├── init.lua                     # Pack initialization
    │   ├── utils.lua                    # Utility functions
    │   └── constants.lua                # Shared constants
    │
    ├── assets/                           # Pack assets (optional)
    │   ├── textures/                    # Texture files
    │   ├── sounds/                      # Sound effects
    │   └── models/                      # 3D models
    │
    ├── localization/                    # Translations
    │   ├── en_US.lua                    # English (US)
    │   ├── es_ES.lua                    # Spanish
    │   └── fr_FR.lua                    # French
    │
    └── tests/                            # Pack tests
        ├── validation/                   # Validation tests
        │   ├── test_resources.lua        # Resource tests
        │   └── test_recipes.lua          # Recipe tests
        └── integration/                  # Integration tests
            └── test_spawning.lua         # Spawning tests
```

## File Naming Conventions

### General Rules
- Use `snake_case` for all file names
- File name should match the ID used in the definition
- Group related items in subdirectories
- Keep directory depth reasonable (max 4 levels)

### Examples
```
ID: berry_bush → File: berry_bush.lua
ID: iron_pickaxe → File: pickaxes/iron_pickaxe.lua
ID: cooked_meat → File: food/meat/cooked_meat.lua
```

## Category Organization

### Resources (`/data/resources/`)
Natural resources that exist in the world and can be harvested:
- **plants/** - Vegetation that can be harvested
- **minerals/** - Rocks and ores that can be mined
- **liquids/** - Water, oil, and other liquids

### Items (`/data/items/`)
Objects that can exist in inventory:
- **tools/** - Items used for work
- **weapons/** - Items used for combat
- **armor/** - Items worn for protection
- **food/** - Consumable food items
- **materials/** - Crafting ingredients
- **misc/** - Other items

### Recipes (`/data/recipes/`)
Crafting instructions grouped by output type:
- **tools/** - Tool crafting recipes
- **weapons/** - Weapon crafting recipes
- **armor/** - Armor crafting recipes
- **food/** - Food preparation recipes
- **buildings/** - Construction recipes
- **materials/** - Material processing

### Entities (`/data/entities/`)
Active game objects:
- **units/** - Movable characters
- **buildings/** - Static structures
- **wildlife/** - Animals and creatures
- **objects/** - Interactive objects

### World (`/data/world/`)
World generation parameters:
- **biomes/** - Biome definitions
- **structures/** - Generated structures
- **terrain/** - Terrain generation
- **config/** - World settings

## Load Order Best Practices

The `load_order` in `pack.lua` should follow dependencies:

```lua
load_order = {
    "resources",  -- First: define raw materials
    "items",      -- Second: items may reference resources
    "recipes",    -- Third: recipes reference items
    "entities",   -- Fourth: entities may have items
    "world",      -- Last: world references everything
}
```

## Multi-Pack Structure

When using multiple packs:

```
assets/packs/
├── base-pack/           # Core game content
├── expansion-pack/      # Additional content
├── user-mod-1/          # User-created mod
└── dev-world/           # Development pack
```

Each pack can reference content from its dependencies.

## Hot Reload Structure

For development with hot reload:

```
assets/packs/dev-world/
├── .watch               # File watch configuration
├── .cache/              # Cached compiled data
└── .backup/             # Backup before changes
```

## Best Practices

1. **One Definition Per File** - Each file contains a single `register_*` call
2. **Logical Grouping** - Group related content in directories
3. **Clear Hierarchy** - Use subdirectories for subcategories
4. **Consistent Naming** - File names match definition IDs
5. **Documentation** - Include comments in Lua files
6. **Validation** - Test files load without errors
7. **Version Control** - Track changes to pack files

## Example File Content

### Resource File (`berry_bush.lua`)
```lua
-- Berry Bush Resource
-- Wild plant that produces berries
-- Found in forest and meadow biomes

register_resource {
    id = "berry_bush",
    name = "Berry Bush",
    -- ... full definition
}
```

### Item File (`berries.lua`)
```lua
-- Berries Item
-- Consumable food harvested from berry bushes
-- Restores 10 hunger points

register_item {
    id = "berries",
    name = "Berries",
    -- ... full definition
}
```

This structure provides maximum flexibility while maintaining organization and discoverability.