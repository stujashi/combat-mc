use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::player::Player;

use super::weapon::WeaponData;

/// All phases of melee combat.
/// In Step 2a, Left-Click cycles `Idle -> Windup -> Active -> Recovery -> Idle`;
/// the remaining states (`Block`, `Parry`, `Stunned`, `Dodge`) will be wired up next.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum CombatState {
    #[default]
    Idle,
    Windup,
    Active,
    Recovery,
    Block,
    Parry,
    Stunned,
    Dodge,
}

/// Raw combat intent buffered in `Update` and consumed in `FixedUpdate`
/// (kept separate so a future multiplayer client can send it to the server).
#[derive(Component, Default)]
pub struct CombatInput {
    pub attack_queued: bool,
}

/// Server-ready combat state machine attached to any fighting entity.
#[derive(Component, Debug, Default)]
pub struct Combatant {
    pub state: CombatState,
    pub state_elapsed: f32,
    pub state_duration: f32,
}

impl Combatant {
    /// Switches to a new combat state and resets its phase timer.
    pub fn enter(&mut self, next: CombatState, duration: f32) {
        self.state = next;
        self.state_elapsed = 0.0;
        self.state_duration = duration;
    }

    /// Returns progress through the current state from `0.0` (start) to `1.0` (end).
    pub fn progress(&self) -> f32 {
        if self.state_duration <= 0.0 {
            return 0.0;
        }
        (self.state_elapsed / self.state_duration).clamp(0.0, 1.0)
    }
}

/// Captures Left-Click attack input when the cursor is already locked.
pub fn gather_combat_input(
    cursor: Single<&CursorOptions, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut input: Single<&mut CombatInput, With<Player>>,
) {
    if cursor.grab_mode == CursorGrabMode::None {
        return;
    }

    if mouse_buttons.just_pressed(MouseButton::Left) {
        input.attack_queued = true;
    }
}

/// Advances the combat state machine on a fixed tick (`FixedUpdate`).
pub fn step_combat_state(
    time: Res<Time<Fixed>>,
    weapon: Res<WeaponData>,
    mut query: Query<(&mut Combatant, &mut CombatInput)>,
) {
    let dt = time.delta_secs();

    for (mut combatant, mut input) in &mut query {
        combatant.state_elapsed += dt;

        match combatant.state {
            CombatState::Idle => {
                if input.attack_queued {
                    combatant.enter(CombatState::Windup, weapon.windup_secs);
                }
            }
            CombatState::Windup => {
                if combatant.state_elapsed >= combatant.state_duration {
                    combatant.enter(CombatState::Active, weapon.active_secs);
                }
            }
            CombatState::Active => {
                if combatant.state_elapsed >= combatant.state_duration {
                    combatant.enter(CombatState::Recovery, weapon.recovery_secs);
                }
            }
            CombatState::Recovery => {
                if combatant.state_elapsed >= combatant.state_duration {
                    combatant.enter(CombatState::Idle, 0.0);
                }
            }
            CombatState::Block | CombatState::Parry | CombatState::Stunned | CombatState::Dodge => {
                if combatant.state_duration > 0.0
                    && combatant.state_elapsed >= combatant.state_duration
                {
                    combatant.enter(CombatState::Idle, 0.0);
                }
            }
        }

        input.attack_queued = false;
    }
}
