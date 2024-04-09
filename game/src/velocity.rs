use bevy::prelude::*;

pub struct VelocityPlugin;

// Velocity in units per second
#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_velocity);
    }
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (vel, mut transform) in &mut query {
        transform.translation += vel.0 * time.delta_seconds()
    }
}
