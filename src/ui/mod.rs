pub mod assets;
pub mod components;
pub mod font_utils;
pub mod settings;
pub mod startup_menu;

use bevy::prelude::*;
use crate::ui::components::{BackButton, BackButtonColors};
use crate::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, back_button_system);
    }
}

fn back_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &BackButtonColors,
        ),
        (Changed<Interaction>, With<BackButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, button_colors) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = button_colors.pressed.into();
                // Navigate back to startup menu
                next_state.set(GameState::Startup);
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

pub use startup_menu::StartupMenuPlugin;
pub use assets::UiAssetsPlugin;
