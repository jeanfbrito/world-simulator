// Main application entry point
import { WorldMap } from './modules/worldMap.js';
import { GameController } from './modules/gameController.js';
import { UIController } from './modules/uiController.js';
import { EventBus } from './modules/eventBus.js';
import { WebSocketClient } from './modules/websocketClient.js';

// Initialize global event bus
window.eventBus = new EventBus();

// Initialize main components
const worldMap = new WorldMap('map', 64, 64);
const gameController = new GameController(worldMap);
const uiController = new UIController(gameController);
const wsClient = new WebSocketClient();

// Expose game controller and WebSocket client globally
window.game = gameController;
window.wsClient = wsClient;

// Initialize the application
document.addEventListener('DOMContentLoaded', () => {
    console.log('World Simulator Frontend Initialized');
    
    // Setup initial map
    worldMap.initialize();
    gameController.generateIslandMap();
    
    // Setup UI
    uiController.initialize();
    
    // Connect to WebSocket server
    wsClient.connect().then(() => {
        console.log('Connected to backend!');
    }).catch(error => {
        console.error('Failed to connect to backend:', error);
    });
    
    // Setup WebSocket event handlers
    setupWebSocketHandlers();
    
    // Start render loop
    let lastTime = 0;
    let fps = 0;
    let fpsCounter = 0;
    let fpsTime = 0;
    
    function animate(currentTime) {
        const deltaTime = currentTime - lastTime;
        lastTime = currentTime;
        
        // Calculate FPS
        fpsCounter++;
        fpsTime += deltaTime;
        if (fpsTime >= 1000) {
            fps = Math.round(fpsCounter * 1000 / fpsTime);
            document.getElementById('fps-counter').textContent = `FPS: ${fps}`;
            fpsCounter = 0;
            fpsTime = 0;
        }
        
        // Update game state
        gameController.update(deltaTime);
        
        // Request next frame
        requestAnimationFrame(animate);
    }
    
    requestAnimationFrame(animate);
    
    // Log ready state
    eventBus.emit('app:ready');
    console.log('Application ready');
});

// Setup WebSocket event handlers
function setupWebSocketHandlers() {
    // Connection status
    window.eventBus.on('websocket:connected', () => {
        uiController.updateConnectionStatus(true);
    });
    
    window.eventBus.on('websocket:disconnected', () => {
        uiController.updateConnectionStatus(false);
    });
    
    // Game state updates
    window.eventBus.on('gamestate:update', (state) => {
        // Update tick counter
        document.getElementById('tick-counter').textContent = `Tick: ${state.tick}`;
        
        // Sync map if needed
        if (state.tiles) {
            for (let y = 0; y < state.tiles.length; y++) {
                for (let x = 0; x < state.tiles[y].length; x++) {
                    worldMap.setTile(x, y, state.tiles[y][x]);
                }
            }
        }
    });
    
    // Entity updates
    window.eventBus.on('entities:update', (entities) => {
        // Clear existing entities
        for (const [id] of worldMap.entities) {
            worldMap.removeEntity(id);
        }
        
        // Add new entities
        for (const entity of entities) {
            const symbol = entity.entity_type === 'worker' ? '👷' : '📦';
            worldMap.addEntity(entity.id, entity.x, entity.y, entity.entity_type, symbol);
        }
        
        // Update counts
        const workers = entities.filter(e => e.entity_type === 'worker').length;
        const buildings = entities.filter(e => e.entity_type === 'building').length;
        const resources = entities.filter(e => e.entity_type === 'resource').length;
        
        document.getElementById('worker-count').textContent = workers;
        document.getElementById('building-count').textContent = buildings;
        document.getElementById('resource-count').textContent = resources;
    });
    
    // Tile updates
    window.eventBus.on('tile:update', (data) => {
        worldMap.setTile(data.x, data.y, data.type);
    });
}