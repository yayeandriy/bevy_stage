mod events;
mod observers;

use bevy::prelude::*;
use crate::{GameState, spaces::flexer::{events::BackButtonPressed, observers::back_button_pressed_observer}, plugins::FlexGridPlugin, ui::assets::UiAssets};

#[derive(Component)]
pub struct BackButton;

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

fn setup_flexer_space(mut commands: Commands, windows: Query<&Window>, ui_assets: Res<UiAssets>) {
    info!("Starting Flexer Space");
    commands.spawn((Camera2d, Msaa::Off, FlexerSpaceEntity));
    
    if let Ok(window) = windows.single() {
        let window_size = Vec2::new(window.width(), window.height());
        
        // Back button - using proper back button image with 40x40 size and 10 margin
        let button_size = 40.0;
        let margin = 10.0;
        
        // Calculate position in top-left corner (fixed world coordinates)
        let button_x = -window_size.x / 2.0 + margin + button_size / 2.0;
        let button_y = window_size.y / 2.0 - margin + button_size / 2.0;
        
        commands.spawn((
            Sprite {
                custom_size: Some(Vec2::new(button_size, button_size)),
                color: Color::linear_rgb(0.15, 0.15, 0.15), // Dark gray instead of white
                ..default()
            },
            Transform::from_xyz(button_x, button_y, 1.0),
            Pickable::default(),
            BackButton,
            FlexerSpaceEntity,
        ))
        .observe(back_button_click_handler());
    }
}



fn back_button_click_handler() -> impl Fn(Trigger<Pointer<Click>>, Commands) {
    move |_ev, mut commands| {
        log::info!("Back button clicked");
        commands.trigger(BackButtonPressed);
    }
}

fn startup(commands: Commands, windows: Query<&Window>, ui_assets: Res<UiAssets>) {
    setup_flexer_space(commands, windows, ui_assets);
}

fn cleanup_flexer_space(
    mut commands: Commands,
    entities: Query<Entity, With<FlexerSpaceEntity>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}
