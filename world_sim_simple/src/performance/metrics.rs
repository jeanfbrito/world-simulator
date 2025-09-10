use bevy::prelude::*;
use std::time::{Duration, Instant};
use crate::debug::{DebugSystem, DebugLevel};

#[derive(Resource)]
pub struct PerformanceMetrics {
    frame_times: Vec<Duration>,
    last_frame: Instant,
    fps: f32,
    avg_frame_time: Duration,
    min_frame_time: Duration,
    max_frame_time: Duration,
    update_interval: f32,
    time_since_update: f32,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            frame_times: Vec::with_capacity(120),
            last_frame: Instant::now(),
            fps: 60.0,
            avg_frame_time: Duration::from_millis(16),
            min_frame_time: Duration::from_secs(1),
            max_frame_time: Duration::ZERO,
            update_interval: 1.0, // Update stats every second
            time_since_update: 0.0,
        }
    }
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        info!("[METRICS] Performance metrics tracking initialized");
        Self::default()
    }
    
    pub fn record_frame(&mut self, delta: Duration) {
        self.frame_times.push(delta);
        
        // Keep only last 120 frames
        if self.frame_times.len() > 120 {
            self.frame_times.remove(0);
        }
        
        // Update min/max
        if delta < self.min_frame_time {
            self.min_frame_time = delta;
        }
        if delta > self.max_frame_time {
            self.max_frame_time = delta;
        }
    }
    
    pub fn update_stats(&mut self) {
        if self.frame_times.is_empty() {
            return;
        }
        
        // Calculate average
        let sum: Duration = self.frame_times.iter().sum();
        self.avg_frame_time = sum / self.frame_times.len() as u32;
        
        // Calculate FPS from average frame time
        if self.avg_frame_time.as_secs_f32() > 0.0 {
            self.fps = 1.0 / self.avg_frame_time.as_secs_f32();
        }
        
        // Reset min/max for next interval
        self.min_frame_time = Duration::from_secs(1);
        self.max_frame_time = Duration::ZERO;
    }
    
    pub fn get_fps(&self) -> f32 {
        self.fps
    }
    
    pub fn get_frame_time_ms(&self) -> f32 {
        self.avg_frame_time.as_secs_f32() * 1000.0
    }
    
    pub fn get_stats(&self) -> (f32, f32, f32, f32) {
        (
            self.fps,
            self.avg_frame_time.as_secs_f32() * 1000.0,
            self.min_frame_time.as_secs_f32() * 1000.0,
            self.max_frame_time.as_secs_f32() * 1000.0,
        )
    }
}

pub struct FrameTimer {
    start: Instant,
    name: String,
}

impl FrameTimer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            name: name.into(),
        }
    }
    
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
    
    pub fn elapsed_ms(&self) -> f32 {
        self.elapsed().as_secs_f32() * 1000.0
    }
}

impl Drop for FrameTimer {
    fn drop(&mut self) {
        let elapsed = self.elapsed_ms();
        if elapsed > 1.0 {
            // Only log slow operations
            trace!("[TIMER] {} took {:.2}ms", self.name, elapsed);
        }
    }
}

pub fn performance_metrics_system(
    mut metrics: ResMut<PerformanceMetrics>,
    time: Res<Time>,
    debug: Res<DebugSystem>,
) {
    let now = Instant::now();
    let delta = now.duration_since(metrics.last_frame);
    metrics.last_frame = now;
    
    metrics.record_frame(delta);
    metrics.time_since_update += time.delta_secs();
    
    // Update stats periodically
    if metrics.time_since_update >= metrics.update_interval {
        metrics.update_stats();
        metrics.time_since_update = 0.0;
        
        let (fps, avg_ms, min_ms, max_ms) = metrics.get_stats();
        
        debug.log(
            DebugLevel::Info,
            "METRICS",
            &format!("FPS: {:.1} | Frame: {:.1}ms (min: {:.1}ms, max: {:.1}ms)", 
                fps, avg_ms, min_ms, max_ms)
        );
        
        info!("[METRICS] FPS: {:.1} | Frame time: {:.1}ms (min: {:.1}ms, max: {:.1}ms)", 
            fps, avg_ms, min_ms, max_ms);
        
        // Warn if performance is poor
        if fps < 30.0 {
            warn!("[METRICS] Low FPS detected: {:.1}", fps);
            debug.log(
                DebugLevel::Error,
                "METRICS",
                &format!("Low FPS warning: {:.1}", fps)
            );
        }
    }
}