use crate::plugins::motors::systems::{motors_update, handle_back_button, cleanup_motors};
use crate::plugins::motors::systems::startup as motors_startup;
use crate::GameState;
use bevy::prelude::*;

pub struct MotorsPlugin;

impl Plugin for MotorsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::GridAndMotors), 
                motors_startup,
            )
            .add_systems(
                Update,
                (motors_update, handle_back_button).run_if(in_state(GameState::GridAndMotors))
            )
            .add_systems(
                OnExit(GameState::GridAndMotors),
                cleanup_motors
            );
    }
}
