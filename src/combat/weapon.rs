use std::fs;

use bevy::prelude::Resource;
use serde::Deserialize;

/// Balance and timing values for a melee weapon, loaded from a RON file.
#[derive(Resource, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct WeaponData {
    pub name: String,
    pub damage: f32,
    pub reach: f32,
    pub windup_secs: f32,
    pub active_secs: f32,
    pub recovery_secs: f32,
}

impl WeaponData {
    /// Reads and parses a weapon RON file from disk.
    pub fn load(path: &str) -> Self {
        let contents = fs::read_to_string(path)
            .unwrap_or_else(|err| panic!("Failed to read weapon data at '{path}': {err}"));

        ron::from_str(&contents)
            .unwrap_or_else(|err| panic!("Failed to parse weapon data at '{path}': {err}"))
    }
}
