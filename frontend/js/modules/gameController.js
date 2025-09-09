// Game Controller Module - Handles game logic and simulation
export class GameController {
    constructor(worldMap) {
        this.worldMap = worldMap;
        this.isRunning = false;
        this.tickCount = 0;
        this.tickRate = 1000; // ms per tick
        this.lastTickTime = 0;
        this.speed = 1;
        this.selectedTool = 'grass';
        
        this.workers = [];
        this.buildings = [];
        this.resources = [];
    }
    
    update(deltaTime) {
        if (!this.isRunning) return;
        
        this.lastTickTime += deltaTime * this.speed;
        
        if (this.lastTickTime >= this.tickRate) {
            this.tick();
            this.lastTickTime = 0;
        }
    }
    
    tick() {
        this.tickCount++;
        document.getElementById('tick-counter').textContent = `Tick: ${this.tickCount}`;
        
        // Update simulation
        this.updateWorkers();
        this.updateBuildings();
        this.updateResources();
        
        // Emit tick event
        window.eventBus.emit('game:tick', { tick: this.tickCount });
    }
    
    updateWorkers() {
        for (const worker of this.workers) {
            // Simple random movement for now
            if (Math.random() < 0.3) {
                const neighbors = this.worldMap.getNeighbors(worker.x, worker.y);
                const walkable = neighbors.filter(n => this.worldMap.isWalkable(n.x, n.y));
                
                if (walkable.length > 0) {
                    const newPos = walkable[Math.floor(Math.random() * walkable.length)];
                    this.worldMap.moveEntity(worker.id, newPos.x, newPos.y);
                    worker.x = newPos.x;
                    worker.y = newPos.y;
                }
            }
        }
    }
    
    updateBuildings() {
        // Buildings don't move, but could process resources
        for (const building of this.buildings) {
            if (building.type === 'workshop' && Math.random() < 0.1) {
                window.eventBus.emit('log:add', {
                    type: 'success',
                    message: `Workshop at (${building.x}, ${building.y}) produced an item`
                });
            }
        }
    }
    
    updateResources() {
        // Resources could regenerate or decay
        for (const resource of this.resources) {
            if (resource.type === 'berry' && Math.random() < 0.01) {
                resource.amount = Math.min(resource.amount + 1, 10);
            }
        }
    }
    
    // Control methods
    play() {
        this.isRunning = true;
        window.eventBus.emit('game:play');
    }
    
    pause() {
        this.isRunning = false;
        window.eventBus.emit('game:pause');
    }
    
    step() {
        this.tick();
    }
    
    reset() {
        this.isRunning = false;
        this.tickCount = 0;
        this.lastTickTime = 0;
        this.workers = [];
        this.buildings = [];
        this.resources = [];
        
        // Clear entities from map
        for (const [id] of this.worldMap.entities) {
            this.worldMap.removeEntity(id);
        }
        
        document.getElementById('tick-counter').textContent = 'Tick: 0';
        window.eventBus.emit('game:reset');
    }
    
    setSpeed(speed) {
        const speeds = [0.5, 1, 2, 4, 8];
        this.speed = speeds[speed] || 1;
        document.getElementById('speed-value').textContent = `${this.speed}x`;
    }
    
    // Map generation methods
    generateRandomMap() {
        const terrainTypes = ['grass', 'grass', 'grass', 'stone', 'sand', 'water'];
        
        for (let y = 0; y < this.worldMap.height; y++) {
            for (let x = 0; x < this.worldMap.width; x++) {
                const type = terrainTypes[Math.floor(Math.random() * terrainTypes.length)];
                this.worldMap.setTile(x, y, type);
            }
        }
        
        // Add resources
        this.addRandomResources(100);
        
        window.eventBus.emit('map:generated', { type: 'random' });
    }
    
    generateIslandMap() {
        const centerX = this.worldMap.width / 2;
        const centerY = this.worldMap.height / 2;
        
        for (let y = 0; y < this.worldMap.height; y++) {
            for (let x = 0; x < this.worldMap.width; x++) {
                const distFromCenter = Math.sqrt((x - centerX) ** 2 + (y - centerY) ** 2);
                const maxDist = Math.min(centerX, centerY);
                
                if (distFromCenter > maxDist * 0.9) {
                    this.worldMap.setTile(x, y, 'deep-water');
                } else if (distFromCenter > maxDist * 0.75) {
                    this.worldMap.setTile(x, y, 'water');
                } else if (distFromCenter > maxDist * 0.6) {
                    this.worldMap.setTile(x, y, 'sand');
                } else {
                    this.worldMap.setTile(x, y, 'grass');
                }
            }
        }
        
        // Add resources in the center
        this.addRandomResources(50, centerX - 15, centerY - 15, 30, 30);
        
        window.eventBus.emit('map:generated', { type: 'island' });
    }
    
    generateForestMap() {
        // Fill with grass
        this.worldMap.clear('grass');
        
        // Add forest patches
        for (let patch = 0; patch < 12; patch++) {
            const centerX = Math.floor(Math.random() * this.worldMap.width);
            const centerY = Math.floor(Math.random() * this.worldMap.height);
            const radius = Math.floor(Math.random() * 8) + 4;
            
            for (let y = Math.max(0, centerY - radius); y < Math.min(this.worldMap.height, centerY + radius); y++) {
                for (let x = Math.max(0, centerX - radius); x < Math.min(this.worldMap.width, centerX + radius); x++) {
                    const dist = Math.sqrt((x - centerX) ** 2 + (y - centerY) ** 2);
                    if (dist < radius && Math.random() > 0.3) {
                        this.worldMap.setTile(x, y, 'tree');
                    }
                }
            }
        }
        
        // Add clearings with resources
        for (let i = 0; i < 30; i++) {
            const x = Math.floor(Math.random() * this.worldMap.width);
            const y = Math.floor(Math.random() * this.worldMap.height);
            if (this.worldMap.getTileType(x, y) === 'grass') {
                this.worldMap.setTile(x, y, Math.random() > 0.5 ? 'berry' : 'stone');
            }
        }
        
        window.eventBus.emit('map:generated', { type: 'forest' });
    }
    
