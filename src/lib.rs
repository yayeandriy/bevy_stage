#![allow(clippy::type_complexity)]

mod ui;
mod plugins;
mod spaces;
mod systems;

use crate::spaces::{GridSpacePlugin, GridAndMotorsSpacePlugin, FlexerSpacePlugin};
use crate::ui::{StartupMenuPlugin, UiAssetsPlugin}; // DrawingMenuPlugin removed due to camera conflicts
use crate::systems::LoadingPlugin;

use bevy::app::App;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    Grid,
    GridAndMotors,
    Flexer,
    // Here the menu is drawn and waiting for player interaction
    Startup,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            LoadingPlugin,
            UiAssetsPlugin,
            StartupMenuPlugin,
            // DrawingMenuPlugin, // Disabled - causes camera conflicts with Motors state
            TilemapPlugin,
            GridSpacePlugin,
            GridAndMotorsSpacePlugin,
            FlexerSpacePlugin
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                // FrameTimeDiagnosticsPlugin::default(),
                // LogDiagnosticsPlugin::default(),
            ));
        }
    }
}
