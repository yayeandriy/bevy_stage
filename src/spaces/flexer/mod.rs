mod events;
mod observers;

use bevy::prelude::*;
use crate::{GameState, spaces::flexer::{events::BackButtonPressed, observers::back_button_pressed_observer}, plugins::FlexGridPlugin};

#[derive(Component)]
struct BackButton;

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

fn setup_flexer_space(mut commands: Commands, windows: Query<&Window>) {
    info!("Starting Flexer Space");
    commands.spawn((Camera2d, Msaa::Off, FlexerSpaceEntity));
    
    if let Ok(window) = windows.single() {
        let window_size = Vec2::new(window.width(), window.height());
        
        // Back button - white triangle pointing left, positioned in top-left corner
        let button_size = 40.0; // Size of the triangle
        let margin = 20.0; // Margin from screen edges
        
        // Calculate position in top-left corner
        let button_x = -window_size.x / 2.0 + margin + button_size / 2.0;
        let button_y = window_size.y / 2.0 - margin - button_size / 2.0;
        
        // Create a simple white square as back button
        commands.spawn((
            Sprite::from_color(Color::WHITE, Vec2::new(button_size, button_size)),
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

fn startup(commands: Commands, windows: Query<&Window>) {
    setup_flexer_space(commands, windows);
}

fn cleanup_flexer_space(
    mut commands: Commands,
    entities: Query<Entity, With<FlexerSpaceEntity>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}
