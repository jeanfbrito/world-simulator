//! Material definition loader from Lua scripts
//! Inspired by Dwarf Fortress material system

use bevy::prelude::AssetServer;
use super::types::{LuaScript, ScriptCollection};
use bevy::prelude::*;
use std::collections::HashMap;

/// Component to mark entities with material scripts
#[derive(Component)]
pub struct MaterialScript {
    pub script_path: String,
    pub loaded: bool,
}

/// Material states
#[derive(Clone, Debug, Reflect, PartialEq)]
pub enum MaterialState {
    Solid,
    Liquid,
    Gas,
    Powder,
    Paste,
}

/// Material categories
#[derive(Clone, Debug, Reflect, PartialEq)]
pub enum MaterialCategory {
    Metal,
    Stone,
    Wood,
    Organic,
    Liquid,
    Gas,
    Glass,
    Ceramic,
    Cloth,
    Leather,
    Gem,
}

/// State transition for materials
#[derive(Clone, Debug, Reflect)]
pub struct StateTransition {
    pub temperature: f32,  // Kelvin
    pub new_state: MaterialState,
}

/// Special material properties
#[derive(Clone, Debug, Reflect, Default)]
pub struct SpecialProperties {
    pub flammable: bool,
    pub organic: bool,
    pub renewable: bool,
    pub oxidizes: bool,
    pub magnetic: bool,
    pub precious: bool,
    pub legendary: bool,
    pub deep_material: bool,
    pub never_dulls: bool,
    pub antimicrobial: bool,
    pub tarnishes: bool,
    pub never_tarnishes: bool,
    pub decorative: bool,
    pub currency: bool,
    pub demon_bait: bool,
    pub igneous: bool,
    pub sedimentary: bool,
    pub volcanic: bool,
    pub magma_safe: bool,
    pub flux: bool,  // Used in steel production
    pub glass_like: bool,
    pub magma_created: bool,
}

/// Main material definition
#[derive(Clone, Debug, Reflect)]
pub struct LuaMaterial {
    pub id: String,
    pub name: String,
    pub category: MaterialCategory,
    
    // Physical properties
    pub density: f32,  // kg/m³
    pub melting_point: f32,  // Kelvin
    pub boiling_point: f32,  // Kelvin
    pub specific_heat: f32,  // J/(kg·K)
    pub thermal_conductivity: f32,  // W/(m·K)
    
    // Mechanical properties (in kPa)
    pub yield_strength: f32,
    pub tensile_strength: f32,
    pub fracture_toughness: f32,
    pub impact_resistance: f32,
    pub shear_resistance: f32,
    pub compressive_strength: f32,
    
    // Material behavior
    pub elasticity: f32,
    pub hardness: f32,  // Mohs scale
    pub toughness: f32,
    pub brittleness: f32,
    
    // Combat properties
    pub edge_retention: f32,
    pub edge_sharpness: f32,
    pub blunt_force: f32,
    pub armor_penetration: f32,
    
    // Crafting properties
    pub malleability: f32,
    pub weldable: bool,
    pub forgeable: bool,
    pub castable: bool,
    pub requires_skill: u32,
    
    // Value and rarity
    pub value_multiplier: f32,
    pub rarity: f32,
    
    // Environmental properties
    pub corrosion_resistance: f32,
    pub electrical_conductivity: f32,
    
    // Special properties
    pub special_properties: SpecialProperties,
    
    // Alloy information
    pub alloy_of: Vec<String>,
    pub alloy_ratios: Vec<f32>,
    
    // Extraction danger (for rare materials like adamantine)
    pub extraction_danger: f32,
    
    // State transitions
    pub state_transitions: Vec<StateTransition>,
}

impl Default for LuaMaterial {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            category: MaterialCategory::Stone,
            density: 1000.0,
            melting_point: 1000.0,
            boiling_point: 2000.0,
            specific_heat: 500.0,
            thermal_conductivity: 1.0,
            yield_strength: 10000.0,
            tensile_strength: 15000.0,
            fracture_toughness: 20000.0,
            impact_resistance: 5000.0,
            shear_resistance: 8000.0,
            compressive_strength: 20000.0,
            elasticity: 0.3,
            hardness: 3.0,
            toughness: 0.5,
            brittleness: 0.5,
            edge_retention: 0.5,
            edge_sharpness: 0.5,
            blunt_force: 1.0,
            armor_penetration: 0.5,
            malleability: 0.5,
            weldable: false,
            forgeable: false,
            castable: false,
            requires_skill: 0,
            value_multiplier: 1.0,
            rarity: 0.5,
            corrosion_resistance: 0.5,
            electrical_conductivity: 0.0,
            special_properties: SpecialProperties::default(),
            alloy_of: Vec::new(),
            alloy_ratios: Vec::new(),
            extraction_danger: 0.0,
            state_transitions: Vec::new(),
        }
    }
}

