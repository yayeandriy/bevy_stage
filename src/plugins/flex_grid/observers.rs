use bevy::prelude::*;
use crate::{GameState, plugins::flex_grid::events::BackButtonPressed};

pub fn back_button_pressed_observer(
    trigger: Trigger<BackButtonPressed>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    log::info!("Back button pressed event received");
    next_state.set(GameState::Startup);
}
