-- WebSocket Client Example Configuration
-- This configuration demonstrates real-time monitoring and control features
-- for the world-simulator through WebSocket connections

---@class WebSocketClientConfig
local config = {
    -- WebSocket Server Configuration
    websocket = {
        -- Server settings
        host = "localhost",
        port = 8080,
        path = "/ws",

        -- Protocol settings
        protocol = "ws_v1",
        subprotocols = {"world-simulator-v1"},

        -- Connection settings
        max_connections = 100,
        connection_timeout = 30, -- seconds
        heartbeat_interval = 15, -- seconds
        idle_timeout = 300, -- seconds

        -- Message settings
        max_message_size = 1048576, -- 1MB
        message_queue_size = 1000,
        batch_messages = true,
        batch_size = 10,
        batch_interval = 0.1, -- seconds

        -- Security settings
        enable_cors = true,
        allowed_origins = {"http://localhost:3000", "http://127.0.0.1:3000"},
        enable_auth = false,
        auth_timeout = 10, -- seconds

        -- Performance settings
        enable_compression = true,
        compression_threshold = 1024, -- bytes
        enable_metrics = true,
        metrics_interval = 60, -- seconds
    },

    -- Data Streams Configuration
    data_streams = {
        -- Simulation state stream
        simulation_state = {
            enabled = true,
            update_interval = 100, -- milliseconds
            buffer_size = 100,
            compression = true,
            include_fields = {
                "tick",
                "resources",
                "units",
                "buildings",
                "fps",
                "delta_time"
            },
            exclude_fields = {},
            transform_functions = {
                timestamp = "iso8601"
            }
        },

        -- Entity updates stream
        entity_updates = {
            enabled = true,
            update_interval = 50, -- milliseconds
            buffer_size = 500,
            compression = true,
            batch_size = 20,
            include_fields = {
                "id",
                "position",
                "health",
                "energy",
                "inventory",
                "state"
            },
            exclude_fields = {
                "path",
                "internal_state"
            },
            filters = {
                changed_only = true,
                visible_only = true,
                range_limit = 50
            }
        },

        -- Performance metrics stream
        performance_metrics = {
            enabled = true,
            update_interval = 1000, -- milliseconds
            buffer_size = 60,
            include_metrics = {
                "tps",
                "memory_mb",
                "cpu_usage",
                "active_connections",
                "messages_per_second",
                "average_latency",
                "error_rate"
            },
            aggregation = {
                window_size = 60, -- seconds
                functions = {
                    "avg",
                    "max",
                    "min",
                    "sum"
                }
            }
        },

        -- World events stream
        world_events = {
            enabled = true,
            update_interval = 0, -- real-time
            buffer_size = 1000,
            event_types = {
                "entity_spawned",
                "entity_destroyed",
                "resource_harvested",
                "building_completed",
                "unit_died",
                "technology_researched"
            },
            filters = {
                importance = {"high", "medium"},
                range_limit = 100
            }
        },

        -- Debug stream
        debug_stream = {
            enabled = false, -- disabled by default
            update_interval = 1000,
            buffer_size = 100,
            include_debug_info = {
                "system_states",
                "entity_states",
                "performance_counters",
                "memory_usage",
                "error_logs"
            }
        }
    },

    -- Client Management
    client_management = {
        -- Authentication
        authentication = {
            enabled = false,
            method = "token", -- token, jwt, basic
            token_expiry = 3600, -- seconds
            refresh_interval = 300, -- seconds
            max_failed_attempts = 5,
            lockout_duration = 300, -- seconds
        },

        -- Permissions
        permissions = {
            roles = {
                viewer = {
                    can_read = true,
                    can_write = false,
                    can_control = false,
                    streams = {"simulation_state", "performance_metrics"}
                },
                operator = {
                    can_read = true,
                    can_write = true,
                    can_control = true,
                    streams = {"simulation_state", "entity_updates", "performance_metrics"}
                },
                admin = {
                    can_read = true,
                    can_write = true,
                    can_control = true,
                    streams = {"all"}
                }
            },
            default_role = "viewer"
        },

        -- Rate limiting
        rate_limiting = {
            enabled = true,
            requests_per_minute = 60,
            burst_size = 10,
            window_size = 60, -- seconds
            penalty_duration = 300, -- seconds
        },

        -- Session management
        sessions = {
            max_sessions_per_client = 3,
            session_timeout = 3600, -- seconds
            cleanup_interval = 300, -- seconds
            persist_sessions = false
        }
    },

    -- Message Processing
    message_processing = {
        -- Command handlers
        commands = {
            spawn_entity = {
                required_params = {"entity_type", "position"},
                optional_params = {"owner", "properties"},
                validation = {
                    position_range = {0, 100},
                    entity_types = {"peasant", "building", "resource"}
                },
                rate_limit = 10, -- per minute
                permission_level = "operator"
            },

            set_speed = {
                required_params = {"speed"},
                optional_params = {"duration"},
                validation = {
                    speed_range = {0.1, 10.0},
                    duration_range = {1, 3600}
                },
                rate_limit = 30,
                permission_level = "operator"
            },

            pause_simulation = {
                required_params = {},
                optional_params = {"duration"},
                validation = {
                    duration_range = {1, 3600}
                },
                rate_limit = 10,
                permission_level = "operator"
            },

            query_state = {
                required_params = {"query_type"},
                optional_params = {"filters", "format"},
                validation = {
                    query_types = {"entities", "resources", "buildings", "performance"}
                },
                rate_limit = 60,
                permission_level = "viewer"
            }
        },

        -- Message validation
        validation = {
            strict_mode = true,
            sanitize_input = true,
            max_nesting_depth = 10,
            max_array_length = 1000,
            max_string_length = 1000
        },

        -- Error handling
        error_handling = {
            log_errors = true,
            send_error_responses = true,
            error_codes = {
                INVALID_FORMAT = 1000,
                INVALID_PARAMS = 1001,
                PERMISSION_DENIED = 2000,
                RATE_LIMITED = 2001,
                NOT_FOUND = 3000,
                INTERNAL_ERROR = 5000
            }
        }
    },

    -- Performance Monitoring
    performance_monitoring = {
        -- Metrics collection
        metrics = {
            connection_metrics = {
                active_connections = true,
                total_connections = true,
                connection_duration = true,
                disconnection_reasons = true
            },
            message_metrics = {
                messages_sent = true,
                messages_received = true,
                message_size_distribution = true,
                message_type_distribution = true
            },
            performance_metrics = {
                cpu_usage = true,
                memory_usage = true,
                latency_percentiles = true,
                throughput = true,
                error_rate = true
            }
        },

        -- Alerting
        alerts = {
            enabled = true,
            rules = {
                {
                    name = "high_latency",
                    condition = "latency_p95 > 1000",
                    threshold = 1000, -- ms
                    duration = 300, -- seconds
                    severity = "warning"
                },
                {
                    name = "high_error_rate",
                    condition = "error_rate > 0.05",
                    threshold = 0.05, -- 5%
                    duration = 60,
                    severity = "critical"
                },
                {
                    name = "low_memory",
                    condition = "memory_usage > 0.8",
                    threshold = 0.8, -- 80%
                    duration = 300,
                    severity = "warning"
                }
            },
            notification_channels = {"log", "console"}
        },

        -- Reporting
        reporting = {
            enabled = true,
            interval = 300, -- seconds
            retention_period = 86400, -- 24 hours
            formats = {"json", "csv"},
            export_destinations = {"logs", "metrics_endpoint"}
        }
    },

    -- Simulation Configuration
    simulation = {
        -- Basic settings
        tick_rate = 30,
        max_ticks = 500,
        headless = false,

        -- World settings
        world = {
            width = 32,
            height = 32,
            seed = 42,
            resource_density = 0.3
        },

        -- Entity settings
        entities = {
            initial_peasants = 5,
            spawn_rate = 0.02,
            max_entities = 100
        },

        -- Performance settings
        performance = {
            enable_batching = true,
            batch_size = 50,
            update_interval = 100, -- milliseconds
            enable_culling = true,
            culling_distance = 100
        }
    },

    -- UI Configuration
    ui = {
        -- Display settings
        show_webconsole = true,
        show_connection_status = true,
        show_performance_metrics = true,
        show_message_log = true,

        -- Console settings
        console = {
            max_lines = 1000,
            log_level = "info",
            timestamps = true,
            colors = true
        },

        -- Visualization settings
        visualization = {
            grid_enabled = true,
            entity_labels = true,
            performance_graph = true,
            connection_indicators = true
        }
    },

    -- Development Configuration
    development = {
        -- Debug settings
        debug = {
            enabled = true,
            log_level = "debug",
            verbose_logging = true,
            print_messages = true,
            print_commands = true
        },

        -- Testing settings
        testing = {
            mock_websocket = true,
            simulated_clients = 3,
            test_commands = true,
            performance_test = true
        },

        -- Hot reload
        hot_reload = {
            enabled = true,
            watch_files = {"config.lua", "*.rs"},
            reload_interval = 2, -- seconds
            preserve_state = true
        }
    }
}

-- Return the configuration
return config