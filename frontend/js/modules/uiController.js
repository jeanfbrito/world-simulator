// UI Controller Module - Handles user interface interactions
export class UIController {
    constructor(gameController) {
        this.gameController = gameController;
        this.selectedTool = 'grass';
        this.isPlaying = false;
    }
    
    initialize() {
        this.setupToolButtons();
        this.setupSimulationControls();
        this.setupEventListeners();
        this.updateTileInfo(null);
    }
    
    setupToolButtons() {
        const toolButtons = document.querySelectorAll('.tool-btn');
        
        toolButtons.forEach(button => {
            button.addEventListener('click', (e) => {
                // Remove active class from all buttons
                toolButtons.forEach(btn => btn.classList.remove('active'));
                
                // Add active class to clicked button
                button.classList.add('active');
                
                // Set selected tool
                this.selectedTool = button.dataset.tool;
                this.gameController.selectedTool = this.selectedTool;
            });
        });
        
        // Set default tool as active
        document.querySelector('.tool-btn[data-tool="grass"]')?.classList.add('active');
    }
    
    setupSimulationControls() {
        const playPauseBtn = document.getElementById('play-pause');
        const stepBtn = document.getElementById('step');
        const resetBtn = document.getElementById('reset');
        const speedSlider = document.getElementById('speed-slider');
        
        playPauseBtn?.addEventListener('click', () => {
            // Use WebSocket if connected, otherwise local control
            if (window.wsClient && window.wsClient.isConnected) {
                window.wsClient.playPause();
            } else {
                if (this.isPlaying) {
                    this.gameController.pause();
                    playPauseBtn.textContent = '▶️ Play';
                    this.isPlaying = false;
                } else {
                    this.gameController.play();
                    playPauseBtn.textContent = '⏸️ Pause';
                    this.isPlaying = true;
                }
            }
        });
        
        stepBtn?.addEventListener('click', () => {
            this.gameController.step();
        });
        
        resetBtn?.addEventListener('click', () => {
            this.gameController.reset();
            playPauseBtn.textContent = '▶️ Play';
            this.isPlaying = false;
        });
        
        speedSlider?.addEventListener('input', (e) => {
            this.gameController.setSpeed(parseInt(e.target.value));
        });
    }
    
    setupEventListeners() {
        // Listen for tile selection
        window.eventBus.on('tile:selected', (data) => {
            this.updateTileInfo(data);
            
            // Apply selected tool if in edit mode
            if (this.selectedTool && !this.isPlaying) {
                this.gameController.worldMap.setTile(data.x, data.y, this.selectedTool);
            }
        });
        
        // Listen for tile hover
        window.eventBus.on('tile:hover', (data) => {
            // Only update on hover if no tile is selected
            // Disabled for now to prevent random updates
            // if (!this.gameController.worldMap.selectedTile) {
            //     this.updateTileInfo(data);
            // }
        });
        
        // Listen for game events
        window.eventBus.on('game:tick', (data) => {
            // Could update UI based on game ticks
        });
        
        // Listen for log events
        window.eventBus.on('log:add', (data) => {
            this.addLogEntry(data.message, data.type);
        });
        
        // Listen for map generation
        window.eventBus.on('map:generated', (data) => {
            this.addLogEntry(`Generated ${data.type} map`, 'success');
        });
        
        window.eventBus.on('map:cleared', () => {
            this.addLogEntry('Map cleared', 'success');
        });
    }
    
    updateTileInfo(data) {
        const posEl = document.getElementById('tile-pos');
        const typeEl = document.getElementById('tile-type');
        const walkableEl = document.getElementById('tile-walkable');
        const entitiesEl = document.getElementById('tile-entities');
        
        if (data) {
            posEl.textContent = `(${data.x}, ${data.y})`;
            typeEl.textContent = data.type;
            
            const walkable = this.gameController.worldMap.isWalkable(data.x, data.y);
            walkableEl.textContent = walkable ? 'Yes' : 'No';
            
            // Count entities at position
            let entityCount = 0;
            for (const [id, entity] of this.gameController.worldMap.entities) {
                if (entity.x === data.x && entity.y === data.y) {
                    entityCount++;
                }
            }
            entitiesEl.textContent = entityCount;
        } else {
            posEl.textContent = '-';
            typeEl.textContent = '-';
            walkableEl.textContent = '-';
            entitiesEl.textContent = '0';
        }
    }
    
    addLogEntry(message, type = 'info') {
        const logContainer = document.getElementById('event-log');
        if (!logContainer) return;
        
        const entry = document.createElement('div');
        entry.className = `log-entry ${type}`;
        
        const timestamp = new Date().toLocaleTimeString();
        entry.textContent = `[${timestamp}] ${message}`;
        
        logContainer.insertBefore(entry, logContainer.firstChild);
        
        // Limit log entries
        while (logContainer.children.length > 50) {
            logContainer.removeChild(logContainer.lastChild);
        }
    }
    
    updateConnectionStatus(connected) {
        const statusEl = document.getElementById('connection-status');
        if (statusEl) {
            if (connected) {
                statusEl.textContent = '🟢 Connected';
                statusEl.className = 'connected';
            } else {
                statusEl.textContent = '⚫ Disconnected';
                statusEl.className = 'disconnected';
            }
        }
    }
}