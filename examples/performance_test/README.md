# Performance Testing Example

This example demonstrates comprehensive performance testing and benchmarking capabilities for the world-simulator. It showcases load testing, stress testing, metrics collection, and performance analysis.

## What This Example Shows

### Performance Testing Features

- **Load Testing**: Gradual increase in entity count and system stress
- **Stress Testing**: Pushing the system beyond normal operating limits
- **Metrics Collection**: Comprehensive performance data collection
- **Bottleneck Detection**: Automatic identification of performance issues
- **Scalability Analysis**: Understanding how performance scales with load

### Test Phases

1. **Baseline**: Light load with minimal entities (10 entities)
2. **Light Load**: Moderate entity count with basic AI (50 entities)
3. **Medium Load**: Increased complexity with more entities (100 entities)
4. **Heavy Load**: Near-maximum performance testing (200 entities)
5. **Stress Test**: Beyond normal operating limits (500+ entities)

### Performance Metrics Collected

- **Frame Metrics**: FPS, frame time, delta time, consistency
- **Memory Metrics**: Usage, allocation rates, garbage collection
- **CPU Metrics**: Usage per system, thread count, context switches
- **Entity Metrics**: Spawn rates, lifecycle times, type distribution
- **System Metrics**: Time per system, call counts, resource usage
- **AI Metrics**: Decision times, pathfinding, success rates

### Advanced Analysis

- **Bottleneck Detection**: Automatic identification of performance issues
- **Scalability Assessment**: How performance changes with load
- **Memory Analysis**: Allocation patterns and memory efficiency
- **CPU Utilization**: Per-system CPU usage and optimization targets
- **Recommendations**: Automated performance improvement suggestions

## Configuration

The performance test uses comprehensive configuration for detailed analysis:

- **Test Phases**: Configurable load levels and durations
- **Metrics Collection**: Detailed performance data collection
- **Benchmarking**: Specific component performance testing
- **Stress Testing**: System limits and stability testing
- **Reporting**: Comprehensive analysis and recommendations

## Running the Example

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)

### From the Project Root

```bash
# Run the performance testing example
cargo run --example performance_test
```

### From the Example Directory

```bash
cd examples/performance_test
cargo run --bin performance_test
```

## Expected Output

The demonstration will run through all test phases with detailed performance reporting:

```
⚡ Starting Performance Testing Example
🔬 Performance test initialized. Running comprehensive benchmarks...
📊 This will showcase performance analysis and optimization insights.
🔬 Setting up Performance Testing Example...
✅ Performance test setup complete!
🎯 Test phases:
   1. Baseline - 10 entities, 0.1 stress level
   2. Light Load - 50 entities, 0.3 stress level
   3. Medium Load - 100 entities, 0.6 stress level
   4. Heavy Load - 200 entities, 1.0 stress level
   5. Stress Test - 500 entities, 1.5 stress level
📊 Metrics collected: FPS, memory usage, CPU usage, entity counts, system times

📊 Memory Monitor - Baseline: 52.3MB (12 entities)
📊 Memory Monitor - Light Load: 58.7MB (47 entities)
📊 Memory Monitor - Medium Load: 68.2MB (98 entities)
📊 Memory Monitor - Heavy Load: 89.5MB (195 entities)
📊 Memory Monitor - Stress Test: 145.8MB (498 entities)

✅ Completed phase: Baseline
📊 Phase Report: Baseline
   Target entities: 10
   Average FPS: 59.8
   Average frame time: 16.72ms
   Average memory: 52.1MB
   Average CPU: 15.3%

✅ Completed phase: Light Load
📊 Phase Report: Light Load
   Target entities: 50
   Average FPS: 58.2
   Average frame time: 17.18ms
   Average memory: 57.9MB
   Average CPU: 23.7%

✅ Completed phase: Medium Load
📊 Phase Report: Medium Load
   Target entities: 100
   Average FPS: 54.3
   Average frame time: 18.42ms
   Average memory: 67.5MB
   Average CPU: 34.1%

✅ Completed phase: Heavy Load
📊 Phase Report: Heavy Load
   Target entities: 200
   Average FPS: 45.7
   Average frame time: 21.88ms
   Average memory: 88.2MB
   Average CPU: 48.9%

✅ Completed phase: Stress Test
📊 Phase Report: Stress Test
   Target entities: 500
   Average FPS: 28.4
   Average frame time: 35.21ms
   Average memory: 143.7MB
   Average CPU: 72.3%
   Bottlenecks detected: 2

🎉 Performance testing completed successfully!
📊 Final Performance Analysis:
   • Overall average FPS: 49.3
   • Overall average frame time: 21.88ms
   • Peak memory usage: 145.8MB
   • Total test duration: 33.3s
   • Scalability results:
        Baseline: 12 entities, 59.8 FPS, 16.72ms frame time
        Light Load: 47 entities, 58.2 FPS, 17.18ms frame time
        Medium Load: 98 entities, 54.3 FPS, 18.42ms frame time
        Heavy Load: 195 entities, 45.7 FPS, 21.88ms frame time
        Stress Test: 498 entities, 28.4 FPS, 35.21ms frame time
   • Bottlenecks identified: 3
        Overall Frame Time: 35.21ms (Critical)
        Memory Usage: 143.7MB (High)
        AI Processing: 8.42ms (Medium)
⚡ Key Performance Insights Demonstrated:
   • Comprehensive metrics collection
   • Load testing and stress testing
   • Memory usage monitoring
   • Bottleneck detection and analysis
   • Scalability assessment
```

## Key Components

### Main Application (`main.rs`)

- Performance test state management
- Comprehensive metrics collection
- Load testing and stress testing scenarios
- Bottleneck detection and analysis
- Scalability assessment and reporting

### Configuration (`config.lua`)

