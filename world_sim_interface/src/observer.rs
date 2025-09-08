//! Observer pattern for engine events

use super::{EngineEvent, WorldSnapshot};

/// Trait for components that want to observe engine events
pub trait EngineObserver: Send + Sync {
    /// Called when events occur
    fn on_events(&mut self, events: &[EngineEvent]);
    
    /// Called when a full snapshot is available
    fn on_snapshot(&mut self, snapshot: &WorldSnapshot);
    
    /// Return true if this observer wants full snapshots
    fn wants_snapshots(&self) -> bool {
        false
    }
}