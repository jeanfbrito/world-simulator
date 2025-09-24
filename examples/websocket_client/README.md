# WebSocket Client Example

This example demonstrates real-time monitoring and control of the world-simulator through WebSocket connections. It showcases live data streaming, remote simulation control, and interactive visualization capabilities.

## What This Example Shows

### WebSocket Features

- **Real-time Data Streaming**: Live updates of simulation state, entity positions, and performance metrics
- **Remote Control**: Commands to spawn entities, adjust simulation speed, and pause/resume
- **Client Management**: Authentication, permissions, and rate limiting
- **Performance Monitoring**: Real-time metrics and alerting system
- **Message Processing**: Command validation, error handling, and message queuing

### Data Streams Demonstrated

1. **Simulation State**: Tick updates, resource counts, entity statistics
2. **Entity Updates**: Position changes, health/energy updates, state transitions
3. **Performance Metrics**: TPS, memory usage, connection statistics
4. **World Events**: Entity spawning/destruction, resource harvesting, building completion
5. **Debug Stream**: Detailed system information for development

### Client Types

- **Simulation Client**: Subscribes to simulation state and entity updates
- **Monitoring Client**: Focuses on performance metrics and system health
- **Control Client**: Sends commands and manages simulation parameters

## Configuration

The WebSocket client uses comprehensive configuration for real-time features:

- **Connection Management**: Authentication, sessions, rate limiting
- **Data Streams**: Configurable update intervals and filters
- **Message Processing**: Command handlers and validation
- **Performance Monitoring**: Metrics collection and alerting

## Running the Example

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)

### From the Project Root

```bash
# Run the WebSocket client example
cargo run --example websocket_client
```

### From the Example Directory

```bash
cd examples/websocket_client
cargo run --bin websocket_client
```

## Expected Output

The demonstration will run for 500 ticks with a visualization window:

```
🌐 Starting WebSocket Client Example
🔗 WebSocket client initialized. Running for 500 ticks...
📡 This will showcase real-time data streaming and remote control capabilities.
🎮 WebSocket client started. Monitoring real-time data streams...
📈 Tracking client connections and message throughput...

📡 WebSocket Monitor - Tick 100: 3 clients, 245 messages sent, 12.3ms avg latency
📡 WebSocket Monitor - Tick 200: 3 clients, 487 messages sent, 11.8ms avg latency
📡 WebSocket Monitor - Tick 300: 3 clients, 732 messages sent, 12.1ms avg latency
📡 WebSocket Monitor - Tick 400: 3 clients, 978 messages sent, 11.9ms avg latency

🎉 WebSocket client demonstration completed successfully!
📊 Final WebSocket Analysis:
   • Total clients served: 3
   • Messages processed: 1225
   • Average latency: 12.0ms
   • Peak connections: 3
   • Simulation ticks: 500
🌐 Key WebSocket Features Demonstrated:
   • Real-time data streaming
   • Client connection management
   • Remote simulation control
   • Performance monitoring
   • Message queue processing
```

## Key Components

### Main Application (`main.rs`)

- WebSocket client state management
- Real-time data streaming systems
- Command processing and validation
- Performance monitoring and metrics
- Client connection simulation

### Configuration (`config.lua`)

- **WebSocket Server**: Connection settings and protocols
- **Data Streams**: Update intervals and content configuration
- **Client Management**: Authentication and permissions
- **Message Processing**: Command handlers and validation rules
- **Performance Monitoring**: Metrics collection and alerting

### Core Systems

- **Simulation Monitor**: Tracks and streams simulation state
- **Data Stream Handler**: Manages multiple data stream types
- **Command Handler**: Processes remote simulation commands
- **Performance Monitor**: Collects and analyzes WebSocket metrics

## Customization

### Modify Data Streams

Adjust stream configuration in the Lua file:

