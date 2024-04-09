#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod collision;
mod game_over;
mod game_session;
mod gravity;
mod hud;
mod loading;
mod menu;
mod obstacle;
mod player;
mod ui;
mod velocity;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use collision::CollisionPlugin;
use game_over::GameOverPlugin;
use game_session::GameSessionPlugin;
use gravity::GravityPlugin;
use hud::HudPlugin;
use obstacle::ObstaclePlugin;
use ui::UiPlugin;
use velocity::VelocityPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    GameOver,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub const PROJECTION_SIZE: Vec2 = Vec2::new(500.0, 500.0);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            ActionsPlugin,
            InternalAudioPlugin,
            PlayerPlugin,
            VelocityPlugin,
            GravityPlugin,
            ObstaclePlugin,
            GameSessionPlugin,
            CollisionPlugin,
            UiPlugin,
            GameOverPlugin,
            HudPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}

pub fn cleanup_entities_with<T: Component>(mut commands: Commands, menu: Query<Entity, With<T>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
