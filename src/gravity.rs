use bevy::prelude::*;

use crate::velocity::Velocity;

pub struct GravityPlugin;

#[derive(Component)]
pub struct Gravity(pub Vec3);

const EARTH_GRAVITY_Y: f32 = -9.8;

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec3 {
            x: 0.0,
            y: EARTH_GRAVITY_Y * 40.0,
            z: 0.0,
        })
    }
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_gravity);
    }
}

fn apply_gravity(mut query: Query<(&Gravity, &mut Velocity)>, time: Res<Time>) {
    for (grav, mut vel) in &mut query {
        vel.0 += grav.0 * time.delta_seconds()
    }
}