    generateVillageMap() {
        // Start with grass
        this.worldMap.clear('grass');
        
        // Create village center
        const centerX = Math.floor(this.worldMap.width / 2);
        const centerY = Math.floor(this.worldMap.height / 2);
        
        // Add roads (stone paths)
        for (let x = centerX - 15; x <= centerX + 15; x++) {
            this.worldMap.setTile(x, centerY, 'stone');
            this.worldMap.setTile(x, centerY - 1, 'stone');
        }
        for (let y = centerY - 15; y <= centerY + 15; y++) {
            this.worldMap.setTile(centerX, y, 'stone');
            this.worldMap.setTile(centerX - 1, y, 'stone');
        }
        
        // Add buildings around the center
        const buildings = [
            { x: centerX - 5, y: centerY - 5, type: 'storage' },
            { x: centerX + 5, y: centerY - 5, type: 'workshop' },
            { x: centerX - 5, y: centerY + 5, type: 'storage' },
            { x: centerX + 5, y: centerY + 5, type: 'workshop' }
        ];
        
        for (const building of buildings) {
            // Create building footprint
            for (let dy = 0; dy < 3; dy++) {
                for (let dx = 0; dx < 3; dx++) {
                    if (dx === 1 && dy === 2) {
                        this.worldMap.setTile(building.x + dx, building.y + dy, 'door');
                    } else {
                        this.worldMap.setTile(building.x + dx, building.y + dy, building.type);
                    }
                }
            }
            
            // Add to buildings list
            this.addBuilding(building.x + 1, building.y + 1, building.type);
        }
        
        // Add walls around village
        const wallRadius = 20;
        for (let x = centerX - wallRadius; x <= centerX + wallRadius; x++) {
            if (Math.abs(x - centerX) === wallRadius) {
                for (let y = centerY - wallRadius; y <= centerY + wallRadius; y++) {
                    if (Math.abs(y - centerY) > 3) { // Leave gaps for gates
                        this.worldMap.setTile(x, y, 'wall');
                    }
                }
            }
        }
        for (let y = centerY - wallRadius; y <= centerY + wallRadius; y++) {
            if (Math.abs(y - centerY) === wallRadius) {
                for (let x = centerX - wallRadius; x <= centerX + wallRadius; x++) {
                    if (Math.abs(x - centerX) > 3) { // Leave gaps for gates
                        this.worldMap.setTile(x, y, 'wall');
                    }
                }
            }
        }
        
        // Add some workers
        for (let i = 0; i < 5; i++) {
            const x = centerX + Math.floor(Math.random() * 10) - 5;
            const y = centerY + Math.floor(Math.random() * 10) - 5;
            if (this.worldMap.isWalkable(x, y)) {
                this.addWorker(x, y);
            }
        }
        
        // Add resources outside walls
        this.addRandomResources(30, 0, 0, this.worldMap.width, this.worldMap.height);
        
        window.eventBus.emit('map:generated', { type: 'village' });
    }
    
    clearMap() {
        this.worldMap.clear('grass');
        this.reset();
        window.eventBus.emit('map:cleared');
    }
    
    // Entity management
    addWorker(x, y) {
        const id = `worker_${Date.now()}_${Math.random()}`;
        const worker = { id, x, y, type: 'worker' };
        this.workers.push(worker);
        this.worldMap.addEntity(id, x, y, 'worker', '👷');
        this.updateEntityCounts();
    }
    
    addBuilding(x, y, type) {
        const id = `building_${Date.now()}_${Math.random()}`;
        const building = { id, x, y, type };
        this.buildings.push(building);
        
        const symbols = {
            storage: '📦',
            workshop: '🔨',
            house: '🏠'
        };
        
        this.worldMap.addEntity(id, x, y, 'building', symbols[type] || '🏢');
        this.updateEntityCounts();
    }
    
    addResource(x, y, type) {
        const id = `resource_${Date.now()}_${Math.random()}`;
        const resource = { id, x, y, type, amount: 10 };
        this.resources.push(resource);
        
        const symbols = {
            tree: '🌳',
            ore: '⛏️',
            berry: '🫐'
        };
        
        this.worldMap.addEntity(id, x, y, 'resource', symbols[type] || '📦');
        this.updateEntityCounts();
    }
    
    addRandomResources(count, startX = 0, startY = 0, width = null, height = null) {
        width = width || this.worldMap.width;
        height = height || this.worldMap.height;
        
        for (let i = 0; i < count; i++) {
            const x = startX + Math.floor(Math.random() * width);
            const y = startY + Math.floor(Math.random() * height);
            const type = this.worldMap.getTileType(x, y);
            
            if (type === 'grass') {
                const resourceType = Math.random() > 0.7 ? 'tree' : Math.random() > 0.5 ? 'berry' : 'ore';
                this.worldMap.setTile(x, y, resourceType);
            }
        }
    }
    
    updateEntityCounts() {
        document.getElementById('worker-count').textContent = this.workers.length;
        document.getElementById('building-count').textContent = this.buildings.length;
        document.getElementById('resource-count').textContent = this.resources.length;
    }
}