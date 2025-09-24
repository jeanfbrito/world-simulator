-- Modding Example Configuration
-- This configuration demonstrates the modding and customization capabilities
-- of the world-simulator, including custom entities, components, and systems

---@class ModdingExampleConfig
local config = {
    -- Mod Configuration
    mods = {
        -- Mod discovery and loading
        mod_directory = "mods/",
        auto_load = true,
        load_order = {
            "core_mods",
            "gameplay_mods",
            "content_mods",
            "ui_mods"
        },

        -- Mod compatibility
        compatibility_mode = "strict",
        version_checking = true,
        dependency_resolution = true,
        conflict_resolution = "manual",

        -- Mod management
        enable_hot_reload = true,
        reload_interval = 2.0,
        save_mod_settings = true,
        mod_settings_file = "mod_settings.json"
    },

    -- Core Mods for Example
    core_mods = {
        magic_system = {
            enabled = true,
            version = "1.0.0",
            description = "Adds magic spells and mana system",
            dependencies = {},
            load_order = 1,

            -- Magic system configuration
            magic = {
                -- Mana system
                mana = {
                    base_mana = 100,
                    mana_regen_rate = 0.5,
                    max_mana_multiplier = 2.0,
                    mana_efficiency = 1.0
                },

                -- Spells
                spells = {
                    fireball = {
                        mana_cost = 20,
                        damage = 50,
                        range = 10,
                        cooldown = 3.0,
                        effects = {"burn", "knockback"}
                    },
                    heal = {
                        mana_cost = 30,
                        healing_amount = 75,
                        range = 5,
                        cooldown = 5.0,
                        effects = {"heal", "regeneration"}
                    },
                    shield = {
                        mana_cost = 25,
                        shield_amount = 100,
                        duration = 10.0,
                        cooldown = 8.0,
                        effects = {"protection", "reflection"}
                    },
                    teleport = {
                        mana_cost = 40,
                        range = 15,
                        cooldown = 10.0,
                        effects = {"instant_movement", "disorientation"}
                    }
                },

                -- Magic entities
                entities = {
                    mage = {
                        base_health = 80,
                        base_energy = 120,
                        mana_multiplier = 1.5,
                        spell_power_multiplier = 1.2,
                        learnable_spells = {"fireball", "heal", "shield"}
                    },
                    mana_crystal = {
                        mana_amount = 200,
                        regeneration_rate = 1.0,
                        spawn_chance = 0.1,
                        biomes = {"forest", "mountains"}
                    }
                },

                -- Magic resources
                resources = {
                    mana_essence = {
                        max_amount = 50,
                        regeneration_rate = 0.1,
                        mana_value = 25,
                        rarity = "uncommon"
                    },
                    arcane_crystal = {
                        max_amount = 20,
                        regeneration_rate = 0.02,
                        mana_value = 100,
                        rarity = "rare"
                    }
                }
            }
        },

        technology_mod = {
            enabled = true,
            version = "1.2.0",
            description = "Adds research and technology progression",
            dependencies = {},
            load_order = 2,

            -- Technology tree configuration
            technology = {
                -- Research system
                research = {
                    base_research_rate = 1.0,
                    research_efficiency_bonus = 0.1,
                    max_research_projects = 3,
                    research_points_per_tick = 0.1
                },

                -- Technology tree
                tech_tree = {
                    basic_science = {
                        cost = {research_points = 50},
                        unlocks = {"advanced_science", "basic_engineering"},
                        requirements = {},
                        description = "Foundation of scientific research"
                    },
                    advanced_science = {
                        cost = {research_points = 150},
                        unlocks = {"theoretical_physics", "applied_sciences"},
                        requirements = {"basic_science"},
                        description = "Advanced scientific principles and theories"
                    },
                    basic_engineering = {
                        cost = {research_points = 100},
                        unlocks = {"mechanical_engineering", "civil_engineering"},
                        requirements = {"basic_science"},
                        description = "Basic engineering principles and techniques"
                    },
                    theoretical_physics = {
                        cost = {research_points = 300},
                        unlocks = {"quantum_mechanics", "particle_physics"},
                        requirements = {"advanced_science"},
                        description = "Theoretical physics and advanced mathematics"
                    },
                    mechanical_engineering = {
                        cost = {research_points = 200},
                        unlocks = {"robotics", "advanced_machinery"},
                        requirements = {"basic_engineering"},
                        description = "Mechanical systems and automation"
                    }
                },

                -- Technology entities
                entities = {
                    scientist = {
                        base_health = 70,
                        base_energy = 100,
                        research_efficiency = 1.2,
                        max_research_projects = 2,
                        specializations = {"theoretical", "experimental", "applied"}
                    },
                    laboratory = {
                        health = 200,
                        research_bonus = 0.5,
                        max_scientists = 4,
                        construction_cost = {wood = 50, stone = 100}
                    },
                    research_computer = {
                        health = 150,
                        processing_power = 2.0,
                        energy_consumption = 5,
                        construction_cost = {stone = 30, ore = 20}
                    }
                },

                -- Technology resources
                resources = {
                    research_data = {
                        max_amount = 100,
                        research_value = 10,
                        generation_rate = 0.05,
                        requires = "laboratory"
                    },
                    experimental_materials = {
                        max_amount = 30,
                        research_value = 50,
                        generation_rate = 0.01,
                        requires = "advanced_science"
                    }
                }
            }
        },

        quest_system = {
            enabled = true,
            version = "0.9.0",
            description = "Adds quests and objectives for entities",
            dependencies = {"magic_system"},
            load_order = 3,

            -- Quest system configuration
            quests = {
                -- Quest generation
                generation = {
                    max_active_quests = 5,
                    quest_refresh_interval = 100,
                    difficulty_scaling = true,
                    personalization = true
                },

                -- Quest types
                quest_types = {
                    -- Collection quests
                    collection = {
                        objectives = {"collect", "gather", "acquire"},
                        rewards = {experience = 50, resources = true},
                        time_limit = 500,
                        difficulty = "easy"
                    },
                    -- Combat quests
                    combat = {
                        objectives = {"defeat", "eliminate", "clear"},
                        rewards = {experience = 100, items = true},
                        time_limit = 300,
                        difficulty = "medium"
                    },
                    -- Exploration quests
                    exploration = {
                        objectives = {"explore", "discover", "survey"},
                        rewards = {experience = 75, reputation = true},
                        time_limit = 800,
                        difficulty = "medium"
                    },
                    -- Construction quests
                    construction = {
                        objectives = {"build", "construct", "create"},
                        rewards = {experience = 80, resources = true},
                        time_limit = 600,
                        difficulty = "hard"
                    },
                    -- Research quests
                    research = {
                        objectives = {"research", "study", "analyze"},
                        rewards = {experience = 120, technology = true},
                        time_limit = 1000,
                        difficulty = "hard"
                    }
                },

                -- Quest entities
                entities = {
                    quest_giver = {
                        base_health = 60,
                        max_quests = 3,
                        quest_refresh_rate = 200,
                        reputation_influence = 1.5
                    },
                    adventurer = {
                        base_health = 90,
                        base_energy = 110,
                        quest_completion_bonus = 0.2,
                        special_abilities = {"tracking", "negotiation", "combat"}
                    }
                },

                -- Quest rewards
                rewards = {
                    experience = {
                        base_amount = 50,
                        scaling_factor = 1.5,
                        difficulty_multiplier = 2.0
                    },
                    reputation = {
                        factions = {"mages_guild", "science_council", "merchants_guild"},
                        reputation_range = {-100, 100},
                        decay_rate = 0.01
                    },
                    special_rewards = {
                        unique_items = true,
                        technology_unlocks = true,
                        permanent_bonuses = true
                    }
                }
            }
        }
    },

    -- Custom Components Configuration
    components = {
        -- Magic components
        magic = {
            component_type = "MagicComponent",
            fields = {
                mana = {type = "f32", default = 100.0, min = 0.0, max = 1000.0},
                max_mana = {type = "f32", default = 100.0, min = 1.0},
                mana_regen_rate = {type = "f32", default = 0.5, min = 0.0},
                spells = {type = "array<string>", default = {}},
                spell_power = {type = "f32", default = 1.0, min = 0.1}
            },
            update_system = "magic_system",
            save_data = true,
            network_sync = true
        },

        -- Technology components
        technology = {
            component_type = "TechnologyComponent",
            fields = {
                research_points = {type = "u32", default = 0, min = 0},
                unlocked_technologies = {type = "array<string>", default = {}},
                current_research = {type = "string", default = nil},
                research_efficiency = {type = "f32", default = 1.0, min = 0.1}
            },
            update_system = "technology_system",
            save_data = true,
            network_sync = true
        },

        -- Quest components
        quest = {
            component_type = "QuestComponent",
            fields = {
                active_quests = {type = "array<Quest>", default = {}},
                completed_quests = {type = "array<string>", default = {}},
                quest_progress = {type = "map<string, f32>", default = {}},
                reputation = {type = "map<string, f32>", default = {}}
            },
            update_system = "quest_system",
            save_data = true,
            network_sync = true
        },

        -- Custom AI components
        custom_ai = {
            component_type = "CustomAIComponent",
            fields = {
                personality_type = {type = "string", default = "balanced"},
                behavior_weights = {type = "map<string, f32>", default = {}},
                memory = {type = "array<AIMemory>", default = {}},
                learning_rate = {type = "f32", default = 0.5, min = 0.0, max = 1.0}
            },
            update_system = "custom_ai_system",
            save_data = true,
            network_sync = false
        }
    },

    -- Custom Systems Configuration
    systems = {
        -- Magic system
        magic_system = {
            system_type = "update",
            update_interval = 0.1,
            priority = 100,
            parallel = true,
            query_requirements = {
                required_components = {"MagicComponent"},
                optional_components = {"UnitStats", "PositionComponent"}
            },
            operations = {
                "mana_regeneration",
                "spell_cooldown_update",
                "magic_effect_processing",
                "spell_casting_validation"
            }
        },

        -- Technology system
        technology_system = {
            system_type = "update",
            update_interval = 1.0,
            priority = 50,
            parallel = true,
            query_requirements = {
                required_components = {"TechnologyComponent"},
                optional_components = {"UnitStats", "BuildingComponent"}
            },
            operations = {
                "research_progress_update",
                "technology_unlock_check",
                "research_efficiency_calculation",
                "technology_bonus_application"
            }
        },

        -- Quest system
        quest_system = {
            system_type = "update",
            update_interval = 0.5,
            priority = 75,
            parallel = false,
            query_requirements = {
                required_components = {"QuestComponent"},
                optional_components = {"UnitStats", "PositionComponent"}
            },
            operations = {
                "quest_progress_update",
                "quest_completion_check",
                "quest_generation",
                "reward_distribution"
            }
        },

        -- Custom AI system
        custom_ai_system = {
            system_type = "update",
            update_interval = 0.2,
            priority = 80,
            parallel = true,
            query_requirements = {
                required_components = {"CustomAIComponent"},
                optional_components = {"UnitStats", "PositionComponent", "GoapAgent"}
            },
            operations = {
                "personality_behavior_update",
                "memory_processing",
                "learning_adaptation",
                "decision_making"
            }
        }
    },

    -- Events Configuration
    events = {
        -- Magic events
        spell_cast = {
            event_type = "action",
            parameters = {
                caster = {type = "entity", required = true},
                spell = {type = "string", required = true},
                target = {type = "entity", required = false},
                position = {type = "vector2", required = false}
            },
            handlers = {
                "apply_spell_effects",
                "consume_mana",
                "trigger_spell_cooldown",
                "log_spell_cast"
            }
        },

        mana_changed = {
            event_type = "state_change",
            parameters = {
                entity = {type = "entity", required = true},
                old_value = {type = "f32", required = true},
                new_value = {type = "f32", required = true},
                change_source = {type = "string", required = false}
            },
            handlers = {
                "update_ui_mana_bar",
                "check_mana_thresholds",
                "trigger_mana_based_abilities"
            }
        },

        -- Technology events
        technology_unlocked = {
            event_type = "achievement",
            parameters = {
                technology = {type = "string", required = true},
                researching_entity = {type = "entity", required = true},
                research_time = {type = "u32", required = true}
            },
            handlers = {
                "apply_technology_bonuses",
                "unlock_new_entities",
                "update_technology_tree_ui",
                "trigger_technology_quests"
            }
        },

        research_completed = {
            event_type = "progress",
            parameters = {
                research_project = {type = "string", required = true},
                contribution = {type = "u32", required = true},
                total_contribution = {type = "u32", required = true}
            },
            handlers = {
                "update_research_progress",
                "award_research_experience",
                "check_research_milestones"
            }
        },

        -- Quest events
        quest_accepted = {
            event_type = "interaction",
            parameters = {
                quest = {type = "Quest", required = true},
                accepting_entity = {type = "entity", required = true},
                quest_giver = {type = "entity", required = true}
            },
            handlers = {
                "add_quest_to_active",
                "update_quest_giver_state",
                "trigger_quest_start_events",
                "update_quest_ui"
            }
        },

        quest_completed = {
            event_type = "achievement",
            parameters = {
                quest = {type = "Quest", required = true},
                completing_entity = {type = "entity", required = true},
                completion_time = {type = "u32", required = true}
            },
            handlers = {
                "distribute_quest_rewards",
                "update_reputation",
                "unlock_follow_up_quests",
                "trigger_achievement_events"
            }
        }
    },

    -- UI Configuration
    ui = {
        -- Magic UI
        magic_ui = {
            enabled = true,
            elements = {
                mana_bar = {
                    type = "progress_bar",
                    position = {x = 10, y = 50},
                    size = {width = 200, height = 20},
                    color = {r = 0.0, g = 0.5, b = 1.0, a = 1.0},
                    binding = "selected_entity.MagicComponent.mana / selected_entity.MagicComponent.max_mana"
                },
                spell_panel = {
                    type = "panel",
                    position = {x = 10, y = 80},
                    size = {width = 300, height = 200},
                    visible = "selected_entity.has_component('MagicComponent')"
                },
                spell_buttons = {
                    type = "button_grid",
                    position = {x = 15, y = 85},
                    size = {width = 290, height = 190},
                    binding = "selected_entity.MagicComponent.spells",
                    on_click = "cast_spell"
                }
            }
        },

        -- Technology UI
        technology_ui = {
            enabled = true,
            elements = {
                tech_tree = {
                    type = "tree_view",
                    position = {x = 320, y = 10},
                    size = {width = 400, height = 300},
                    binding = "game_state.technology_tree",
                    on_node_click = "start_research"
                },
                research_progress = {
                    type = "progress_panel",
                    position = {x = 320, y = 320},
                    size = {width = 400, height = 100},
                    binding = "selected_entity.TechnologyComponent.current_research"
                }
            }
        },

        -- Quest UI
        quest_ui = {
            enabled = true,
            elements = {
                quest_log = {
                    type = "scroll_panel",
                    position = {x = 730, y = 10},
                    size = {width = 250, height = 400},
                    binding = "selected_entity.QuestComponent.active_quests"
                },
                quest_tracker = {
                    type = "minimap_overlay",
                    position = {x = 0, y = 0},
                    size = {width = 200, height = 200},
                    binding = "game_state.active_quest_objectives"
                }
            }
        }
    },

    -- Simulation Configuration
    simulation = {
        tick_rate = 30,
        max_ticks = 400,
        headless = false,

        -- World settings
        world = {
            width = 32,
            height = 32,
            seed = 42,
            resource_density = 0.3
        },

        -- Mod-specific settings
        mod_settings = {
            magic_enabled = true,
            technology_enabled = true,
            quest_enabled = true,
            mod_interaction = true,
            cross_mod_events = true
        }
    },

    -- Development Configuration
    development = {
        debug = {
            enabled = true,
            log_level = "debug",
            verbose_logging = true,
            print_mod_events = true,
            print_system_updates = true
        },

        testing = {
            enable_mod_tests = true,
            test_mod_compatibility = true,
            test_mod_performance = true,
            generate_test_reports = true
        },

        hot_reload = {
            enabled = true,
            watch_files = {"**/*.lua", "**/*.json", "**/*.toml"},
            reload_interval = 1.0,
            preserve_state = true
        }
    }
}

-- Return the configuration
return config