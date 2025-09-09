//! Asset Pack Test - Interactive example for testing different game packs

use std::io::{self, Write};
use bevy::prelude::*;
use world_sim_core::{
    asset_manager::{AssetPackManager, AssetPackPlugin, SwitchPackEvent},
    scripting::{
        item_loader::{ItemRegistry, ReloadItemScriptsCommand},
        building_loader::{BuildingRegistry, ReloadBuildingScriptsCommand},
        material_loader::{MaterialRegistry, ReloadMaterialScriptsCommand},
    },
    SimulationPlugin,
};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(AssetReaderPlugin::<FileAssetReader>::new())
        .add_plugins(SimulationPlugin)
        .add_plugins(AssetPackPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            menu_system,
            display_loaded_content,
        ))
        .run();
}

#[derive(Resource)]
struct TestState {
    show_menu: bool,
    last_update: std::time::Instant,
}

impl Default for TestState {
    fn default() -> Self {
        Self {
            show_menu: true,
            last_update: std::time::Instant::now(),
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_manager: Res<AssetPackManager>,
) {
    commands.insert_resource(TestState::default());
    
    println!("\n=== Asset Pack Test System ===");
    println!("Current pack: {}", asset_manager.active_pack);
    
    if let Some(config) = &asset_manager.config {
        println!("Pack: {} v{}", config.pack.name, config.pack.version);
        println!("Author: {}", config.pack.author);
        println!("Description: {}", config.pack.description);
    }
    
    println!("\nAvailable packs:");
    for (name, metadata) in &asset_manager.available_packs {
        println!("  - {}: {}", name, metadata.description);
    }
}

fn menu_system(
    mut state: ResMut<TestState>,
    mut switch_events: EventWriter<SwitchPackEvent>,
    mut reload_items: EventWriter<ReloadItemScriptsCommand>,
    mut reload_buildings: EventWriter<ReloadBuildingScriptsCommand>,
    mut reload_materials: EventWriter<ReloadMaterialScriptsCommand>,
    asset_manager: Res<AssetPackManager>,
) {
    // Only show menu every 2 seconds to avoid spam
    if state.last_update.elapsed().as_secs() < 2 {
        return;
    }
    
    if !state.show_menu {
        return;
    }
    
    state.last_update = std::time::Instant::now();
    
    println!("\n=== Main Menu ===");
    println!("Current Pack: {}", asset_manager.active_pack);
    println!("\n1. Switch to Dwarf Fortress pack");
    println!("2. Switch to Stronghold pack");
    println!("3. Reload current pack scripts");
    println!("4. Show loaded content");
    println!("5. Toggle features");
    println!("6. Exit");
    println!("\nEnter choice (1-6): ");
    
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        match input.trim() {
            "1" => {
                println!("Switching to Dwarf Fortress pack...");
                switch_events.send(SwitchPackEvent {
                    pack_name: "dwarf_fortress".to_string(),
                });
                reload_items.send(ReloadItemScriptsCommand);
                reload_buildings.send(ReloadBuildingScriptsCommand);
                reload_materials.send(ReloadMaterialScriptsCommand);
            }
            "2" => {
                println!("Switching to Stronghold pack...");
                switch_events.send(SwitchPackEvent {
                    pack_name: "stronghold".to_string(),
                });
                reload_items.send(ReloadItemScriptsCommand);
                reload_buildings.send(ReloadBuildingScriptsCommand);
                reload_materials.send(ReloadMaterialScriptsCommand);
            }
            "3" => {
                println!("Reloading scripts...");
                reload_items.send(ReloadItemScriptsCommand);
                reload_buildings.send(ReloadBuildingScriptsCommand);
                reload_materials.send(ReloadMaterialScriptsCommand);
            }
            "4" => {
                state.show_menu = false;
                println!("Showing loaded content (press Enter to return to menu)...");
            }
            "5" => {
                show_features(&asset_manager);
            }
            "6" => {
                println!("Exiting...");
                std::process::exit(0);
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}

fn display_loaded_content(
    mut state: ResMut<TestState>,
    items: Res<ItemRegistry>,
    buildings: Res<BuildingRegistry>,
    materials: Res<MaterialRegistry>,
    asset_manager: Res<AssetPackManager>,
) {
    if state.show_menu {
        return;
    }
    
    // Display content based on current pack
    println!("\n=== Loaded Content for {} ===", asset_manager.active_pack);
    
    // Show items
    println!("\n📦 Items ({}):", items.items.len());
    let mut categories = std::collections::HashMap::new();
    for item in items.items.values() {
        *categories.entry(item.category.clone()).or_insert(0) += 1;
    }
    for (category, count) in categories {
        println!("  - {}: {} items", category, count);
    }
    
    // Show some example items
    println!("\nExample items:");
    for (i, item) in items.items.values().take(5).enumerate() {
        println!("  {}. {} - {}", i + 1, item.name, item.description);
    }
    
    // Show buildings
    println!("\n🏗️ Buildings ({}):", buildings.buildings.len());
    let mut building_types = std::collections::HashMap::new();
    for building in buildings.buildings.values() {
        *building_types.entry(building.category.clone()).or_insert(0) += 1;
    }
    for (category, count) in building_types {
        println!("  - {}: {} buildings", category, count);
    }
    
    // Show materials (if Dwarf Fortress pack)
    if asset_manager.active_pack == "dwarf_fortress" {
        println!("\n⚒️ Materials ({}):", materials.materials.len());
        for (i, material) in materials.materials.values().take(5).enumerate() {
            println!("  {}. {} - Melting point: {}K", 
                i + 1, 
                material.name, 
                material.melting_point.unwrap_or(0)
            );
        }
    }
    
    // Show pack-specific content
    match asset_manager.active_pack.as_str() {
        "dwarf_fortress" => {
            println!("\n🏭 Workshop Tiers:");
            println!("  - Raw Processing: Butcher, Tanner, etc.");
            println!("  - Secondary: Carpenter, Mason, etc.");
            println!("  - Advanced: Forge, Glass Furnace, etc.");
        }
        "stronghold" => {
            println!("\n⚔️ Military Units:");
            println!("  - Archer: 12 gold, bow required");
            println!("  - Spearman: 8 gold, spear required");
            println!("  - Knight: 40 gold, sword + armor + horse");
            
            println!("\n🏰 Production Chains:");
            println!("  - Bread: Wheat Farm → Mill → Bakery");
            println!("  - Ale: Hops Farm → Brewery → Inn");
            println!("  - Weapons: Iron Mine → Blacksmith → Armory");
        }
        _ => {}
    }
    
    println!("\nPress Enter to return to menu...");
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        state.show_menu = true;
    }
}

fn show_features(asset_manager: &AssetPackManager) {
    if let Some(config) = &asset_manager.config {
        println!("\n=== Features for {} ===", asset_manager.active_pack);
        println!("Z-levels: {}", config.features.z_levels);
        println!("Temperature: {}", config.features.temperature_simulation);
        println!("Fluids: {}", config.features.fluid_dynamics);
        println!("Weather: {}", config.features.weather_system);
        println!("Seasons: {}", config.features.seasons);
        println!("Complex Crafting: {}", config.features.complex_crafting);
        println!("Material Properties: {}", config.features.material_properties);
        
        if let Some(moods) = config.features.moods_and_artifacts {
            println!("Moods & Artifacts: {}", moods);
        }
        if let Some(nobles) = config.features.nobles_and_mandates {
            println!("Nobles & Mandates: {}", nobles);
        }
        if let Some(military) = config.features.military_squads {
            println!("Military Squads: {}", military);
        }
        if let Some(trade) = config.features.trade_caravans {
            println!("Trade Caravans: {}", trade);
        }
        if let Some(castle) = config.features.castle_building {
            println!("Castle Building: {}", castle);
        }
        if let Some(siege) = config.features.siege_equipment {
            println!("Siege Equipment: {}", siege);
        }
    }
}