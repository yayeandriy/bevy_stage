use bevy::prelude::*;

#[derive(Event)]
pub struct ToggleCell {
    pub row: usize,
    pub col: usize,
}

#[derive(Event)]
pub struct CellResized {
    // pub row: usize,
    // pub col: usize,
    pub width: f32, // Change in width
    pub height: f32, // Change in height
    pub col: usize,
    pub row: usize,
}
