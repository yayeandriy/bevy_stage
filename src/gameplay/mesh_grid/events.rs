use bevy::prelude::*;

#[derive(Event)]
pub struct ToggleCell {
    pub row: usize,
    pub col: usize,
}

#[derive(Event)]
pub struct ResizeCell {
    pub row: usize,
    pub col: usize,
    pub delta_x: f32, // Change in width
    pub delta_y: f32, // Change in height
}
