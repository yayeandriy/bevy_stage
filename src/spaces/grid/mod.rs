mod events;
mod observers;

use bevy::prelude::*;
use crate::{GameState, spaces::grid::{events::BackButtonPressed, observers::back_button_pressed_observer}, plugins::line_grid::LineGridPlugin};

#[derive(Component)]
struct BackButton;

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
                Update,
                update_back_button_position.run_if(in_state(GameState::Grid))
            )
            .add_systems(
                OnExit(GameState::Grid),
                cleanup_grid_space
            )
            .add_observer(back_button_pressed_observer);
    }
}

fn setup_grid_space(mut commands: Commands, windows: Query<&Window>) {
    info!("Starting Grid Space");
    commands.spawn((Camera2d, Msaa::Off, GridSpaceEntity));
    
    if let Ok(window) = windows.single() {
        let window_size = Vec2::new(window.width(), window.height());
        
        // Back button - white square positioned in top-left corner
        let button_size = 40.0;
        let margin = 20.0;
        
        // Calculate position in top-left corner
        let button_x = -window_size.x / 2.0 + margin + button_size / 2.0;
        let button_y = window_size.y / 2.0 - margin - button_size / 2.0;
        
        commands.spawn((
            Sprite::from_color(Color::WHITE, Vec2::new(button_size, button_size)),
            Transform::from_xyz(button_x, button_y, 1.0),
            Pickable::default(),
            BackButton,
            GridSpaceEntity,
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
    setup_grid_space(commands, windows);
}

fn cleanup_grid_space(
    mut commands: Commands,
    entities: Query<Entity, With<GridSpaceEntity>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}

fn update_back_button_position(
    mut back_button_query: Query<&mut Transform, With<BackButton>>,
    windows: Query<&Window, Changed<Window>>,
) {
    if let Ok(window) = windows.single() {
        let window_size = Vec2::new(window.width(), window.height());
        let button_size = 40.0;
        let margin = 20.0;
        
        // Calculate new position in top-left corner
        let button_x = -window_size.x / 2.0 + margin + button_size / 2.0;
        let button_y = window_size.y / 2.0 - margin - button_size / 2.0;
        
        for mut transform in back_button_query.iter_mut() {
            transform.translation.x = button_x;
            transform.translation.y = button_y;
        }
    }
}
