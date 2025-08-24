mod events;
mod observers;

use bevy::prelude::*;
use crate::{GameState, spaces::flexer::{events::BackButtonPressed, observers::back_button_pressed_observer}, plugins::FlexGridPlugin, ui::components::spawn_back_button, systems::loading::FontAssets};

#[derive(Component)]
struct FlexerSpaceEntity;

pub struct FlexerSpacePlugin;

impl Plugin for FlexerSpacePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FlexGridPlugin)
            .add_event::<BackButtonPressed>()
            .add_systems(
                OnEnter(GameState::Flexer), 
                startup
            )
            .add_systems(
                OnExit(GameState::Flexer),
                cleanup_flexer_space
            )
            .add_observer(back_button_pressed_observer);
    }
}

fn setup_flexer_space(mut commands: Commands, asset_server: Res<AssetServer>, fonts: Res<FontAssets>) {
    info!("Starting Flexer Space");
    commands.spawn((Camera2d, Msaa::Off, FlexerSpaceEntity));
    
    // Spawn UI back button
    spawn_back_button(&mut commands, &asset_server, &fonts);
}

fn startup(commands: Commands, asset_server: Res<AssetServer>, fonts: Res<FontAssets>) {
    setup_flexer_space(commands, asset_server, fonts);
}

fn cleanup_flexer_space(
    mut commands: Commands,
    entities: Query<Entity, With<FlexerSpaceEntity>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}
