//! Common utilities for end-to-end tests
//!
//! This module provides shared utilities for setting up and managing
//! end-to-end test environments.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use world_sim_simple::simulation::Simulation;
use world_sim_simple::ipc::IPCMessage;

use crate::common::{TestContext, TestConfig};

/// Extended test context for end-to-end tests
pub struct E2ETestContext {
    pub inner: TestContext,
    pub websocket_server: Option<Arc<RwLock<MockWebSocketServer>>>,
    pub monitoring_enabled: bool,
    pub persistence_enabled: bool,
    pub test_metrics: TestMetrics,
}

/// Mock WebSocket server for testing
pub struct MockWebSocketServer {
    pub connected_clients: usize,
    pub messages_received: Vec<IPCMessage>,
    pub messages_sent: Vec<IPCMessage>,
}

impl MockWebSocketServer {
    pub fn new() -> Self {
        Self {
            connected_clients: 0,
            messages_received: Vec::new(),
            messages_sent: Vec::new(),
        }
    }

    pub async fn handle_client(&mut self) -> MockWebSocketClient {
        self.connected_clients += 1;
        MockWebSocketClient::new(self)
    }

    pub async fn broadcast(&mut self, message: IPCMessage) {
        self.messages_sent.push(message);
    }
}

/// Mock WebSocket client for testing
pub struct MockWebSocketClient {
    server: Arc<RwLock<MockWebSocketServer>>,
    received_messages: Vec<IPCMessage>,
}

impl MockWebSocketClient {
    pub fn new(server: &MockWebSocketServer) -> Self {
        Self {
            server: Arc::new(RwLock::new(MockWebSocketServer::new())),
            received_messages: Vec::new(),
        }
    }

    pub async fn send(&mut self, message: IPCMessage) {
        let mut server = self.server.write().await;
        server.messages_received.push(message);
    }

    pub async fn try_recv(&mut self) -> Option<IPCMessage> {
        if self.received_messages.is_empty() {
            None
        } else {
            Some(self.received_messages.remove(0))
        }
    }

    pub async fn disconnect(&mut self) {
        let mut server = self.server.write().await;
        server.connected_clients = server.connected_clients.saturating_sub(1);
    }
}

/// Test metrics collection
#[derive(Debug, Clone)]
pub struct TestMetrics {
    pub start_time: std::time::Instant,
    pub tick_count: u32,
    pub total_entities: usize,
    pub total_resources: usize,
    pub errors_encountered: u32,
    pub performance_metrics: HashMap<String, f64>,
}

impl TestMetrics {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            tick_count: 0,
            total_entities: 0,
            total_resources: 0,
            errors_encountered: 0,
            performance_metrics: HashMap::new(),
        }
    }

    pub fn record_tick(&mut self) {
        self.tick_count += 1;
    }

    pub fn record_entities(&mut self, count: usize) {
        self.total_entities = count;
    }

    pub fn record_resources(&mut self, count: usize) {
        self.total_resources = count;
    }

    pub fn record_error(&mut self) {
        self.errors_encountered += 1;
    }

    pub fn record_performance(&mut self, metric_name: &str, value: f64) {
        self.performance_metrics.insert(metric_name.to_string(), value);
    }

    pub fn get_summary(&self) -> String {
        format!(
            "Test Summary: {} ticks, {} entities, {} resources, {} errors",
            self.tick_count, self.total_entities, self.total_resources, self.errors_encountered
        )
    }
}

/// Extended test configuration for end-to-end tests
#[derive(Debug, Clone)]
pub struct E2ETestConfig {
    pub base_config: TestConfig,
    pub enable_websocket: bool,
    pub enable_persistence: bool,
    pub enable_monitoring: bool,
    pub test_duration_secs: u64,
    pub expected_min_entities: usize,
    pub expected_min_resources: usize,
    pub max_allowed_errors: u32,
}

