// WebSocket Client Module - Handles real-time communication with the Rust backend
export class WebSocketClient {
    constructor(url = 'ws://localhost:8080') {
        this.url = url;
        this.ws = null;
        this.isConnected = false;
        this.clientId = null;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        this.reconnectDelay = 2000;
        this.messageHandlers = new Map();
        this.pingInterval = null;
    }
    
    connect() {
        return new Promise((resolve, reject) => {
            try {
                console.log(`Connecting to WebSocket server at ${this.url}...`);
                this.ws = new WebSocket(this.url);
                
                this.ws.onopen = () => {
                    console.log('WebSocket connected!');
                    this.isConnected = true;
                    // Don't reset reconnectAttempts here - wait for successful message exchange
                    
                    // Send initial connect message
                    this.send({
                        type: 'Connect',
                        client_id: this.generateClientId()
                    });
                    
                    // Start ping interval to keep connection alive
                    this.startPingInterval();
                    
                    // Update UI
                    window.eventBus.emit('websocket:connected');
                    resolve();
                };
                
                this.ws.onmessage = (event) => {
                    try {
                        const message = JSON.parse(event.data);
                        this.handleMessage(message);
                    } catch (error) {
                        console.error('Error parsing WebSocket message:', error);
                    }
                };
                
                this.ws.onerror = (error) => {
                    console.error('WebSocket error:', error);
                    window.eventBus.emit('websocket:error', error);
                };
                
                this.ws.onclose = () => {
                    console.log('WebSocket disconnected');
                    this.isConnected = false;
                    this.stopPingInterval();
                    window.eventBus.emit('websocket:disconnected');
                    
                    // Attempt to reconnect
                    if (this.reconnectAttempts < this.maxReconnectAttempts) {
                        setTimeout(() => {
                            this.reconnectAttempts++;
                            console.log(`Reconnecting... (attempt ${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
                            this.connect();
                        }, this.reconnectDelay);
                    } else {
                        console.log('Max reconnection attempts reached');
                    }
                };
                
            } catch (error) {
                console.error('Failed to create WebSocket:', error);
                reject(error);
            }
        });
    }
    
    disconnect() {
        if (this.ws) {
            this.send({
                type: 'Disconnect',
                client_id: this.clientId
            });
            this.ws.close();
            this.ws = null;
        }
        this.isConnected = false;
        this.stopPingInterval();
    }
    
    send(message) {
        if (this.isConnected && this.ws && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify(message));
        } else {
            console.warn('WebSocket not connected, queuing message:', message);
            // Could implement a message queue here
        }
    }
    
    handleMessage(message) {
        switch (message.type) {
            case 'Welcome':
                this.clientId = message.client_id;
                console.log('Received welcome, client ID:', this.clientId);
                // Now we can reset reconnect attempts after successful handshake
                this.reconnectAttempts = 0;
                window.eventBus.emit('websocket:welcome', message);
                // Request initial game state
                this.send({ type: 'RequestState' });
                break;
                
            case 'GameState':
                this.handleGameState(message.state);
                break;
                
            case 'TileUpdate':
                window.eventBus.emit('tile:update', {
                    x: message.x,
                    y: message.y,
                    type: message.tile_type
                });
                break;
                
            case 'EntityUpdate':
                window.eventBus.emit('entities:update', message.entities);
                break;
                
            case 'TickUpdate':
                window.eventBus.emit('tick:update', message.tick);
                break;
                
            case 'Error':
                console.error('Server error:', message.message);
                window.eventBus.emit('server:error', message.message);
                break;
                
            default:
                console.log('Unknown message type:', message.type, message);
        }
        
        // Call custom handlers if registered
        if (this.messageHandlers.has(message.type)) {
            this.messageHandlers.get(message.type)(message);
        }
    }
    
    handleGameState(state) {
        // Update the entire game state
        window.eventBus.emit('gamestate:update', state);
        
        // Update map tiles
        if (state.tiles) {
            for (let y = 0; y < state.tiles.length; y++) {
                for (let x = 0; x < state.tiles[y].length; x++) {
                    window.eventBus.emit('tile:update', {
                        x,
                        y,
                        type: state.tiles[y][x]
                    });
                }
            }
        }
        
        // Update entities
        if (state.entities) {
            window.eventBus.emit('entities:update', state.entities);
        }
        
        // Update simulation state
        window.eventBus.emit('simulation:update', {
            tick: state.tick,
            running: state.running,
            speed: state.speed
        });
    }
    
    // Command methods
    playPause() {
        this.send({ type: 'PlayPause' });
    }
    
    setSpeed(speed) {
        this.send({ 
            type: 'SetSpeed',
            speed: parseFloat(speed)
        });
    }
    
    setTile(x, y, tileType) {
        this.send({
            type: 'SetTile',
            x: parseInt(x),
            y: parseInt(y),
            tile_type: tileType
        });
    }
    
    spawnWorker(x, y) {
        this.send({
            type: 'SpawnWorker',
            x: parseInt(x),
            y: parseInt(y)
        });
    }
    
    generateMap(mapType) {
        this.send({
            type: 'GenerateMap',
            map_type: mapType
        });
    }
    
    sendCommand(action, data) {
        this.send({
            type: 'Command',
            action,
            data
        });
    }
    
    // Handler registration
    on(messageType, handler) {
        this.messageHandlers.set(messageType, handler);
    }
    
    off(messageType) {
        this.messageHandlers.delete(messageType);
    }
    
    // Utility methods
    generateClientId() {
        return 'client_' + Math.random().toString(36).substr(2, 9);
    }
    
    startPingInterval() {
        this.pingInterval = setInterval(() => {
            if (this.isConnected && this.ws && this.ws.readyState === WebSocket.OPEN) {
                // Send ping to keep connection alive
                this.send({ type: 'Ping' });
            }
        }, 5000); // Every 5 seconds to prevent disconnection
    }
    
    stopPingInterval() {
        if (this.pingInterval) {
            clearInterval(this.pingInterval);
            this.pingInterval = null;
        }
    }
}