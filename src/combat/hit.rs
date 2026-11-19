use bevy::prelude::*;

use crate::player::config::PlayerConfig;
use crate::player::{Player, PlayerLook};

use super::dummy::{DummyConfig, HitFeedback};
use super::health::{Health, Hurtbox, ray_intersects_aabb};
use super::state::{CombatState, Combatant};
use super::weapon::WeaponData;

/// Checks melee hit registration during the `Active` window of a swing (`FixedUpdate`).
/// Uses only simulation data (feet transform + yaw/pitch + eye height) so a future
/// headless multiplayer server can run the exact same hit check.
pub fn detect_melee_hits(
    player_config: Res<PlayerConfig>,
    weapon: Res<WeaponData>,
    dummy_config: Res<DummyConfig>,
    mut attacker: Single<(&Transform, &PlayerLook, &mut Combatant), With<Player>>,
    mut targets: Query<
        (&Transform, &Hurtbox, &mut Health, Option<&mut HitFeedback>),
        Without<Player>,
    >,
) {
    let (player_transform, look, ref mut combatant) = *attacker;

    if combatant.state != CombatState::Active || combatant.hit_this_swing {
        return;
    }

    let ray_origin = player_transform.translation + Vec3::Y * player_config.eye_height;
    let ray_dir = Quat::from_euler(EulerRot::YXZ, look.yaw, look.pitch, 0.0) * Vec3::NEG_Z;

    for (target_transform, hurtbox, mut health, feedback) in &mut targets {
        if !health.is_alive() {
            continue;
        }

        let (min, max) = hurtbox.bounds(target_transform.translation);
        if ray_intersects_aabb(ray_origin, ray_dir, weapon.reach, min, max) {
            health.apply_damage(weapon.damage);
            combatant.hit_this_swing = true;

            if let Some(mut feedback) = feedback {
                feedback.flash_remaining = dummy_config.hit_flash_secs;
            }

            info!(
                "Hit target with {} for {:.0} dmg ({:.0}/{:.0} HP)",
                weapon.name, weapon.damage, health.current, health.max
            );
            break;
        }
    }
}
