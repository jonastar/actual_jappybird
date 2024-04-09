use bevy::prelude::*;

use crate::{obstacle::SpawnObstacleEvent, player::Player, GameState, PROJECTION_SIZE};

pub struct GameSessionPlugin;

impl Plugin for GameSessionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTimer(0.0))
            .add_systems(
                Update,
                (update_timer, obstacle_spawner).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::Playing), reset_timer);
    }
}

#[derive(Resource)]
pub struct GameTimer(pub f32);

impl GameTimer {
    pub fn get_score(&self) -> i32 {
        (self.0 * 10.0).floor() as i32
    }
}

fn reset_timer(mut timer: ResMut<GameTimer>) {
    timer.0 = 0.0;
}

fn update_timer(time: Res<Time>, mut timer: ResMut<GameTimer>) {
    timer.0 += time.delta_seconds();
}

fn obstacle_spawner(
    timer: Res<GameTimer>,
    mut last_spawned_obstacle: Local<f32>,
    player: Query<&Transform, With<Player>>,
    mut send_events: EventWriter<SpawnObstacleEvent>,
) {
    let Ok(player_transform) = player.get_single() else {
        return;
    };

    if *last_spawned_obstacle > timer.0 {
        *last_spawned_obstacle = timer.0;
        return;
    }

    if timer.0 - *last_spawned_obstacle < 2.5 {
        return;
    }

    let space = 200.0;
    let first_length = fastrand::f32() * (PROJECTION_SIZE.y - (space + 10.0));

    send_events.send(SpawnObstacleEvent {
        length: first_length,
        origin: crate::obstacle::ObstacleOrigin::Top,
        x_position: player_transform.translation.x + PROJECTION_SIZE.x + 20.0,
    });

    send_events.send(SpawnObstacleEvent {
        length: (PROJECTION_SIZE.y - first_length) - space,
        origin: crate::obstacle::ObstacleOrigin::Bottom,
        x_position: player_transform.translation.x + PROJECTION_SIZE.x + 20.0,
    });

    *last_spawned_obstacle = timer.0;
}
