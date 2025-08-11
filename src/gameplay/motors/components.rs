use bevy::prelude::*;

#[derive(Component)]
pub struct MotorsContainer;

#[derive(Component, Debug, Clone)]
pub struct Background;

#[derive(Component, Debug, Clone)]
pub struct Motor {
    pub freq: f64,    
}
#[derive(Component, Debug, Clone)]
pub struct MotorButton {
    pub freq: f64,
}