impl Default for E2ETestConfig {
    fn default() -> Self {
        Self {
            base_config: TestConfig::default(),
            enable_websocket: true,
            enable_persistence: true,
            enable_monitoring: true,
            test_duration_secs: 30,
            expected_min_entities: 1,
            expected_min_resources: 1,
            max_allowed_errors: 0,
        }
    }
}

impl E2ETestConfig {
    pub fn with_websocket(mut self, enabled: bool) -> Self {
        self.enable_websocket = enabled;
        self
    }

    pub fn with_persistence(mut self, enabled: bool) -> Self {
        self.enable_persistence = enabled;
        self
    }

    pub fn with_monitoring(mut self, enabled: bool) -> Self {
        self.enable_monitoring = enabled;
        self
    }

    pub fn with_duration(mut self, seconds: u64) -> Self {
        self.test_duration_secs = seconds;
        self
    }

    pub fn production() -> Self {
        Self {
            base_config: TestConfig::production(),
            enable_websocket: true,
            enable_persistence: true,
            enable_monitoring: true,
            test_duration_secs: 60,
            expected_min_entities: 10,
            expected_min_resources: 5,
            max_allowed_errors: 1,
        }
    }
}

/// Setup comprehensive end-to-end test environment
pub async fn setup_e2e_test_environment() -> E2ETestContext {
    let inner = crate::common::setup_test_environment().await;

    E2ETestContext {
        inner,
        websocket_server: None,
        monitoring_enabled: false,
        persistence_enabled: false,
        test_metrics: TestMetrics::new(),
    }
}

/// Initialize simulation with extended configuration
pub async fn initialize_e2e_simulation(
    context: &mut E2ETestContext,
    config: &E2ETestConfig,
) -> Result<Simulation, Box<dyn std::error::Error>> {
    let simulation = context
        .inner
        .initialize_simulation(&config.base_config)
        .await?;

    if config.enable_websocket {
        let ws_server = Arc::new(RwLock::new(MockWebSocketServer::new()));
        context.websocket_server = Some(ws_server);
    }

    context.monitoring_enabled = config.enable_monitoring;
    context.persistence_enabled = config.enable_persistence;

    Ok(simulation)
}

/// Run comprehensive test suite
pub async fn run_comprehensive_test_suite(
    context: &mut E2ETestContext,
    simulation: &mut Simulation,
    config: &E2ETestConfig,
) -> Result<TestMetrics, Box<dyn std::error::Error>> {
    let test_duration = std::time::Duration::from_secs(config.test_duration_secs);
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < test_duration {
        // Execute simulation tick
        let tick_start = std::time::Instant::now();
        let result = simulation.tick().await;
        let tick_duration = tick_start.elapsed();

        // Record metrics
        context.test_metrics.record_tick();
        context.test_metrics.record_performance("tick_duration_ms", tick_duration.as_millis() as f64);

        if let Err(e) = result {
            context.test_metrics.record_error();
            eprintln!("Tick failed: {}", e);
            if context.test_metrics.errors_encountered > config.max_allowed_errors {
                return Err(format!("Too many errors: {}", context.test_metrics.errors_encountered).into());
            }
        }

        // Collect state information
        let state = simulation.get_state().await?;
        context.test_metrics.record_entities(state.entities.len());
        context.test_metrics.record_resources(state.resources.len());

        // Process WebSocket messages if enabled
        if let Some(ws_server) = &context.websocket_server {
            let mut server = ws_server.write().await;
            // Simulate broadcasting state updates
            server.broadcast(IPCMessage::WorldUpdate {
                tick: state.tick,
                entities: state.entities.len(),
            });
        }

        // Check test invariants
        if context.test_metrics.tick_count % 10 == 0 {
            validate_test_invariants(context, config)?;
        }

        // Adaptive delay based on performance
        let delay_ms = if tick_duration.as_millis() > 100 {
            10 // Slow down if ticks are taking too long
        } else {
            50 // Normal pace
        };

        tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
    }

    // Final validation
    validate_test_completion(context, config)?;

    Ok(context.test_metrics.clone())
}

