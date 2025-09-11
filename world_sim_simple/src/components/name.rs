use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct NameComponent {
    pub name: String,
    pub display_name: String,
}

impl NameComponent {
    pub fn new(name: impl Into<String>) -> Self {
        let n = name.into();
        Self {
            display_name: n.clone(),
            name: n,
        }
    }

    pub fn with_title(name: impl Into<String>, title: impl Into<String>) -> Self {
        let n = name.into();
        let t = title.into();
        Self {
            name: n.clone(),
            display_name: format!("{} {}", t, n),
        }
    }
}
