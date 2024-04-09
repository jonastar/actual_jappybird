use crate::actions::Actions;
use crate::collision::{Collider, CollisionEvent};
use crate::gravity::Gravity;
use crate::loading::TextureAssets;
use crate::obstacle::Obstacle;
use crate::velocity::Velocity;
use crate::{cleanup_entities_with, GameState, PROJECTION_SIZE};
use bevy::math::bounding::BoundingCircle;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Default)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(OnExit(GameState::GameOver), cleanup_entities_with::<Player>)
            .add_systems(
                Update,
                (
                    player_up,
                    player_follow_camera,
                    handle_player_collision,
                    player_out_of_bounds_check,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    sprite: SpriteBundle,
    player: Player,
    velocity: Velocity,
    gravity: Gravity,
    collider: Collider,
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn(PlayerBundle {
        sprite: SpriteBundle {
            texture: textures.bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 50.0, y: 50.0 }),
                ..Default::default()
            },
            ..Default::default()
        },
        velocity: Velocity(Vec3 {
            x: 100.0,
            y: 0.0,
            z: 0.0,
        }),
        collider: Collider::Circle(BoundingCircle::new(Vec2::ZERO, 25.0)),
        player: Player,
        gravity: Gravity::default(),
    });
}

const UP_VELOCITY: Vec3 = Vec3::new(0.0, 300.0, 0.0);

fn player_up(actions: Res<Actions>, mut player_query: Query<&mut Velocity, With<Player>>) {
    if actions.player_up {
        for mut vel in &mut player_query {
            vel.0.y = UP_VELOCITY.y
        }
    }
}

fn player_follow_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    for mut camera in &mut camera_query {
        camera.translation.x = player_transform.translation.x;
    }
}

fn handle_player_collision(
    player_query: Query<&Transform, With<Player>>,
    mut collisions: EventReader<CollisionEvent>,
    obstacles: Query<(), With<Obstacle>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in collisions.read() {
        let Ok((player, other)) = event.get_query_entity(&player_query) else {
            continue;
        };

        info!("Player collided at {player:?} with {other:?}");
        if obstacles.contains(other) {
            next_state.set(GameState::GameOver)
        }
    }
}

fn player_out_of_bounds_check(
    player_query: Query<&Transform, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for player in &player_query {
        if player.translation.y > (PROJECTION_SIZE.y / 2.0)
            || player.translation.y < (-PROJECTION_SIZE.y / 2.0)
        {
            next_state.set(GameState::GameOver)
        }
    }
}
