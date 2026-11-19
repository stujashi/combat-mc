pub mod dummy;
pub mod health;
mod hit;
pub mod state;
mod viewmodel;
pub mod weapon;

use bevy::prelude::*;

use crate::player::{Player, toggle_cursor_lock};

use dummy::{DummyConfig, spawn_training_dummy, step_dummy_respawn, update_dummy_visuals};
use hit::detect_melee_hits;
use state::{CombatInput, Combatant, gather_combat_input, step_combat_state};
use viewmodel::{animate_sword_viewmodel, spawn_sword_viewmodel};
use weapon::WeaponData;

const SWORD_DATA_PATH: &str = "assets/weapons/sword.ron";
const DUMMY_CONFIG_PATH: &str = "assets/config/dummy.ron";

/// Feature plugin responsible for melee weapons, combat states, hit detection, and dummies.
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        let weapon = WeaponData::load(SWORD_DATA_PATH);
        let dummy_config = DummyConfig::load(DUMMY_CONFIG_PATH);

        app.insert_resource(weapon)
            .insert_resource(dummy_config)
            .add_systems(Startup, spawn_training_dummy)
            .add_systems(PostStartup, (attach_player_combat, spawn_sword_viewmodel))
            .add_systems(
                Update,
                (
                    gather_combat_input.before(toggle_cursor_lock),
                    animate_sword_viewmodel,
                    update_dummy_visuals,
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    step_combat_state,
                    detect_melee_hits.after(step_combat_state),
                    step_dummy_respawn.after(detect_melee_hits),
                ),
            );
    }
}

/// Adds combat components onto the player spawned by `PlayerPlugin`.
fn attach_player_combat(mut commands: Commands, player: Single<Entity, With<Player>>) {
    commands
        .entity(*player)
        .insert((Combatant::default(), CombatInput::default()));
}
