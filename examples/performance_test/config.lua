-- Performance Testing Configuration
-- This configuration demonstrates comprehensive performance testing
-- and benchmarking capabilities for the world-simulator

---@class PerformanceTestConfig
local config = {
    -- Test Configuration
    test = {
        -- Overall test settings
        duration_ticks = 1000,
        warmup_ticks = 100,
        cooldown_ticks = 100,
        random_seed = 42,

        -- Test phases
        phases = {
            {
                name = "baseline",
                duration_ticks = 200,
                entity_target = 10,
                ai_enabled = true,
                stress_level = 0.1,
                measurements = {"fps", "memory", "cpu", "entities"}
            },
            {
                name = "light_load",
                duration_ticks = 200,
                entity_target = 50,
                ai_enabled = true,
                stress_level = 0.3,
                measurements = {"fps", "entities", "system_times", "memory"}
            },
            {
                name = "medium_load",
                duration_ticks = 200,
                entity_target = 100,
                ai_enabled = true,
                stress_level = 0.6,
                measurements = {"fps", "memory", "bottlenecks", "entities"}
            },
            {
                name = "heavy_load",
                duration_ticks = 200,
                entity_target = 200,
                ai_enabled = true,
                stress_level = 1.0,
                measurements = {"fps", "cpu", "scalability", "memory"}
            },
            {
                name = "stress_test",
                duration_ticks = 200,
                entity_target = 500,
                ai_enabled = true,
                stress_level = 1.5,
                measurements = {"memory", "bottlenecks", "stability", "entities"}
            }
        },

        -- Load testing configuration
        load_testing = {
            max_entities = 500,
            spawn_rate = 0.05,
            despawn_rate = 0.02,
            stress_intervals = {50, 100, 200, 300, 500},
            measurement_intervals = {10, 50, 100, 200},
            burst_tests = {
                {
                    name = "entity_burst",
                    burst_size = 100,
                    burst_interval = 50,
                    recovery_time = 100
                },
                {
                    name = "ai_burst",
                    burst_size = 50,
                    burst_interval = 30,
                    recovery_time = 70
                }
            }
        },

        -- Performance thresholds
        thresholds = {
            fps_min = 30.0,
            fps_target = 60.0,
            frame_time_max_ms = 33.33,
            frame_time_target_ms = 16.67,
            memory_max_mb = 1000.0,
            memory_warning_mb = 500.0,
            cpu_max_percent = 90.0,
            cpu_warning_percent = 70.0
        }
    },

    -- Metrics Collection
    metrics = {
        -- Frame metrics
        frame = {
            enabled = true,
            sample_rate = 1.0,
            buffer_size = 1000,
            measurements = {
                "frame_time",
                "fps",
                "delta_time",
                "frame_consistency"
            }
        },

        -- Memory metrics
        memory = {
            enabled = true,
            sample_rate = 0.1,
            buffer_size = 500,
            measurements = {
                "total_memory_mb",
                "used_memory_mb",
                "free_memory_mb",
                "allocation_rate",
                "gc_time_ms",
                "gc_frequency",
                "memory_fragmentation"
            },
            track_allocations = true,
            track_deallocations = true,
            allocation_tracking_level = "detailed"
        },

        -- CPU metrics
        cpu = {
            enabled = true,
            sample_rate = 0.2,
            buffer_size = 500,
            measurements = {
                "cpu_usage_percent",
                "system_cpu_percent",
                "user_cpu_percent",
                "cpu_time_per_system",
                "thread_count",
                "context_switches"
            }
        },

        -- Entity metrics
        entities = {
            enabled = true,
            sample_rate = 0.5,
            buffer_size = 1000,
            measurements = {
                "total_entities",
                "entities_per_type",
                "spawn_rate",
                "despawn_rate",
                "entity_lifecycle_time",
                "entity_memory_usage"
            }
        },

        -- System metrics
        systems = {
            enabled = true,
            sample_rate = 1.0,
            buffer_size = 1000,
            track_all_systems = true,
            measurements = {
                "system_time_ms",
                "system_calls_per_frame",
                "system_memory_usage",
                "system_cpu_usage"
            },
            critical_systems = {
                "movement_system",
                "ai_system",
                "rendering_system",
                "physics_system",
                "resource_system"
            }
        },

        -- AI metrics
        ai = {
            enabled = true,
            sample_rate = 0.3,
            buffer_size = 500,
            measurements = {
                "ai_entities_count",
                "ai_decision_time_ms",
                "ai_pathfinding_time_ms",
                "ai_plan_complexity",
                "ai_success_rate",
                "ai_failure_analysis"
            },
            track_per_ai_type = true,
            ai_types = {"goap", "utility", "state_machine"}
        },

        -- Network metrics (if applicable)
        network = {
            enabled = false,
            sample_rate = 0.1,
            buffer_size = 500,
            measurements = {
                "bandwidth_usage",
                "latency_ms",
                "packet_loss",
                "connection_count",
                "message_queue_size"
            }
        }
    },

    -- Profiling Configuration
    profiling = {
        -- CPU profiling
        cpu = {
            enabled = true,
            sampling_rate = 100, -- Hz
            max_samples = 100000,
            thread_filter = "all",
            include_stacks = true,
            stack_depth = 32
        },

        -- Memory profiling
        memory = {
            enabled = true,
            track_allocations = true,
            allocation_stack_depth = 16,
            snapshot_interval = 1000,
            max_snapshots = 10,
            diff_snapshots = true
        },

        -- System profiling
        systems = {
            enabled = true,
            profile_all_systems = true,
            min_system_time_ms = 0.1,
            max_system_time_ms = 100.0,
            profile_hierarchy = true
        }
    },

    -- Benchmark Configuration
    benchmarks = {
        -- Individual benchmarks
        entity_spawn = {
            enabled = true,
            iterations = 1000,
            warmup_iterations = 100,
            entity_types = {"peasant", "building", "resource"},
            measure = {"time", "memory", "cpu"}
        },

        entity_movement = {
            enabled = true,
            iterations = 5000,
            warmup_iterations = 500,
            entity_counts = {10, 50, 100, 200, 500},
            path_complexity = {"simple", "medium", "complex"},
            measure = {"time", "path_length", "cpu"}
        },

        ai_processing = {
            enabled = true,
            iterations = 1000,
            warmup_iterations = 100,
            ai_types = {"goap", "utility", "state_machine"},
            entity_counts = {10, 25, 50, 100},
            measure = {"time", "memory", "success_rate"}
        },

        resource_gathering = {
            enabled = true,
            iterations = 500,
            warmup_iterations = 50,
            resource_types = {"wood", "stone", "food"},
            gatherer_counts = {1, 5, 10, 20},
            measure = {"time", "resources_per_second", "efficiency"}
        },

        memory_operations = {
            enabled = true,
            iterations = 10000,
            warmup_iterations = 1000,
            operation_types = {"allocation", "deallocation", "copy", "move"},
            data_sizes = {1, 10, 100, 1000}, -- KB
            measure = {"time", "throughput", "memory_usage"}
        }
    },

    -- Scalability Testing
    scalability = {
        -- Entity scaling tests
        entity_scaling = {
            enabled = true,
            start_entities = 10,
            max_entities = 1000,
            step_size = 50,
            duration_per_step = 100,
            metrics = {"fps", "memory", "cpu", "entity_spawns_per_second"}
        },

        -- World size scaling
        world_scaling = {
            enabled = true,
            world_sizes = {
                {width = 16, height = 16},
                {width = 32, height = 32},
                {width = 64, height = 64},
                {width = 128, height = 128},
                {width = 256, height = 256}
            },
            entity_density = 0.1,
            metrics = {"fps", "memory", "loading_time", "cpu"}
        },

        -- AI complexity scaling
        ai_scaling = {
            enabled = true,
            ai_entity_counts = {10, 25, 50, 100, 200},
            ai_complexity_levels = {"simple", "medium", "complex"},
            metrics = {"ai_time", "fps", "memory", "decision_quality"}
        }
    },

    -- Stress Testing
    stress_testing = {
        -- Memory stress
        memory_stress = {
            enabled = true,
            max_memory_mb = 2000,
            memory_increment_mb = 100,
            stress_duration = 100,
            recovery_duration = 50,
            measure = {"memory_usage", "gc_time", "fps", "stability"}
        },

        -- CPU stress
        cpu_stress = {
            enabled = true,
            max_cpu_percent = 95,
            cpu_increment = 10,
            stress_duration = 100,
            recovery_duration = 50,
            measure = {"cpu_usage", "fps", "temperature", "stability"}
        },

        -- Entity stress
        entity_stress = {
            enabled = true,
            spawn_rate = 0.1,
            max_entities = 1000,
            stress_duration = 200,
            measure = {"entity_count", "fps", "memory", "spawn_success_rate"}
        }
    },

    -- Bottleneck Detection
    bottleneck_detection = {
        enabled = true,
        detection_interval = 100,
        thresholds = {
            frame_time = 16.67,
            memory_usage = 500,
            cpu_usage = 70,
            system_time = 5.0,
            ai_time = 2.0
        },
        analysis = {
            correlation_analysis = true,
            trend_analysis = true,
            statistical_analysis = true,
            machine_learning_analysis = false
        },
        reporting = {
            detailed_reports = true,
            include_recommendations = true,
            severity_levels = {"low", "medium", "high", "critical"}
        }
    },

    -- Reporting Configuration
    reporting = {
        -- Real-time reporting
        realtime = {
            enabled = true,
            update_interval = 100,
            console_output = true,
            file_output = true,
            metrics_to_show = {"fps", "memory", "cpu", "entities"}
        },

        -- Final report
        final_report = {
            enabled = true,
            include_summary = true,
            include_detailed_metrics = true,
            include_bottlenecks = true,
            include_recommendations = true,
            include_scalability_analysis = true,
            output_formats = {"console", "json", "csv", "html"}
        },

        -- Data export
        export = {
            enabled = true,
            formats = {"json", "csv", "parquet"},
            compression = true,
            include_raw_data = false,
            include_aggregated_data = true,
            export_interval = 1000
        }
    },

    -- Visualization Configuration
    visualization = {
        enabled = false, -- Disabled for performance testing
        show_debug_info = true,
        show_performance_overlay = true,
        show_metrics_graphs = true,
        graph_update_interval = 100,
        metrics_to_graph = {"fps", "memory", "cpu", "entities"}
    },

    -- Development Configuration
    development = {
        debug = {
            enabled = true,
            log_level = "debug",
            verbose_logging = true,
            print_metrics = true,
            print_bottlenecks = true
        },

        hot_reload = {
            enabled = true,
            watch_config = true,
            watch_code = false,
            reload_interval = 2
        },

        validation = {
            enabled = true,
            validate_metrics = true,
            validate_results = true,
            cross_reference_results = true
        }
    }
}

-- Return the configuration
return config