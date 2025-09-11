use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
pub struct PreTickEvent {
    pub tick: u64,
    pub delta_time: f32,
}

#[derive(Event, Debug, Clone)]
pub struct TickEvent {
    pub tick: u64,
    pub sub_tick: u32,
}

#[derive(Event, Debug, Clone)]
pub struct PostTickEvent {
    pub tick: u64,
    pub ticks_processed: u32,
}

#[derive(Event, Debug, Clone)]
pub struct SimulationSpeedChangedEvent {
    pub old_speed: super::tick_config::SimulationSpeed,
    pub new_speed: super::tick_config::SimulationSpeed,
}

#[derive(Event, Debug, Clone)]
pub struct SimulationPausedEvent {
    pub tick: u64,
}

#[derive(Event, Debug, Clone)]
pub struct SimulationResumedEvent {
    pub tick: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum TickSystemSet {
    PreTick,
    MainTick,
    PostTick,
}

pub struct TickEventPlugin;

impl Plugin for TickEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PreTickEvent>()
            .add_event::<TickEvent>()
            .add_event::<PostTickEvent>()
            .add_event::<SimulationSpeedChangedEvent>()
            .add_event::<SimulationPausedEvent>()
            .add_event::<SimulationResumedEvent>()
            .configure_sets(
                Update,
                (
                    TickSystemSet::PreTick,
                    TickSystemSet::MainTick,
                    TickSystemSet::PostTick,
                )
                    .chain(),
            );
    }
}