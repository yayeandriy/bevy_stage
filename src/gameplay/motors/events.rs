use bevy::prelude::*;

#[derive(Event)]
pub struct ToggleMotor {
    pub row: usize,
    pub col: usize,
}

