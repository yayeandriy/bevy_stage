#![allow(clippy::type_complexity)]

mod ui;
mod gameplay;
mod shaders;
mod systems;

use crate::gameplay::ui_grid::UIGridPlugin;
use crate::ui::{StartupMenuPlugin, DrawingMenuPlugin};
use crate::gameplay::{ActionsPlugin, FlockPlugin, MeshGridPlugin, TileMapGridPlugin};
use crate::shaders::AnimatedShaderPlugin;
use crate::systems::{LoadingPlugin, InternalAudioPlugin};

use bevy::app::App;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_picking::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    MeshGrid,
    TileMapGrid,
    // Here the menu is drawn and waiting for player interaction
    Startup,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            LoadingPlugin,
            StartupMenuPlugin,
            DrawingMenuPlugin,
            ActionsPlugin,
            InternalAudioPlugin,
            // PlayerPlugin,
            FlockPlugin,
            // AnimatedShaderPlugin,
            // UIGridPlugin,
            MeshGridPlugin,
            TilemapPlugin,
            TileMapGridPlugin
            // ShaderPlugin
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
