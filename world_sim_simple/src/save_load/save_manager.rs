use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use super::SaveState;

#[derive(Debug)]
pub enum SaveError {
    IoError(std::io::Error),
    SerializationError(String),
    DeserializationError(String),
    ValidationError(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveFormat {
    Json,
    Binary,
}

#[derive(Resource)]
pub struct SaveManager {
    save_directory: PathBuf,
    autosave_enabled: bool,
    autosave_interval: f32,
    autosave_timer: f32,
    save_format: SaveFormat,
    max_saves: usize,
    max_autosaves: usize,
}

impl Default for SaveManager {
    fn default() -> Self {
        let save_dir = PathBuf::from("saves");
        
        // Create saves directory if it doesn't exist
        if !save_dir.exists() {
            fs::create_dir_all(&save_dir).unwrap_or_else(|e| {
                error!("[SAVE] Failed to create saves directory: {}", e);
            });
        }
        
        info!("[SAVE] Save manager initialized with directory: {:?}", save_dir);
        
        Self {
            save_directory: save_dir,
            autosave_enabled: true,
            autosave_interval: 300.0, // 5 minutes
            autosave_timer: 0.0,
            save_format: SaveFormat::Json,
            max_saves: 10,
            max_autosaves: 3,
        }
    }
}

impl SaveManager {
    pub fn save_game(&self, save_state: &SaveState, filename: &str) -> Result<(), SaveError> {
        let file_path = self.save_directory.join(filename);
        
        info!("[SAVE] Saving game to: {:?}", file_path);
        
        // Validate save state
        save_state.validate()
            .map_err(|e| SaveError::ValidationError(e))?;
        
        match self.save_format {
            SaveFormat::Json => self.save_json(save_state, &file_path),
            SaveFormat::Binary => self.save_binary(save_state, &file_path),
        }
    }
    
    pub fn load_game(&self, filename: &str) -> Result<SaveState, SaveError> {
        let file_path = self.save_directory.join(filename);
        
        info!("[SAVE] Loading game from: {:?}", file_path);
        
        if !file_path.exists() {
            return Err(SaveError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Save file not found: {:?}", file_path),
            )));
        }
        
        let save_state = match self.save_format {
            SaveFormat::Json => self.load_json(&file_path),
            SaveFormat::Binary => self.load_binary(&file_path),
        }?;
        
        // Validate loaded save
        save_state.validate()
            .map_err(|e| SaveError::ValidationError(e))?;
        
        info!("[SAVE] Game loaded successfully from: {:?}", file_path);
        Ok(save_state)
    }
    
    fn save_json(&self, save_state: &SaveState, path: &Path) -> Result<(), SaveError> {
        let json = serde_json::to_string_pretty(save_state)
            .map_err(|e| SaveError::SerializationError(e.to_string()))?;
        
        let mut file = fs::File::create(path)
            .map_err(SaveError::IoError)?;
        
        file.write_all(json.as_bytes())
            .map_err(SaveError::IoError)?;
        
        info!("[SAVE] JSON save completed: {} bytes", json.len());
        Ok(())
    }
    
    fn load_json(&self, path: &Path) -> Result<SaveState, SaveError> {
        let mut file = fs::File::open(path)
            .map_err(SaveError::IoError)?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(SaveError::IoError)?;
        
        let save_state = serde_json::from_str(&contents)
            .map_err(|e| SaveError::DeserializationError(e.to_string()))?;
        
        info!("[SAVE] JSON load completed: {} bytes", contents.len());
        Ok(save_state)
    }
    
    fn save_binary(&self, save_state: &SaveState, path: &Path) -> Result<(), SaveError> {
        let bytes = bincode::serialize(save_state)
            .map_err(|e| SaveError::SerializationError(e.to_string()))?;
        
        let mut file = fs::File::create(path)
            .map_err(SaveError::IoError)?;
        
        file.write_all(&bytes)
            .map_err(SaveError::IoError)?;
        
        info!("[SAVE] Binary save completed: {} bytes", bytes.len());
        Ok(())
    }
    
    fn load_binary(&self, path: &Path) -> Result<SaveState, SaveError> {
        let mut file = fs::File::open(path)
            .map_err(SaveError::IoError)?;
        
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .map_err(SaveError::IoError)?;
        
        let save_state = bincode::deserialize(&bytes)
            .map_err(|e| SaveError::DeserializationError(e.to_string()))?;
        
        info!("[SAVE] Binary load completed: {} bytes", bytes.len());
        Ok(save_state)
    }
    
    pub fn quick_save(&self, save_state: &SaveState) -> Result<(), SaveError> {
        let filename = "quicksave.json";
        info!("[SAVE] Quick save triggered");
        self.save_game(save_state, filename)
    }
    
    pub fn quick_load(&self) -> Result<SaveState, SaveError> {
        let filename = "quicksave.json";
        info!("[SAVE] Quick load triggered");
        self.load_game(filename)
    }
    
    pub fn autosave(&self, save_state: &SaveState) -> Result<(), SaveError> {
        let filename = format!("autosave_{}.json", 
            save_state.timestamp % self.max_autosaves as u64);
        info!("[SAVE] Autosave to: {}", filename);
        self.save_game(save_state, &filename)
    }
    
    pub fn list_saves(&self) -> Result<Vec<String>, SaveError> {
        let entries = fs::read_dir(&self.save_directory)
            .map_err(SaveError::IoError)?;
        
        let mut saves = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".json") || name.ends_with(".bin") {
                        saves.push(name.to_string());
                    }
                }
            }
        }
        
        info!("[SAVE] Found {} save files", saves.len());
        Ok(saves)
    }
    
    pub fn delete_save(&self, filename: &str) -> Result<(), SaveError> {
        let file_path = self.save_directory.join(filename);
        fs::remove_file(file_path)
            .map_err(SaveError::IoError)?;
        info!("[SAVE] Deleted save: {}", filename);
        Ok(())
    }
    
    pub fn update_autosave_timer(&mut self, delta: f32) {
        if self.autosave_enabled {
            self.autosave_timer += delta;
        }
    }
    
    pub fn should_autosave(&self) -> bool {
        self.autosave_enabled && self.autosave_timer >= self.autosave_interval
    }
    
    pub fn reset_autosave_timer(&mut self) {
        self.autosave_timer = 0.0;
    }
    
    pub fn get_autosave_interval(&self) -> f32 {
        self.autosave_interval
    }
    
    pub fn set_autosave_enabled(&mut self, enabled: bool) {
        self.autosave_enabled = enabled;
        info!("[SAVE] Autosave {}", if enabled { "enabled" } else { "disabled" });
    }
    
    pub fn set_autosave_interval(&mut self, interval: f32) {
        self.autosave_interval = interval;
        info!("[SAVE] Autosave interval set to {} seconds", interval);
    }
}