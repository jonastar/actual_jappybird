use bevy::prelude::*;

use crate::GameState;

pub struct UiPlugin;

// Velocity in units per second
#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (button_hover_effects, button_actions));
    }
}

#[derive(Component)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

fn button_hover_effects(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonColors),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
            Interaction::Pressed => {}
        }
    }
}

#[derive(Component)]
pub struct ButtonChangeState(pub GameState);

#[derive(Component)]
pub struct ButtonOpenLink(pub &'static str);

fn button_actions(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            Option<&ButtonChangeState>,
            Option<&ButtonOpenLink>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, change_state, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {}
            Interaction::None => {}
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open link {error:?}");
                    }
                }
            }
        }
    }
}