- **Test Phases**: Configurable load levels and test scenarios
- **Metrics Collection**: Detailed performance monitoring configuration
- **Benchmarking**: Individual component performance tests
- **Stress Testing**: System limit and stability testing
- **Reporting**: Analysis and recommendation generation

### Core Systems

- **Metrics Collector**: Real-time performance data collection
- **Load Test Manager**: Entity spawning and stress level management
- **Memory Monitor**: Memory usage tracking and analysis
- **Bottleneck Detector**: Automatic performance issue identification
- **Scalability Analyzer**: Performance scaling assessment

## Customization

### Modify Test Phases

Adjust test phases in the configuration:

```lua
-- Change test duration
config.test.phases[1].duration_ticks = 500
config.test.phases[1].entity_target = 25

-- Add custom test phase
table.insert(config.test.phases, {
    name = "custom_test",
    duration_ticks = 300,
    entity_target = 150,
    stress_level = 0.8,
    measurements = {"fps", "memory", "custom_metric"}
})
```

### Configure Metrics Collection

Adjust metrics collection settings:

```lua
-- Change sampling rates
config.metrics.frame.sample_rate = 0.5
config.metrics.memory.sample_rate = 0.2

-- Add custom measurements
config.metrics.custom = {
    enabled = true,
    sample_rate = 1.0,
    measurements = {"custom_metric_1", "custom_metric_2"}
}
```

### Configure Stress Testing

Adjust stress testing parameters:

```lua
-- Modify stress test limits
config.test.stress_testing.memory_stress.max_memory_mb = 3000
config.test.stress_testing.cpu_stress.max_cpu_percent = 98
config.test.stress_testing.entity_stress.spawn_rate = 0.2
```

### Add Custom Benchmarks

Create specialized benchmark tests:

```lua
config.benchmarks.custom_benchmark = {
    enabled = true,
    iterations = 2000,
    warmup_iterations = 200,
    parameters = {
        param1 = {1, 5, 10, 20},
        param2 = {0.1, 0.5, 1.0}
    },
    measure = {"time", "memory", "throughput"}
}
```

## Learning Points

This example teaches you:

1. **Performance Testing**: Comprehensive testing methodology and practices
2. **Metrics Collection**: Real-time performance data collection and analysis
3. **Load Testing**: Gradual increase in system load and monitoring
4. **Stress Testing**: Pushing systems beyond normal operating limits
5. **Bottleneck Analysis**: Automatic identification of performance issues

## Advanced Concepts

### Performance Methodology

- **Baseline Testing**: Establishing performance baselines
- **Incremental Testing**: Gradual increase in load
- **Peak Performance Testing**: Maximum sustainable load testing
- **Stress Testing**: Beyond normal operating limits
- **Endurance Testing**: Long-term stability assessment

### Metrics Analysis

- **Statistical Analysis**: Mean, median, percentiles, standard deviation
- **Trend Analysis**: Performance changes over time
- **Correlation Analysis**: Relationships between different metrics
- **Bottleneck Identification**: Root cause analysis
- **Predictive Analysis**: Performance projection and modeling

### Optimization Techniques

- **Code Optimization**: Algorithmic improvements and code efficiency
- **Memory Optimization**: Allocation patterns and memory management
- **System Optimization**: ECS optimization and system scheduling
- **AI Optimization**: Algorithm improvements and LOD systems
- **Rendering Optimization**: Visual effects and rendering pipeline

## Next Steps

After understanding this performance testing example, you can explore:

- **Custom World Generation**: Advanced procedural generation techniques
- **AI Demonstration**: Complex AI behaviors and decision making
- **WebSocket Client**: Real-time monitoring and control
- **Modding Example**: Creating custom game modes and modifications

## Troubleshooting

### Common Issues

**Test runs too slow**: Reduce test duration or entity counts in configuration.

**Memory usage too high**: Enable memory optimization and reduce entity counts.

**Inconsistent results**: Ensure stable system conditions and consistent test parameters.

**Missing metrics**: Verify metric configuration and sampling rates.

**Analysis unclear**: Enable detailed logging and extended reporting.

### Getting Help

If you encounter issues:

1. Check the detailed performance output for specific metrics
2. Verify configuration parameters and test settings
3. Enable debug logging for detailed performance traces
4. Review the generated reports for analysis and recommendations
5. Monitor system resources during testing

## Performance Considerations

- Performance testing itself consumes system resources
- Background processes can affect test results
- Hardware variations impact performance metrics
- Test duration affects metric accuracy
- Multiple test runs improve result reliability

## Best Practices

- Always run tests on representative hardware
- Use consistent system conditions for testing
- Run multiple iterations for reliable results
- Monitor system resources during testing
- Document test configurations and conditions
- Compare results against established baselines

For more detailed information about performance testing in the world-simulator, see the main project documentation.

## Technical Details

### Testing Methodology

1. **Baseline Establishment**: Measure performance with minimal load
2. **Incremental Loading**: Gradually increase system load
3. **Peak Assessment**: Test maximum sustainable performance
4. **Stress Evaluation**: Push beyond normal operating limits
5. **Analysis Phase**: Comprehensive analysis and reporting

### Metric Categories

- **Performance Metrics**: FPS, frame time, throughput
- **Resource Metrics**: Memory usage, CPU usage, allocations
- **System Metrics**: Per-system performance and scheduling
- **Quality Metrics**: Accuracy, stability, consistency
- **Scalability Metrics**: Performance changes with load

### Analysis Techniques

- **Statistical Analysis**: Mean, variance, distributions
- **Trend Analysis**: Performance changes over time
- **Comparative Analysis**: Before/after optimization comparisons
- **Correlation Analysis**: Relationship between metrics
- **Root Cause Analysis**: Identifying performance bottlenecks