use colored::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::time::Instant;
use bevy::prelude::*;

#[derive(Resource)]
pub struct DebugSystem {
    log_buffer: Arc<Mutex<VecDeque<DebugMessage>>>,
    command_tx: Sender<DebugCommand>,
    command_rx: Arc<Mutex<Receiver<DebugCommand>>>,
    start_time: Instant,
    frame_count: u64,
    show_grid: bool,
    show_agents: bool,
    show_stats: bool,
    verbosity: DebugLevel,
}

#[derive(Clone, Debug)]
pub struct DebugMessage {
    pub timestamp: f64,
    pub level: DebugLevel,
    pub category: String,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DebugLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

pub enum DebugCommand {
    SetVerbosity(DebugLevel),
    ToggleGrid,
    ToggleAgents,
    ToggleStats,
    DumpState,
    ClearBuffer,
    Pause,
    Resume,
    Step,
}

impl DebugSystem {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            log_buffer: Arc::new(Mutex::new(VecDeque::with_capacity(1000))),
            command_tx: tx,
            command_rx: Arc::new(Mutex::new(rx)),
            start_time: Instant::now(),
            frame_count: 0,
            show_grid: false,
            show_agents: true,
            show_stats: true,
            verbosity: DebugLevel::Info,
        }
    }

    pub fn log(&self, level: DebugLevel, category: &str, message: &str) {
        if level > self.verbosity {
            return;
        }

        let timestamp = self.start_time.elapsed().as_secs_f64();
        let msg = DebugMessage {
            timestamp,
            level,
            category: category.to_string(),
            message: message.to_string(),
        };

        // Print to terminal with colors
        let formatted = format!(
            "[{:.3}] [{}] {}: {}",
            timestamp,
            category,
            self.level_str(level),
            message
        );

        match level {
            DebugLevel::Error => println!("{}", formatted.red().bold()),
            DebugLevel::Warn => println!("{}", formatted.yellow()),
            DebugLevel::Info => println!("{}", formatted.green()),
            DebugLevel::Debug => println!("{}", formatted.blue()),
            DebugLevel::Trace => println!("{}", formatted.dimmed()),
        }

        // Store in buffer
        let mut buffer = self.log_buffer.lock().unwrap();
        if buffer.len() >= 1000 {
            buffer.pop_front();
        }
        buffer.push_back(msg);
    }

    fn level_str(&self, level: DebugLevel) -> &str {
        match level {
            DebugLevel::Error => "ERROR",
            DebugLevel::Warn => "WARN",
            DebugLevel::Info => "INFO",
            DebugLevel::Debug => "DEBUG",
            DebugLevel::Trace => "TRACE",
        }
    }

    pub fn process_commands(&mut self) {
        if let Ok(rx) = self.command_rx.try_lock() {
            while let Ok(cmd) = rx.try_recv() {
                match cmd {
                    DebugCommand::SetVerbosity(level) => {
                        self.verbosity = level;
                        self.log(DebugLevel::Info, "DEBUG", &format!("Verbosity set to {:?}", level));
                    }
                    DebugCommand::ToggleGrid => {
                        self.show_grid = !self.show_grid;
                        self.log(DebugLevel::Info, "DEBUG", &format!("Grid display: {}", self.show_grid));
                    }
                    DebugCommand::ToggleAgents => {
                        self.show_agents = !self.show_agents;
                        self.log(DebugLevel::Info, "DEBUG", &format!("Agents display: {}", self.show_agents));
                    }
                    DebugCommand::ToggleStats => {
                        self.show_stats = !self.show_stats;
                        self.log(DebugLevel::Info, "DEBUG", &format!("Stats display: {}", self.show_stats));
                    }
                    DebugCommand::ClearBuffer => {
                        self.log_buffer.lock().unwrap().clear();
                        println!("{}", "Debug buffer cleared".yellow());
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn display_stats(&self) {
        if !self.show_stats {
            return;
        }

        let elapsed = self.start_time.elapsed().as_secs_f64();
        let fps = self.frame_count as f64 / elapsed;

        println!("\n{}", "=== STATS ===".cyan().bold());
        println!("  Time: {:.1}s", elapsed);
        println!("  Frame: {}", self.frame_count);
        println!("  FPS: {:.1}", fps);
    }

    pub fn update_frame(&mut self) {
        self.frame_count += 1;
    }

    pub fn get_command_sender(&self) -> Sender<DebugCommand> {
        self.command_tx.clone()
    }
}

// Bevy plugin for debug system
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugSystem::new())
            .add_systems(Update, debug_update_system)
            .add_systems(PostUpdate, debug_display_system);
    }
}

fn debug_update_system(
    mut debug: ResMut<DebugSystem>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    debug.process_commands();
    debug.update_frame();

    // Keyboard shortcuts for debug commands
    if keyboard.just_pressed(KeyCode::F1) {
        debug.show_stats = !debug.show_stats;
    }
    if keyboard.just_pressed(KeyCode::F2) {
        debug.show_grid = !debug.show_grid;
    }
    if keyboard.just_pressed(KeyCode::F3) {
        debug.show_agents = !debug.show_agents;
    }
    if keyboard.just_pressed(KeyCode::F5) {
        debug.log_buffer.lock().unwrap().clear();
    }

}

fn debug_display_system(
    debug: Res<DebugSystem>,
) {
    // Terminal display handled through logging
}