```lua
-- Change update frequency
config.data_streams.simulation_state.update_interval = 50
config.data_streams.entity_updates.batch_size = 50

-- Add custom stream
config.data_streams.custom_stream = {
    enabled = true,
    update_interval = 200,
    include_fields = {"custom_data"}
}
```

### Add Custom Commands

Create new command handlers:

```lua
config.message_processing.commands.custom_command = {
    required_params = {"param1"},
    optional_params = {"param2"},
    validation = {
        param1_range = {0, 100}
    },
    permission_level = "operator"
}
```

### Configure Authentication

Set up client authentication:

```lua
config.websocket.enable_auth = true
config.client_management.authentication.enabled = true
config.client_management.authentication.method = "token"
```

### Adjust Performance Settings

Optimize for different use cases:

```lua
-- High-frequency updates
config.data_streams.entity_updates.update_interval = 10
config.websocket.batch_messages = false

-- Large-scale deployment
config.websocket.max_connections = 1000
config.performance_monitoring.metrics.enabled = true
```

## Learning Points

This example teaches you:

1. **Real-time Communication**: WebSocket implementation for live data streaming
2. **Message Processing**: Command validation, queuing, and error handling
3. **Client Management**: Authentication, permissions, and rate limiting
4. **Performance Monitoring**: Real-time metrics and system health tracking
5. **Event Streaming**: Efficient data distribution to multiple clients

## Advanced Concepts

### WebSocket Protocol

- Connection establishment and handshaking
- Message framing and binary protocols
- Subprotocol negotiation
- Heartbeat and connection management

### Data Stream Optimization

- Message batching and compression
- Selective field inclusion
- Change detection and filtering
- Rate limiting and throttling

### Security Considerations

- Authentication and authorization
- Input validation and sanitization
- Rate limiting and DDoS protection
- CORS and origin validation

## Next Steps

After understanding this WebSocket client example, you can explore:

- **Custom World Generation**: Advanced procedural generation techniques
- **AI Demonstration**: Complex AI behaviors and decision making
- **Performance Testing**: Benchmarking and optimization
- **Modding Example**: Creating custom game modes and modifications

## Troubleshooting

### Common Issues

**WebSocket connection fails**: Check server configuration and firewall settings.

**Message delivery slow**: Adjust batch settings and update intervals.

**High memory usage**: Reduce buffer sizes and enable compression.

**Client authentication fails**: Verify token configuration and expiration.

**Performance degradation**: Monitor metrics and adjust rate limiting.

### Getting Help

If you encounter issues:

1. Check the WebSocket console output for connection details
2. Verify configuration parameters and network settings
3. Enable debug logging for detailed message traces
4. Monitor performance metrics for bottlenecks
5. Review WebSocket documentation for implementation details

## Performance Considerations

- Message frequency impacts both client and server performance
- Large message sizes benefit from compression
- Connection count affects memory usage and CPU load
- Rate limiting prevents abuse and ensures fair usage
- Batch processing improves throughput but increases latency

## Security Best Practices

- Always validate incoming messages and commands
- Implement proper authentication and authorization
- Use rate limiting to prevent abuse
- Sanitize all user input
- Monitor for unusual connection patterns
- Keep sensitive data out of logs

For more detailed information about WebSocket integration in the world-simulator, see the main project documentation.

## Technical Details

### Message Flow

1. **Client Connection**: WebSocket handshake and authentication
2. **Subscription**: Clients subscribe to specific data streams
3. **Data Streaming**: Server pushes updates based on subscriptions
4. **Command Processing**: Clients send commands for remote control
5. **Error Handling**: Graceful handling of connection issues

### Performance Characteristics

- **Latency**: Typically 10-50ms for local connections
- **Throughput**: Hundreds of messages per second per connection
- **Scalability**: Designed for dozens of concurrent clients
- **Memory Usage**: ~50MB base + ~1MB per active connection

### Extension Points

- Custom data streams and message types
- Additional command handlers and validation
- Alternative authentication methods
- Custom metrics and alerting rules
- Integration with external monitoring systems