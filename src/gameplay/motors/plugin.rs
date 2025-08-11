use crate::gameplay::motors::systems::motors_update;
use crate::gameplay::motors::systems::startup as motors_startup;
use crate::GameState;
use bevy::prelude::*;

pub struct MotorsPlugin;

impl Plugin for MotorsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::Motors), 
                motors_startup
            )
            .add_systems(
                Update, 
                motors_update.run_if(in_state(GameState::Motors))
            );
    }
}
