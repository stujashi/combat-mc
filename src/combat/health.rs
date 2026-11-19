use bevy::prelude::*;

/// Hit points for any damageable entity (training dummies, players, etc.).
#[derive(Component, Debug)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    pub fn fraction(&self) -> f32 {
        if self.max <= 0.0 {
            return 0.0;
        }
        (self.current / self.max).clamp(0.0, 1.0)
    }

    pub fn apply_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn reset(&mut self) {
        self.current = self.max;
    }
}

/// Axis-aligned melee hurtbox positioned relative to an entity's `Transform` (feet).
#[derive(Component, Debug)]
pub struct Hurtbox {
    pub center_offset: Vec3,
    pub half_extents: Vec3,
}

impl Hurtbox {
    pub fn bounds(&self, feet_pos: Vec3) -> (Vec3, Vec3) {
        let center = feet_pos + self.center_offset;
        (center - self.half_extents, center + self.half_extents)
    }
}

/// Returns `true` if the ray `origin + dir * t` intersects the box `[min, max]`
/// for any distance `t` in `[0.0, max_distance]`.
pub fn ray_intersects_aabb(
    origin: Vec3,
    dir: Vec3,
    max_distance: f32,
    min: Vec3,
    max: Vec3,
) -> bool {
    let mut t_min = 0.0_f32;
    let mut t_max = max_distance;

    for axis in 0..3 {
        let o = origin[axis];
        let d = dir[axis];
        let b_min = min[axis];
        let b_max = max[axis];

        if d.abs() < 1e-6 {
            if o < b_min || o > b_max {
                return false;
            }
        } else {
            let inv_d = 1.0 / d;
            let mut t1 = (b_min - o) * inv_d;
            let mut t2 = (b_max - o) * inv_d;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            t_min = t_min.max(t1);
            t_max = t_max.min(t2);
            if t_min > t_max {
                return false;
            }
        }
    }

    true
}