/// Validate test invariants during execution
fn validate_test_invariants(
    context: &E2ETestContext,
    config: &E2ETestConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    if context.test_metrics.total_entities < config.expected_min_entities {
        return Err(format!(
            "Entity count too low: {} < {}",
            context.test_metrics.total_entities, config.expected_min_entities
        )
        .into());
    }

    if context.test_metrics.total_resources < config.expected_min_resources {
        return Err(format!(
            "Resource count too low: {} < {}",
            context.test_metrics.total_resources, config.expected_min_resources
        )
        .into());
    }

    // Check performance metrics
    if let Some(&avg_tick_time) = context.test_metrics.performance_metrics.get("tick_duration_ms") {
        if avg_tick_time > 1000.0 {
            return Err(format!(
                "Average tick time too high: {} ms",
                avg_tick_time
            )
            .into());
        }
    }

    Ok(())
}

/// Validate test completion criteria
fn validate_test_completion(
    context: &E2ETestContext,
    config: &E2ETestConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    validate_test_invariants(context, config)?;

    if context.test_metrics.tick_count < 10 {
        return Err("Test didn't run long enough".into());
    }

    if context.test_metrics.errors_encountered > config.max_allowed_errors {
        return Err(format!(
            "Too many errors encountered: {}",
            context.test_metrics.errors_encountered
        )
        .into());
    }

    Ok(())
}

/// Generate comprehensive test report
pub fn generate_test_report(metrics: &TestMetrics) -> String {
    use std::time::Duration;

    let total_duration = metrics.start_time.elapsed();
    let avg_tick_time = total_duration / metrics.tick_count;

    format!(
        r#"
E2E Test Report
===============

Duration: {:?}
Total Ticks: {}
Average Tick Time: {:?}
Total Entities: {}
Total Resources: {}
Errors Encountered: {}

Performance Metrics:
{}
"#,
        total_duration,
        metrics.tick_count,
        avg_tick_time,
        metrics.total_entities,
        metrics.total_resources,
        metrics.errors_encountered,
        metrics
            .performance_metrics
            .iter()
            .map(|(k, v)| format!("  {}: {:.2}", k, v))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

/// Cleanup end-to-end test environment
pub async fn cleanup_e2e_test_environment(context: E2ETestContext) {
    context.inner.cleanup().await;

    // Additional E2E cleanup if needed
    if context.websocket_server.is_some() {
        // Clean up WebSocket server resources
    }

    println!("E2E Test cleanup completed");
    println!("{}", generate_test_report(&context.test_metrics));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_e2e_context_creation() {
        let context = setup_e2e_test_environment().await;
        assert_eq!(context.test_metrics.tick_count, 0);
        assert_eq!(context.test_metrics.errors_encountered, 0);
        cleanup_e2e_test_environment(context).await;
    }

    #[tokio::test]
    async fn test_metrics_collection() {
        let mut metrics = TestMetrics::new();
        metrics.record_tick();
        metrics.record_entities(10);
        metrics.record_resources(5);
        metrics.record_performance("test_metric", 42.0);

        assert_eq!(metrics.tick_count, 1);
        assert_eq!(metrics.total_entities, 10);
        assert_eq!(metrics.total_resources, 5);
        assert_eq!(metrics.performance_metrics.get("test_metric"), Some(&42.0));
    }

    #[tokio::test]
    async fn test_config_creation() {
        let config = E2ETestConfig::production();
        assert!(config.enable_websocket);
        assert!(config.enable_persistence);
        assert!(config.enable_monitoring);
        assert_eq!(config.test_duration_secs, 60);
        assert_eq!(config.expected_min_entities, 10);
    }

    #[tokio::test]
    async fn test_websocket_mock() {
        let mut server = MockWebSocketServer::new();
        assert_eq!(server.connected_clients, 0);

        let _client = server.handle_client().await;
        assert_eq!(server.connected_clients, 1);
    }
}