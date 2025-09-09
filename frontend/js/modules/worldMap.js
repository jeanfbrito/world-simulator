// World Map Module - Handles map rendering and tile management
export class WorldMap {
    constructor(containerId, width, height) {
        this.container = document.getElementById(containerId);
        this.width = width;
        this.height = height;
        this.tiles = [];
        this.entities = new Map();
        this.selectedTile = null;
        this.zoom = 1;
        this.tileSize = 12;
    }
    
    initialize() {
        this.container.innerHTML = '';
        this.container.style.setProperty('--tile-size', `${this.tileSize}px`);
        
        // Create tile grid
        for (let y = 0; y < this.height; y++) {
            for (let x = 0; x < this.width; x++) {
                const tile = this.createTile(x, y);
                this.tiles.push(tile);
                this.container.appendChild(tile);
            }
        }
        
        this.setupEventHandlers();
    }
    
    createTile(x, y) {
        const tile = document.createElement('div');
        tile.className = 'tile grass';
        tile.dataset.x = x;
        tile.dataset.y = y;
        tile.dataset.type = 'grass';
        
        tile.addEventListener('click', (e) => this.handleTileClick(e));
        tile.addEventListener('mouseenter', (e) => this.handleTileHover(e));
        
        return tile;
    }
    
    setupEventHandlers() {
        // Zoom controls
        document.getElementById('zoom-in')?.addEventListener('click', () => this.zoomIn());
        document.getElementById('zoom-out')?.addEventListener('click', () => this.zoomOut());
        document.getElementById('zoom-reset')?.addEventListener('click', () => this.resetZoom());
    }
    
    handleTileClick(event) {
        const tile = event.target;
        const x = parseInt(tile.dataset.x);
        const y = parseInt(tile.dataset.y);
        
        // Clear previous selection
        if (this.selectedTile) {
            this.selectedTile.classList.remove('selected');
        }
        
        // Set new selection
        this.selectedTile = tile;
        tile.classList.add('selected');
        
        // Emit event
        window.eventBus.emit('tile:selected', { x, y, type: tile.dataset.type });
    }
    
    handleTileHover(event) {
        const tile = event.target;
        const x = parseInt(tile.dataset.x);
        const y = parseInt(tile.dataset.y);
        
        window.eventBus.emit('tile:hover', { x, y, type: tile.dataset.type });
    }
    
    setTile(x, y, type) {
        const index = y * this.width + x;
        const tile = this.tiles[index];
        if (tile) {
            // Remove all tile type classes
            tile.className = 'tile';
            // Add new type class
            tile.classList.add(type);
            tile.dataset.type = type;
        }
    }
    
    getTile(x, y) {
        const index = y * this.width + x;
        return this.tiles[index];
    }
    
    getTileType(x, y) {
        const tile = this.getTile(x, y);
        return tile ? tile.dataset.type : null;
    }
    
    clear(type = 'grass') {
        for (let y = 0; y < this.height; y++) {
            for (let x = 0; x < this.width; x++) {
                this.setTile(x, y, type);
            }
        }
    }
    
    // Entity management
    addEntity(id, x, y, type, symbol) {
        const tile = this.getTile(x, y);
        if (!tile) return;
        
        // Remove existing entity at this position if any
        this.removeEntityAt(x, y);
        
        const entity = document.createElement('div');
        entity.className = `entity ${type}`;
        entity.textContent = symbol;
        entity.dataset.id = id;
        
        tile.appendChild(entity);
        this.entities.set(id, { x, y, element: entity });
    }
    
    moveEntity(id, newX, newY) {
        const entity = this.entities.get(id);
        if (!entity) return;
        
        const newTile = this.getTile(newX, newY);
        if (!newTile) return;
        
        // Move entity element to new tile
        newTile.appendChild(entity.element);
        
        // Update position
        entity.x = newX;
        entity.y = newY;
    }
    
    removeEntity(id) {
        const entity = this.entities.get(id);
        if (entity) {
            entity.element.remove();
            this.entities.delete(id);
        }
    }
    
    removeEntityAt(x, y) {
        for (const [id, entity] of this.entities) {
            if (entity.x === x && entity.y === y) {
                this.removeEntity(id);
                break;
            }
        }
    }
    
    // Zoom functionality
    zoomIn() {
        if (this.zoom < 3) {
            this.zoom *= 1.25;
            this.updateZoom();
        }
    }
    
    zoomOut() {
        if (this.zoom > 0.5) {
            this.zoom *= 0.8;
            this.updateZoom();
        }
    }
    
    resetZoom() {
        this.zoom = 1;
        this.updateZoom();
    }
    
    updateZoom() {
        const newSize = Math.round(this.tileSize * this.zoom);
        this.container.style.setProperty('--tile-size', `${newSize}px`);
        document.getElementById('zoom-value').textContent = `${Math.round(this.zoom * 100)}%`;
    }
    
    // Utility methods
    isWalkable(x, y) {
        const type = this.getTileType(x, y);
        const nonWalkable = ['water', 'deep-water', 'wall', 'blocked', 'tree', 'ore'];
        return !nonWalkable.includes(type);
    }
    
    getNeighbors(x, y) {
        const neighbors = [];
        const directions = [
            { x: 0, y: -1 }, // North
            { x: 1, y: 0 },  // East
            { x: 0, y: 1 },  // South
            { x: -1, y: 0 }  // West
        ];
        
        for (const dir of directions) {
            const nx = x + dir.x;
            const ny = y + dir.y;
            if (nx >= 0 && nx < this.width && ny >= 0 && ny < this.height) {
                neighbors.push({ x: nx, y: ny });
            }
        }
        
        return neighbors;
    }
}