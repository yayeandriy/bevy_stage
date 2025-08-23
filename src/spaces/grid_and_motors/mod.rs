use bevy::prelude::*;
use crate::plugins::{MotorsPlugin, TileMapGridPlugin};
use crate::GameState;

pub struct GridAndMotorsSpacePlugin;

impl Plugin for GridAndMotorsSpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TileMapGridPlugin,
            MotorsPlugin,
        ));
    }
}
