use bevy::prelude::*;
use crate::{GameState, spaces::grid_and_motors::events::BackButtonPressed};

pub fn back_button_pressed_observer(
    trigger: Trigger<BackButtonPressed>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    log::info!("Back button pressed event received");
    next_state.set(GameState::Startup);
}
