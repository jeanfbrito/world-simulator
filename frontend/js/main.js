// Main application entry point
import { WorldMap } from './modules/worldMap.js';
import { GameController } from './modules/gameController.js';
import { UIController } from './modules/uiController.js';
import { EventBus } from './modules/eventBus.js';

// Initialize global event bus
window.eventBus = new EventBus();

// Initialize main components
const worldMap = new WorldMap('map', 64, 64);
const gameController = new GameController(worldMap);
const uiController = new UIController(gameController);

// Expose game controller globally for HTML onclick handlers
window.game = gameController;

// Initialize the application
document.addEventListener('DOMContentLoaded', () => {
    console.log('World Simulator Frontend Initialized');
    
    // Setup initial map
    worldMap.initialize();
    gameController.generateIslandMap();
    
    // Setup UI
    uiController.initialize();
    
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