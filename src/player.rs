use crate::actions::Actions;
use crate::gravity::Gravity;
use crate::loading::TextureAssets;
use crate::velocity::Velocity;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Default)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, player_up.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Bundle, Default)]
struct PlayerBundle {
    sprite: SpriteBundle,
    player: Player,
    velocity: Velocity,
    gravity: Gravity,
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn(PlayerBundle {
        sprite: SpriteBundle {
            texture: textures.bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 100.0, y: 100.0 }),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

const UP_VELOCITY: Vec3 = Vec3::new(0.0, 300.0, 0.0);

fn player_up(actions: Res<Actions>, mut player_query: Query<&mut Velocity, With<Player>>) {
    if actions.player_up {
        for mut vel in &mut player_query {
            vel.0 = UP_VELOCITY
        }
    }
}
