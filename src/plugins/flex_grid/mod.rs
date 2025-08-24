pub mod events;
pub mod observers;

use bevy::prelude::*;
use crate::{GameState, plugins::flex_grid::{events::BackButtonPressed, observers::back_button_pressed_observer}};

pub struct FlexGridPlugin;

impl Plugin for FlexGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BackButtonPressed>()
            .add_observer(back_button_pressed_observer);
    }
}
