mod events;
mod observers;
pub mod components;

use crate::{plugins::line_grid::{components::{GridCell, SelectedCell, MainCell, Selector}, events::CellResized, observers::cell_resized_observer}, GameState};
use bevy::prelude::*;
use bevy::ecs::system::ParamSet;
use std::fmt::Debug;


const CELL_SIZE: f32 = 20.0;
const GAP_SIZE: f32 = 0.0;
const N: usize = 20;

#[derive(Component)]
struct BackButton;

#[derive(Component)]
struct LineGridEntity;

pub struct LineGridPlugin;

impl Plugin for LineGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::Grid), 
                startup
            )
            .add_systems(
                Update,
                handle_back_button.run_if(in_state(GameState::Grid))
            )
            .add_systems(
                OnExit(GameState::Grid),
                cleanup_line_grid
            )            
            .add_observer(cell_resized_observer);
    }
}

fn setup_line_grid(mut commands: Commands) {
    info!("Starting Line Grid");
    commands.spawn((Camera2d, Msaa::Off, LineGridEntity));
    
    let main_cell_transform = Transform::from_xyz(0.0, 0.0, 0.0);
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::new(CELL_SIZE, CELL_SIZE)),
        main_cell_transform,
        MainCell,
        LineGridEntity,
    ));

    for i in 0..N {
        for j in 0..N {
            let x = (i as f32 - N as f32 / 2.0) * (CELL_SIZE + GAP_SIZE);
            let y = (j as f32 - N as f32 / 2.0) * (CELL_SIZE + GAP_SIZE);
            let transform = Transform::from_xyz(x, y, 0.0);
            commands.spawn((
                Sprite::from_color(Color::BLACK, Vec2::new(CELL_SIZE, CELL_SIZE)),
                transform,
                GridCell { row: i, col: j },
                LineGridEntity,
            ));
        }
    }
    
    let selector_transform = Transform::from_xyz(-400.0, 300.0, 1.0);
    commands.spawn((
        Sprite::from_color(Color::linear_rgb(1.0, 0.0, 0.0), Vec2::new(CELL_SIZE, CELL_SIZE)),
        selector_transform,
        Selector,
        LineGridEntity,
    ));

    // Back button
    commands.spawn((
        Button,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            width: Val::Px(100.0),
            height: Val::Px(50.0),
            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(Color::BLACK),
        BackgroundColor(Color::WHITE),
        BackButton,
        LineGridEntity,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Back"),
            TextColor(Color::BLACK),
            TextFont {
                font_size: 16.0,
                ..default()
            },
        ));
    });
}

fn startup(
    mut commands: Commands,
    mut cell_query: Query<(Entity, &GridCell, &mut Transform), Without<MainCell>>,
    mut main_cell_query: Query<&mut Transform, (With<MainCell>, Without<GridCell>)>,
    mut selector_query: Query<&mut Transform, (With<Selector>, Without<GridCell>, Without<MainCell>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    setup_line_grid(commands);
}

fn handle_back_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Startup);
            }
            _ => {}
        }
    }
}

fn cleanup_line_grid(
    mut commands: Commands,
    entities: Query<Entity, With<LineGridEntity>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
