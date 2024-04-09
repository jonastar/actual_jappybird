use bevy::prelude::*;

use crate::{cleanup_entities_with, game_session::GameTimer, GameState};

pub struct HudPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_hud)
            .add_systems(
                OnExit(GameState::Playing),
                cleanup_entities_with::<HudMarker>,
            )
            .add_systems(Update, update_score_text);
    }
}

#[derive(Component)]
struct HudMarker;

#[derive(Component)]
struct ScoreText;

fn spawn_hud(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    top: Val::Px(5.),
                    width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            HudMarker,
        ))
        .with_children(|children| {
            children
                .spawn(TextBundle::from_section(
                    "0",
                    TextStyle {
                        font_size: 15.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ))
                .insert(ScoreText);
        });
}

fn update_score_text(mut score_text: Query<&mut Text, With<ScoreText>>, timer: Res<GameTimer>) {
    for mut text in &mut score_text {
        if !text.sections.is_empty() {
            text.sections[0].value = format!("{}", timer.get_score());
        }
    }
}
