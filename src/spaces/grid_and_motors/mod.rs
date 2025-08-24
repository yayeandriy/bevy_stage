mod events;
mod observers;

use bevy::prelude::*;
use crate::{GameState, spaces::grid_and_motors::{events::BackButtonPressed, observers::back_button_pressed_observer}, plugins::{MotorsPlugin, TileMapGridPlugin}, ui::components::spawn_back_button, systems::loading::FontAssets};

#[derive(Component)]
struct GridAndMotorsSpaceEntity;

pub struct GridAndMotorsSpacePlugin;

impl Plugin for GridAndMotorsSpacePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                TileMapGridPlugin,
                // MotorsPlugin,
            ))
            .add_event::<BackButtonPressed>()
            .add_systems(
                OnEnter(GameState::GridAndMotors), 
                startup
            )
            .add_systems(
                OnExit(GameState::GridAndMotors),
                cleanup_grid_and_motors_space
            )
            .add_observer(back_button_pressed_observer);
    }
}

fn setup_grid_and_motors_space(mut commands: Commands, asset_server: Res<AssetServer>, fonts: Res<FontAssets>) {
    info!("Starting Grid and Motors Space");
    commands.spawn((Camera2d, Msaa::Off, GridAndMotorsSpaceEntity));
    
    // Spawn UI back button
    spawn_back_button(&mut commands, &asset_server, &fonts);
}

fn startup(commands: Commands, asset_server: Res<AssetServer>, fonts: Res<FontAssets>) {
    setup_grid_and_motors_space(commands, asset_server, fonts);
}

fn cleanup_grid_and_motors_space(
    mut commands: Commands,
    entities: Query<Entity, With<GridAndMotorsSpaceEntity>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}
