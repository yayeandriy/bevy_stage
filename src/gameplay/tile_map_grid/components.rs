use bevy::prelude::*;

#[derive(Component)]
pub struct MeshGridContainer;

#[derive(Component, Debug, Clone)]
pub struct GridCell {
    pub row: usize,
    pub col: usize,
}

impl Default for GridCell {
    fn default() -> Self {
        Self {
            row: 0,
            col: 0,
        }
    }
}
