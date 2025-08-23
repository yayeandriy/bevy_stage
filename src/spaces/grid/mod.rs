use bevy::prelude::*;
use crate::plugins::line_grid::LineGridPlugin;
use crate::GameState;

pub struct GridSpacePlugin;

impl Plugin for GridSpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LineGridPlugin);
    }
}
