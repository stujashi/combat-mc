use bevy::prelude::*;

use crate::world::voxel_world::VoxelWorld;

use super::config::PlayerConfig;
use super::{Player, PlayerLook};

const COLLISION_EPSILON: f32 = 0.0001;

/// Raw movement intent captured from the keyboard.
/// Kept as a separate component so a future multiplayer client can send this
/// exact input struct to the server.
#[derive(Component, Default)]
pub struct PlayerInput {
    /// Local XZ intent: `x` = right/left, `y` = forward/back.
    pub move_axes: Vec2,
    /// Set true when Space is pressed; consumed on the next `FixedUpdate` tick.
    pub jump_queued: bool,
}

/// Physics state for a moving player body.
#[derive(Component, Default)]
pub struct PlayerBody {
    pub velocity: Vec3,
    pub grounded: bool,
}

/// Samples keyboard input every frame (`Update`) so short key taps like Space
/// are never dropped between fixed physics ticks.
pub fn gather_player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut input: Single<&mut PlayerInput, With<Player>>,
) {
    let mut axes = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        axes.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        axes.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        axes.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        axes.x -= 1.0;
    }

    input.move_axes = axes.normalize_or_zero();

    if keys.just_pressed(KeyCode::Space) {
        input.jump_queued = true;
    }
}

/// Steps movement, gravity, jumping, and voxel AABB collision on a fixed tick (`FixedUpdate`)
/// so physics behaves identically regardless of frame rate (and matches a future server).
pub fn step_player_movement(
    time: Res<Time<Fixed>>,
    config: Res<PlayerConfig>,
    world: Res<VoxelWorld>,
    mut player: Single<(&mut PlayerInput, &mut PlayerBody, &PlayerLook, &mut Transform), With<Player>>,
) {
    let (ref mut input, ref mut body, look, ref mut transform) = *player;
    let dt = time.delta_secs();

    // Turn local (right, forward) input into world XZ direction using the player's yaw.
    // In Bevy, forward is -Z and right is +X.
    let local_dir = Vec3::new(input.move_axes.x, 0.0, -input.move_axes.y);
    let world_dir = Quat::from_rotation_y(look.yaw) * local_dir;

    body.velocity.x = world_dir.x * config.move_speed;
    body.velocity.z = world_dir.z * config.move_speed;

    if body.grounded && input.jump_queued {
        body.velocity.y = config.jump_speed;
        body.grounded = false;
    }
    input.jump_queued = false;

    body.velocity.y -= config.gravity * dt;

    let hw = config.half_width;
    let height = config.height;
    let mut pos = transform.translation;

    // Resolve X axis.
    let dx = body.velocity.x * dt;
    let next_x = pos.x + dx;
    if overlaps_solid(&world, Vec3::new(next_x, pos.y, pos.z), hw, height) {
        if dx > 0.0 {
            pos.x = (next_x + hw - COLLISION_EPSILON).floor() - hw;
        } else if dx < 0.0 {
            pos.x = (next_x - hw + COLLISION_EPSILON).floor() + 1.0 + hw;
        }
        body.velocity.x = 0.0;
    } else {
        pos.x = next_x;
    }

    // Resolve Z axis.
    let dz = body.velocity.z * dt;
    let next_z = pos.z + dz;
    if overlaps_solid(&world, Vec3::new(pos.x, pos.y, next_z), hw, height) {
        if dz > 0.0 {
            pos.z = (next_z + hw - COLLISION_EPSILON).floor() - hw;
        } else if dz < 0.0 {
            pos.z = (next_z - hw + COLLISION_EPSILON).floor() + 1.0 + hw;
        }
        body.velocity.z = 0.0;
    } else {
        pos.z = next_z;
    }

    // Resolve Y axis (ground + ceiling).
    body.grounded = false;
    let dy = body.velocity.y * dt;
    let next_y = pos.y + dy;
    if overlaps_solid(&world, Vec3::new(pos.x, next_y, pos.z), hw, height) {
        if dy > 0.0 {
            pos.y = (next_y + height - COLLISION_EPSILON).floor() - height;
        } else if dy < 0.0 {
            pos.y = (next_y + COLLISION_EPSILON).floor() + 1.0;
            body.grounded = true;
        }
        body.velocity.y = 0.0;
    } else {
        pos.y = next_y;
    }

    transform.translation = pos;
}

/// Returns true if the player's axis-aligned bounding box (feet at `pos`)
/// overlaps any solid voxel in `world`.
fn overlaps_solid(world: &VoxelWorld, pos: Vec3, half_width: f32, height: f32) -> bool {
    let min_x = (pos.x - half_width + COLLISION_EPSILON).floor() as i32;
    let max_x = (pos.x + half_width - COLLISION_EPSILON).floor() as i32;
    let min_y = (pos.y + COLLISION_EPSILON).floor() as i32;
    let max_y = (pos.y + height - COLLISION_EPSILON).floor() as i32;
    let min_z = (pos.z - half_width + COLLISION_EPSILON).floor() as i32;
    let max_z = (pos.z + half_width - COLLISION_EPSILON).floor() as i32;

    for y in min_y..=max_y {
        for z in min_z..=max_z {
            for x in min_x..=max_x {
                if world.get_block(x, y, z).is_solid() {
                    return true;
                }
            }
        }
    }

    false
}
