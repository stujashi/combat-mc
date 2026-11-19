pub mod state;
mod viewmodel;
pub mod weapon;

use bevy::prelude::*;

use crate::player::{Player, toggle_cursor_lock};

use state::{CombatInput, Combatant, gather_combat_input, step_combat_state};
use viewmodel::{animate_sword_viewmodel, spawn_sword_viewmodel};
use weapon::WeaponData;

const SWORD_DATA_PATH: &str = "assets/weapons/sword.ron";

/// Feature plugin responsible for melee weapons, combat states, and weapon viewmodels.
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        let weapon = WeaponData::load(SWORD_DATA_PATH);

        app.insert_resource(weapon)
            .add_systems(PostStartup, (attach_player_combat, spawn_sword_viewmodel))
            .add_systems(
                Update,
                (
                    gather_combat_input.before(toggle_cursor_lock),
                    animate_sword_viewmodel,
                ),
            )
            .add_systems(FixedUpdate, step_combat_state);
    }
}

/// Adds combat components onto the player spawned by `PlayerPlugin`.
fn attach_player_combat(mut commands: Commands, player: Single<Entity, With<Player>>) {
    commands
        .entity(*player)
        .insert((Combatant::default(), CombatInput::default()));
}
