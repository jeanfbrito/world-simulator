# Sim Viewer - IPC to WebSocket Bridge

## Overview

The `world_sim_viewer` is a Rust binary that bridges IPC (Inter-Process Communication) output from the World Simulator to WebSocket clients, enabling web-based viewers to receive real-time simulation data.

## Architecture

```
┌─────────────────┐    IPC (JSON)    ┌─────────────────┐    WebSocket    ┌─────────────────┐
│  World Simulator │ ───────────────> │  Sim Viewer     │ ─────────────> │  Web Client     │
│  (headless)      │   stdout/stdin   │  (bridge)       │   real-time    │  (viewer.html)  │
└─────────────────┘                 └─────────────────┘                └─────────────────┘
```

## Features

- **IPC Input**: Reads JSON messages from stdin or file
- **WebSocket Server**: Broadcasts to connected web clients
- **Real-time Communication**: Async message processing
- **Client Management**: Automatic connection/disconnection handling
- **State Broadcasting**: Latest state sent to new connections

## Installation

Build from source:

```bash
cargo build -p world_sim_viewer
```

## Usage

### Basic Usage

Pipe simulator output to Sim Viewer:

```bash
# Start simulator with IPC output
RUST_LOG=info cargo run -p world_sim_simple | cargo run -p world_sim_viewer
```

### With Custom Port

```bash
cargo run -p world_sim_viewer -- --port 8080
```

### With File Input

```bash
# Save simulator output to file
RUST_LOG=info cargo run -p world_sim_simple > sim_output.log 2>&1

# Process file with Sim Viewer
cargo run -p world_sim_viewer -- --ipc-file sim_output.log --port 8080
```

### Command Line Options

```bash
$ cargo run -p world_sim_viewer -- --help
Sim Viewer - IPC to WebSocket bridge

Usage: world_sim_viewer [OPTIONS]

Options:
  -i, --ipc-file <IPC_FILE>  IPC input file (stdin if not specified)
  -p, --port <PORT>          WebSocket server port [default: 8080]
  -v, --verbose              Enable verbose logging
  -h, --help                 Print help
```

## IPC Message Format

The Sim Viewer expects JSON messages in the format:

```json
{
  "version": 1,
  "timestamp": 1234567890,
  "seq_num": 1,
  "payload": {
    "Heartbeat": {
      "sender": "simulator",
      "sent_at": 1234567890,
      "metrics": null
    }
  }
}
```

Supported payload types:
- `Heartbeat`: System heartbeat
- `GameState`: Complete game state snapshot
- `PackDefinitions`: Pack and entity definitions

## WebSocket API

### Connection

```javascript
const ws = new WebSocket('ws://localhost:8080');
```

### Message Format

Messages are sent as JSON strings matching the IPC payload format:

```json
{
  "type": "GameState",
  "tick": 123,
  "world_size": [100, 100],
  "entities": [...],
  "global_state": {...}
}
```

## Development

### Running Tests

```bash
cargo test -p world_sim_viewer
```

### Building

```bash
cargo build -p world_sim_viewer
```

### Adding Features

1. Update IPC message types in `world_sim_interface`
2. Add message handling in `sim_viewer/src/main.rs`
3. Update web client to handle new message types
4. Add tests for new functionality

## Integration with Monitor Script

The `monitor_world.sh` script can be updated to use the Sim Viewer:

```bash
# Start simulator and pipe to Sim Viewer
RUST_LOG=info cargo run -p world_sim_simple 2>&1 | \
  cargo run -p world_sim_viewer -- --port 8080 &

# Open web viewer
open viewer.html
```

## Performance Considerations

- Messages are broadcast to all connected clients
- Each connection runs in its own async task
- Client disconnections are handled automatically
- No message buffering - all messages are processed immediately

## Troubleshooting

### Common Issues

1. **Port already in use**: Use `--port` to specify a different port
2. **No IPC messages**: Ensure simulator is running with IPC output enabled
3. **WebSocket connection fails**: Check firewall and port accessibility

### Debug Mode

Enable verbose logging for detailed output:

```bash
cargo run -p world_sim_viewer -- --verbose
```

## Future Enhancements

- [ ] Message batching for high-frequency updates
- [ ] Client authentication
- [ ] Message compression
- [ ] Differential updates (only send changed data)
- [ ] Connection rate limiting
- [ ] Metrics and monitoring