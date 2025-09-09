//! Simple test to verify asset pack loading works

use std::fs;
use std::path::PathBuf;

fn main() {
    println!("=== Asset Pack Loader Test ===\n");
    
    // Test loading Stronghold pack (our default)
    test_pack("stronghold");
    
    println!("\n{}\n", "=".repeat(50));
    
    // Test loading Dwarf Fortress pack
    test_pack("dwarf_fortress");
}

fn test_pack(pack_name: &str) {
    println!("Testing pack: {}", pack_name);
    println!("{}", "-".repeat(30));
    
    let pack_path = PathBuf::from(format!("assets/packs/{}", pack_name));
    
    // Check if pack exists
    if !pack_path.exists() {
        println!("❌ Pack directory not found: {:?}", pack_path);
        return;
    }
    println!("✅ Pack directory found");
    
    // Check config.toml
    let config_path = pack_path.join("config.toml");
    if !config_path.exists() {
        println!("❌ Config file not found");
        return;
    }
    
    match fs::read_to_string(&config_path) {
        Ok(config_str) => {
            println!("✅ Config loaded ({} bytes)", config_str.len());
            
            // Parse basic info without full deserialization
            if config_str.contains("[pack]") {
                println!("  - Has pack metadata");
            }
            if config_str.contains("[features]") {
                println!("  - Has feature flags");
            }
            if config_str.contains("[balance]") {
                println!("  - Has balance settings");
            }
        }
        Err(e) => {
            println!("❌ Failed to read config: {}", e);
            return;
        }
    }
    
    // Check scripts directory
    let scripts_path = pack_path.join("scripts");
    if scripts_path.exists() {
        println!("✅ Scripts directory found");
        
        // Count script files by category
        count_scripts(&scripts_path, "items");
        count_scripts(&scripts_path, "buildings");
        count_scripts(&scripts_path, "units");
        count_scripts(&scripts_path, "economy");
        count_scripts(&scripts_path, "workshops");
        count_scripts(&scripts_path, "materials");
    } else {
        println!("⚠️  No scripts directory");
    }
    
    // Check data directory
    let data_path = pack_path.join("data");
    if data_path.exists() {
        println!("✅ Data directory found");
    }
    
    // Show pack-specific content
    match pack_name {
        "stronghold" => {
            println!("\n📦 Stronghold Content Summary:");
            println!("  - Focus: Castle building & siege warfare");
            println!("  - Simple production chains (Wheat → Flour → Bread)");
            println!("  - Military units with gold costs");
            println!("  - Castle defenses (walls, towers, moats)");
            println!("  - Happiness/Fear factor economy");
        }
        "dwarf_fortress" => {
            println!("\n📦 Dwarf Fortress Content Summary:");
            println!("  - Focus: Complex crafting & materials");
            println!("  - Material properties (melting points, density)");
            println!("  - Workshop tiers (raw → secondary → advanced)");
            println!("  - Reactions and transformations");
            println!("  - Z-levels and underground");
        }
        _ => {}
    }
}

fn count_scripts(scripts_path: &PathBuf, category: &str) {
    let category_path = scripts_path.join(category);
    if category_path.exists() {
        let count = fs::read_dir(&category_path)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.path()
                            .extension()
                            .and_then(|s| s.to_str())
                            .map(|s| s == "lua")
                            .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0);
        
        if count > 0 {
            println!("  - {}: {} scripts", category, count);
        }
    }
}