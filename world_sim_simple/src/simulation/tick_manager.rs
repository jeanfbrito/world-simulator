use bevy::prelude::*;
use super::{
    TickAccumulator, SimulationSpeed, SimulationTickState,
    SimulationSpeedChangedEvent, SimulationPausedEvent, SimulationResumedEvent
};

#[derive(Resource, Default)]
pub struct TickManager {
    requested_speed: Option<SimulationSpeed>,
}

impl TickManager {
    pub fn set_speed(&mut self, speed: SimulationSpeed) {
        self.requested_speed = Some(speed);
    }

    pub fn pause(&mut self) {
        self.set_speed(SimulationSpeed::Paused);
    }

    pub fn resume(&mut self) {
        self.set_speed(SimulationSpeed::Normal);
    }

    pub fn speed_up(&mut self, current: SimulationSpeed) {
        let new_speed = match current {
            SimulationSpeed::Paused => SimulationSpeed::Slow,
            SimulationSpeed::Slow => SimulationSpeed::Normal,
            SimulationSpeed::Normal => SimulationSpeed::Fast,
            SimulationSpeed::Fast => SimulationSpeed::VeryFast,
            SimulationSpeed::VeryFast => SimulationSpeed::UltraFast,
            SimulationSpeed::UltraFast => SimulationSpeed::UltraFast,
        };
        self.set_speed(new_speed);
    }

    pub fn slow_down(&mut self, current: SimulationSpeed) {
        let new_speed = match current {
            SimulationSpeed::Paused => SimulationSpeed::Paused,
            SimulationSpeed::Slow => SimulationSpeed::Paused,
            SimulationSpeed::Normal => SimulationSpeed::Slow,
            SimulationSpeed::Fast => SimulationSpeed::Normal,
            SimulationSpeed::VeryFast => SimulationSpeed::Fast,
            SimulationSpeed::UltraFast => SimulationSpeed::VeryFast,
        };
        self.set_speed(new_speed);
    }
}

pub fn tick_manager_system(
    mut manager: ResMut<TickManager>,
    mut accumulator: ResMut<TickAccumulator>,
    tick_state: Res<SimulationTickState>,
    mut speed_events: EventWriter<SimulationSpeedChangedEvent>,
    mut pause_events: EventWriter<SimulationPausedEvent>,
    mut resume_events: EventWriter<SimulationResumedEvent>,
) {
    if let Some(requested_speed) = manager.requested_speed.take() {
        if let Some((old_speed, new_speed)) = accumulator.set_speed(requested_speed) {
            // Emit appropriate events
            speed_events.send(SimulationSpeedChangedEvent {
                old_speed,
                new_speed,
            });

            // Check for pause/resume
            if old_speed != SimulationSpeed::Paused && new_speed == SimulationSpeed::Paused {
                pause_events.send(SimulationPausedEvent {
                    tick: tick_state.current_tick,
                });
            } else if old_speed == SimulationSpeed::Paused && new_speed != SimulationSpeed::Paused {
                resume_events.send(SimulationResumedEvent {
                    tick: tick_state.current_tick,
                });
            }

            info!(
                "Simulation speed changed from {} to {}",
                old_speed.name(),
                new_speed.name()
            );
        }
    }
}

pub fn handle_speed_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut manager: ResMut<TickManager>,
    accumulator: Res<TickAccumulator>,
) {
    let current = accumulator.get_speed();

    if keyboard.just_pressed(KeyCode::Space) {
        if current == SimulationSpeed::Paused {
            manager.resume();
        } else {
            manager.pause();
        }
    }

    if keyboard.just_pressed(KeyCode::Equal) || keyboard.just_pressed(KeyCode::NumpadAdd) {
        manager.speed_up(current);
    }

    if keyboard.just_pressed(KeyCode::Minus) || keyboard.just_pressed(KeyCode::NumpadSubtract) {
        manager.slow_down(current);
    }

    // Number keys for direct speed selection
    if keyboard.just_pressed(KeyCode::Digit0) {
        manager.pause();
    }
    if keyboard.just_pressed(KeyCode::Digit1) {
        manager.set_speed(SimulationSpeed::Slow);
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        manager.set_speed(SimulationSpeed::Normal);
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        manager.set_speed(SimulationSpeed::Fast);
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        manager.set_speed(SimulationSpeed::VeryFast);
    }
    if keyboard.just_pressed(KeyCode::Digit5) {
        manager.set_speed(SimulationSpeed::UltraFast);
    }
}