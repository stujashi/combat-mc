use std::fs;

use bevy::prelude::Resource;
use serde::Deserialize;

/// Player tuning values loaded from `assets/config/player.ron`.
#[derive(Resource, Deserialize, Debug, Clone)]
pub struct PlayerConfig {
    pub mouse_sensitivity: f32,
    pub eye_height: f32,
    pub max_pitch_degrees: f32,
    pub move_speed: f32,
    pub jump_speed: f32,
    pub gravity: f32,
    pub half_width: f32,
    pub height: f32,
}

impl PlayerConfig {
    /// Reads and parses a RON config file from disk.
    pub fn load(path: &str) -> Self {
        let contents = fs::read_to_string(path)
            .unwrap_or_else(|err| panic!("Failed to read player config at '{path}': {err}"));

        ron::from_str(&contents)
            .unwrap_or_else(|err| panic!("Failed to parse player config at '{path}': {err}"))
    }
}
