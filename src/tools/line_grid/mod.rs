mod events;
mod observers;
pub mod components;

use bevy::prelude::*;
use crate::{tools::line_grid::{components::{GridCell, SelectedCell, MainCell, Selector}, events::CellResized, observers::cell_resized_observer}, GameState};
use bevy::ecs::system::ParamSet;
use std::fmt::Debug;

const CELL_SIZE: f32 = 20.0;
const GAP_SIZE: f32 = 0.0;
const N: usize = 20;

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
                OnExit(GameState::Grid),
                cleanup_line_grid
            )            
            .add_observer(cell_resized_observer);
    }
}

fn setup_line_grid(mut commands: Commands) {
    info!("Starting Line Grid");
    
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
}

fn startup(commands: Commands) {
    setup_line_grid(commands);
}

fn cleanup_line_grid(
    mut commands: Commands,
    entities: Query<Entity, With<LineGridEntity>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}
