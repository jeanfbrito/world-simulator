use crate::debug::{DebugLevel, DebugSystem};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct HealthComponent {
    pub current: f32,
    pub maximum: f32,
    pub regeneration_rate: f32,
}

impl HealthComponent {
    pub fn new(maximum: f32) -> Self {
        Self {
            current: maximum,
            maximum,
            regeneration_rate: 0.1,
        }
    }

    pub fn damage(&mut self, amount: f32, entity_name: &str, debug: &DebugSystem) {
        let old_health = self.current;
        self.current = (self.current - amount).max(0.0);

        debug.log(
            DebugLevel::Info,
            "HEALTH",
            &format!(
                "{}: {} → {} (-{} damage)",
                entity_name, old_health, self.current, amount
            ),
        );

        if self.is_dead() {
            debug.log(
                DebugLevel::Warn,
                "HEALTH",
                &format!("{} has died!", entity_name),
            );
        }
    }

    pub fn heal(&mut self, amount: f32, entity_name: &str, debug: &DebugSystem) {
        let old_health = self.current;
        self.current = (self.current + amount).min(self.maximum);

        debug.log(
            DebugLevel::Info,
            "HEALTH",
            &format!(
                "{}: {} → {} (+{} healing)",
                entity_name, old_health, self.current, amount
            ),
        );
    }

    pub fn percentage(&self) -> f32 {
        (self.current / self.maximum) * 100.0
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn is_full(&self) -> bool {
        self.current >= self.maximum
    }
}
