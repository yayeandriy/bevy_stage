mod events;
mod observers;

use bevy::prelude::*;
use crate::{GameState, spaces::grid::{events::BackButtonPressed, observers::back_button_pressed_observer}, plugins::line_grid::LineGridPlugin, ui::components::spawn_back_button, systems::loading::FontAssets};

#[derive(Component)]
struct GridSpaceEntity;

pub struct GridSpacePlugin;

impl Plugin for GridSpacePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(LineGridPlugin)
            .add_event::<BackButtonPressed>()
            .add_systems(
                OnEnter(GameState::Grid), 
                startup
            )
            .add_systems(
                OnExit(GameState::Grid),
                cleanup_grid_space
            )
            .add_observer(back_button_pressed_observer);
    }
}

fn setup_grid_space(mut commands: Commands, asset_server: Res<AssetServer>, fonts: Res<FontAssets>) {
    info!("Starting Grid Space");
    commands.spawn((Camera2d, Msaa::Off, GridSpaceEntity));
    
    // Spawn UI back button
    spawn_back_button(&mut commands, &asset_server, &fonts);
}

fn startup(commands: Commands, asset_server: Res<AssetServer>, fonts: Res<FontAssets>) {
    setup_grid_space(commands, asset_server, fonts);
}

fn cleanup_grid_space(
    mut commands: Commands,
    entities: Query<Entity, With<GridSpaceEntity>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}
