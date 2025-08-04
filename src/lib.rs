#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod loading;
mod startup_menu;
mod drawing_menu;
mod player;
mod flock;
mod shader;
mod animated_shader;

use crate::animated_shader::AnimatedShaderPlugin;
use crate::drawing_menu::DrawingMenuPlugin;
use crate::shader::ShaderPlugin;
use crate::{actions::ActionsPlugin, flock::FlockPlugin};
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::startup_menu::StartupMenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

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
    Drawing,
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
            AnimatedShaderPlugin,
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
