use bevy::prelude::*;

use crate::{
    cleanup_entities_with,
    game_session::GameTimer,
    ui::{ButtonChangeState, ButtonColors},
    GameState,
};

pub struct GameOverPlugin;

// Velocity in units per second
#[derive(Component)]
pub struct GameOverEntityMarker;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_game_over)
            .add_systems(
                OnExit(GameState::GameOver),
                cleanup_entities_with::<GameOverEntityMarker>,
            );
    }
}

fn spawn_game_over(mut commands: Commands, timer: Res<GameTimer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            GameOverEntityMarker,
        ))
        .with_children(|children| {
            children.spawn(TextBundle::from_section(
                "Game Over",
                TextStyle {
                    font_size: 15.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));

            children.spawn(TextBundle::from_section(
                format!("{}", timer.get_score()),
                TextStyle {
                    font_size: 15.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));

            let button_colors = ButtonColors::default();
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    },
                    button_colors,
                    ButtonChangeState(GameState::Playing),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}
