//! End-to-end tests for the world simulator
//!
//! This module contains comprehensive end-to-end tests that verify
//! the complete simulation workflow under various conditions.

mod simulation_workflow;
mod deployment_scenarios;

pub mod common;

// Re-export common test utilities
pub use common::*;

/// Test runner configuration for end-to-end tests
pub struct E2ETestConfig {
    pub simulation_duration: std::time::Duration,
    pub enable_websocket: bool,
    pub enable_persistence: bool,
    pub enable_monitoring: bool,
    pub cleanup_on_complete: bool,
}

impl Default for E2ETestConfig {
    fn default() -> Self {
        Self {
            simulation_duration: std::time::Duration::from_secs(30),
            enable_websocket: true,
            enable_persistence: true,
            enable_monitoring: true,
            cleanup_on_complete: true,
        }
    }
}

/// Helper macro for running end-to-end tests with different configurations
macro_rules! e2e_test {
    ($name:ident, $config:expr, $body:block) => {
        #[tokio::test]
        async fn $name() {
            let config = $config;
            let mut context = common::setup_test_environment().await;

            // Setup test infrastructure
            if config.enable_websocket {
                let ws_handle = context.start_websocket_server().await;
                assert!(ws_handle.is_ok(), "Failed to start WebSocket server");
            }

            if config.enable_persistence {
                context.enable_persistence().await;
            }

            if config.enable_monitoring {
                context.start_monitoring().await;
            }

            // Execute test body
            let result = async move $body.await;

            // Cleanup
            if config.cleanup_on_complete {
                context.cleanup().await;
            }

            result
        }
    };
}

// Export the macro for use in other test modules
pub(crate) use e2e_test;