/// Material registry for Lua-defined materials
#[derive(Resource, Default)]
pub struct MaterialRegistry {
    materials: HashMap<String, LuaMaterial>,
    categories: HashMap<MaterialCategory, Vec<String>>,
    alloys: HashMap<String, Vec<String>>,  // Alloy ID -> component IDs
}

impl MaterialRegistry {
    pub fn register(&mut self, material: LuaMaterial) {
        // Add to category index
        self.categories
            .entry(material.category.clone())
            .or_insert_with(Vec::new)
            .push(material.id.clone());
        
        // Track alloys
        if !material.alloy_of.is_empty() {
            self.alloys.insert(material.id.clone(), material.alloy_of.clone());
        }
        
        // Store material
        self.materials.insert(material.id.clone(), material);
    }
    
    pub fn get(&self, id: &str) -> Option<&LuaMaterial> {
        self.materials.get(id)
    }
    
    pub fn get_by_category(&self, category: MaterialCategory) -> Vec<&LuaMaterial> {
        self.categories
            .get(&category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.materials.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    pub fn get_alloy_components(&self, alloy_id: &str) -> Option<Vec<&LuaMaterial>> {
        self.alloys.get(alloy_id).map(|component_ids| {
            component_ids.iter()
                .filter_map(|id| self.materials.get(id))
                .collect()
        })
    }
    
    pub fn calculate_alloy_properties(&self, components: &[String], ratios: &[f32]) -> LuaMaterial {
        let mut alloy = LuaMaterial::default();
        
        // Weighted average of properties
        for (i, component_id) in components.iter().enumerate() {
            if let Some(mat) = self.materials.get(component_id) {
                let ratio = ratios.get(i).copied().unwrap_or(0.0);
                
                alloy.density += mat.density * ratio;
                alloy.melting_point += mat.melting_point * ratio;
                alloy.yield_strength += mat.yield_strength * ratio;
                alloy.tensile_strength += mat.tensile_strength * ratio;
                alloy.thermal_conductivity += mat.thermal_conductivity * ratio;
                alloy.hardness += mat.hardness * ratio;
            }
        }
        
        // Alloys are often stronger than their components
        alloy.yield_strength *= 1.2;
        alloy.tensile_strength *= 1.2;
        
        alloy
    }
    
    pub fn get_material_state(&self, id: &str, temperature: f32) -> MaterialState {
        if let Some(mat) = self.materials.get(id) {
            // Check state transitions
            for transition in &mat.state_transitions {
                if temperature >= transition.temperature {
                    return transition.new_state.clone();
                }
            }
            
            // Default state logic
            if temperature > mat.boiling_point {
                MaterialState::Gas
            } else if temperature > mat.melting_point {
                MaterialState::Liquid
            } else {
                MaterialState::Solid
            }
        } else {
            MaterialState::Solid
        }
    }
}

/// Command event to trigger material script reloading
#[derive(Event)]
pub struct ReloadMaterialScriptsCommand;

/// System to load material scripts on command
pub fn load_material_scripts(
    mut commands: Commands,
    mut reload_events: EventReader<ReloadMaterialScriptsCommand>,
    asset_server: Res<AssetServer>,
    existing_scripts: Query<Entity, With<MaterialScript>>,
) {
    // Only load when commanded
    if reload_events.is_empty() {
        return;
    }
    
    reload_events.clear();
    
    // Clear existing scripts
    for entity in existing_scripts.iter() {
        commands.entity(entity).despawn();
    }
    
    // Load material definition scripts
    let material_scripts = vec![
        "scripts/materials/materials.lua",
        "scripts/materials/alloys.lua",
        "scripts/materials/magical.lua",
    ];
    
    for script_path in material_scripts {
        commands.spawn((
            MaterialScript {
                script_path: script_path.to_string(),
                loaded: false,
            },
            ScriptCollection::<LuaScript>::default(),
        ));
        
        tracing::info!("Loading material script: {}", script_path);
    }
}

/// System to process loaded material scripts
pub fn process_material_scripts(
    mut scripts: Query<(Entity, &mut MaterialScript, &ScriptCollection<LuaScript>)>,
    mut material_registry: ResMut<MaterialRegistry>,
) {
    for (entity, mut script, collection) in scripts.iter_mut() {
        if !script.loaded && !collection.scripts.is_empty() {
            script.loaded = true;
            tracing::debug!("Material script loaded: {}", script.script_path);
        }
    }
}

/// Component for entities made of a specific material
#[derive(Component, Clone, Debug, Reflect)]
pub struct MaterialComponent {
    pub material_id: String,
    pub temperature: f32,  // Current temperature in Kelvin
    pub state: MaterialState,
    pub quality: f32,  // Quality modifier 0.0 to 2.0
}

impl MaterialComponent {
    pub fn new(material_id: String) -> Self {
        Self {
            material_id,
            temperature: 293.0,  // Room temperature
            state: MaterialState::Solid,
            quality: 1.0,
        }
    }
    
    pub fn heat(&mut self, amount: f32) {
        self.temperature += amount;
    }
    
    pub fn cool(&mut self, amount: f32) {
        self.temperature = (self.temperature - amount).max(0.0);
    }
}

/// System to update material states based on temperature
pub fn update_material_states(
    mut materials: Query<&mut MaterialComponent>,
    material_registry: Res<MaterialRegistry>,
) {
    for mut material in materials.iter_mut() {
        let new_state = material_registry.get_material_state(
            &material.material_id,
            material.temperature
        );
        
        if material.state != new_state {
            tracing::debug!(
                "Material {} changed state from {:?} to {:?}",
                material.material_id,
                material.state,
                new_state
            );
            material.state = new_state;
        }
    }
}

/// Calculate damage between materials (for combat)
pub fn calculate_material_damage(
    weapon_material: &LuaMaterial,
    armor_material: &LuaMaterial,
    attack_type: &str,
) -> f32 {
    let damage = match attack_type {
        "slash" => {
            let sharpness_factor = weapon_material.edge_sharpness;
            let hardness_ratio = weapon_material.hardness / armor_material.hardness;
            let strength_ratio = weapon_material.yield_strength / armor_material.yield_strength;
            
            sharpness_factor * hardness_ratio * strength_ratio
        },
        "pierce" => {
            let penetration = weapon_material.armor_penetration;
            let hardness_ratio = weapon_material.hardness / armor_material.hardness;
            let brittleness_penalty = armor_material.brittleness;
            
            penetration * hardness_ratio * (1.0 + brittleness_penalty)
        },
        "blunt" => {
            let force = weapon_material.blunt_force;
            let density_ratio = weapon_material.density / armor_material.density;
            let brittleness_bonus = armor_material.brittleness * 2.0;
            
            force * density_ratio * (1.0 + brittleness_bonus)
        },
        _ => 1.0,
    };
    
    damage.max(0.1)  // Minimum damage
}

/// Check material reactions
pub fn check_material_reaction(
    material: &LuaMaterial,
    temperature: f32,
    environment: &EnvironmentConditions,
) -> Vec<MaterialReaction> {
    let mut reactions = Vec::new();
    
    // Temperature reactions
    if material.special_properties.flammable {
        if let Some(ignition_point) = get_ignition_point(material) {
            if temperature > ignition_point {
                reactions.push(MaterialReaction::Burning);
            }
        }
    }
    
    if temperature > material.melting_point {
        reactions.push(MaterialReaction::Melting);
    }
    
    if temperature > material.boiling_point {
        reactions.push(MaterialReaction::Boiling);
    }
    
    // Environmental reactions
    if environment.has_water && material.special_properties.oxidizes {
        reactions.push(MaterialReaction::Oxidizing);
    }
    
    if environment.has_acid {
        reactions.push(MaterialReaction::Dissolving);
    }
    
    reactions
}

/// Material reactions
#[derive(Clone, Debug)]
pub enum MaterialReaction {
    Burning,
    Melting,
    Boiling,
    Oxidizing,
    Dissolving,
}

/// Environment conditions
#[derive(Resource, Clone, Debug, Default)]
pub struct EnvironmentConditions {
    pub has_water: bool,
    pub has_acid: bool,
    pub has_magma: bool,
    pub oxygen_level: f32,
}

fn get_ignition_point(material: &LuaMaterial) -> Option<f32> {
    // Estimate ignition point based on material properties
    if material.special_properties.flammable {
        match material.category {
            MaterialCategory::Wood => Some(573.0),
            MaterialCategory::Cloth => Some(523.0),
            MaterialCategory::Organic => Some(473.0),
            _ => None,
        }
    } else {
        None
    }
}

/// System to handle material reactions
pub fn material_reaction_system(
    mut materials: Query<(&MaterialComponent, &Transform, Entity)>,
    environment: Res<EnvironmentConditions>,
    material_registry: Res<MaterialRegistry>,
    mut commands: Commands,
) {
    for (material_comp, transform, entity) in materials.iter_mut() {
        if let Some(material) = material_registry.get(&material_comp.material_id) {
            let reactions = check_material_reaction(
                material,
                material_comp.temperature,
                &environment,
            );
            
            for reaction in reactions {
                match reaction {
                    MaterialReaction::Burning => {
                        // Add fire component
                        commands.entity(entity).insert(BurningComponent {
                            damage_per_second: 10.0,
                            spread_radius: 2.0,
                        });
                    },
                    MaterialReaction::Melting => {
                        // Change to liquid state
                        commands.entity(entity).insert(MeltedComponent);
                    },
                    MaterialReaction::Oxidizing => {
                        // Add rust/corrosion
                        commands.entity(entity).insert(CorrodedComponent {
                            corrosion_level: 0.1,
                        });
                    },
                    _ => {}
                }
            }
        }
    }
}

/// Marker components for material states
#[derive(Component)]
pub struct BurningComponent {
    pub damage_per_second: f32,
    pub spread_radius: f32,
}

#[derive(Component)]
pub struct MeltedComponent;

#[derive(Component)]
pub struct CorrodedComponent {
    pub corrosion_level: f32,
}