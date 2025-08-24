mod events;
mod observers;
pub mod components;

use bevy::prelude::*;
use crate::{tools::tile_map_grid::{components::{GridCell, SelectedCell, MainCell, Selector}, events::CellResized, observers::cell_resized_observer}, GameState};
use bevy::ecs::system::ParamSet;
use std::fmt::Debug;


const CELL_SIZE: f32 = 20.0;
const GAP_SIZE: f32 = 0.0;
const N: usize = 20;

#[derive(Component)]
struct TileMapGridEntity;

pub struct TileMapGridPlugin;

impl Plugin for TileMapGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::GridAndMotors), 
                startup
            )
            .add_systems(
                OnExit(GameState::GridAndMotors),
                cleanup_tile_map_grid
            )            
            .add_observer(cell_resized_observer);
    }
}




fn startup(mut commands: Commands) {
    info!("Starting TileMapGrid");
    
    // Add background
    commands.spawn((
        Sprite::from_color(Color::linear_rgb(0.1, 0.2, 0.3), Vec2::new(800.0, 600.0)),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        TileMapGridEntity,
    ));
   
    let grid = Vec2::new(N as f32, N as f32); // 2x2 grid
    let grid_vec = Vec2::new(
        CELL_SIZE + GAP_SIZE, 
        CELL_SIZE + GAP_SIZE
    );
    let sprite_size = Vec2::new(CELL_SIZE, CELL_SIZE);
    (0..grid.x as usize).for_each(|i|
        (0..grid.y as usize).for_each(|j| {
            let pos = Vec2::new(
                i as f32 , 
                j as f32 
            );
            let pos = pos * grid_vec;
            spawn_cell(&mut commands, pos, sprite_size, i, j);
        }));
}

fn spawn_cell(
    commands: &mut Commands,
    position: Vec2,
    size: Vec2,
    col: usize,
    row: usize,
) {
    commands.spawn((
        Sprite::from_color(Color::WHITE, size),
        Transform::from_translation(position.extend(0.0)),
        Pickable::default(),
        GridCell {
            row,
            col,
        },
        MainCell,
    ))
    .observe(resize_on_drag())
    .observe(toggle_color_on::<Pointer<Click>>());
}

fn toggle_color_on<E: Debug + Clone + Reflect>() -> impl Fn(Trigger<E>, Commands, 
    ParamSet<
        (
        Query<(&mut Sprite, &Transform, &GridCell, Option<&SelectedCell>), With<MainCell>>, 
        Query<&mut Sprite, With<MainCell>>, Query<Entity, With<MainCell>>)>, 
        Query<Entity, With<Selector>>
    ) {
    move |ev, mut commands, mut param_set, selector_entities| {
        log::info!("Cell clicked");
        let clicked_entity = ev.target();
        
        // First, get the clicked cell data
        let (transform, grid_cell, is_selected) = {
            let mut main_sprites = param_set.p0();
            let Ok((_sprite, transform, grid_cell, selected_cell)) = main_sprites.get_mut(clicked_entity) else {
                return;
            };
            (*transform, grid_cell.clone(), selected_cell.is_some())
        };
        
        // Remove any existing selector entities (blue borders)
        for entity in selector_entities.iter() {
            commands.entity(entity).despawn();
        }
        
        // Remove SelectedCell component from all main cells
        {
            let main_cell_entities = param_set.p2();
            for entity in main_cell_entities.iter() {
                commands.entity(entity).remove::<SelectedCell>();
            }
        }
        
        // Reset all main cells to white (unselected state)
        // {
        //     let mut all_main_cells = param_set.p1();
        //     for mut cell_sprite in all_main_cells.iter_mut() {
        //         cell_sprite.color = Color::WHITE;
        //     }
        // }
        
        // If the clicked cell was not selected, select it
        if !is_selected {
            // Set the clicked cell to black and add SelectedCell component
            {
                let mut main_sprites = param_set.p0();
                if let Ok((mut sprite, _, _, _)) = main_sprites.get_mut(clicked_entity) {
                    sprite.color = Color::BLACK;
                }
            }
            commands.entity(clicked_entity).insert(SelectedCell);
            
            // Spawn a blue selector sprite underneath the selected cell
            let cell_size = Vec2::new(CELL_SIZE, CELL_SIZE);
            let smaller_size = cell_size * 0.8; // 20% larger for border effect
            
            commands.spawn((
                Sprite::from_color(Color::hsl(240.0, 0.8, 0.6), smaller_size),
                Transform::from_translation(transform.translation - Vec3::new(0.0, 0.0, -0.1)), // Slightly before
                Selector,
                GridCell {
                    row: grid_cell.row,
                    col: grid_cell.col,
                },
            ));
        }
        // If the clicked cell was selected, it gets deselected (already done above)
    }
}
fn resize_on_drag() -> impl Fn(Trigger<Pointer<Drag>>, Query<(&mut Transform, &GridCell, &Sprite)>, Commands) {
    move |mut ev, mut query, mut commands| {
        let Ok((mut transform, grid_cell, sprite)) = query.get_mut(ev.target()) else {
            return;
        };
        
        ev.propagate(false);
        let drag_event = ev.event();
        let delta = drag_event.delta;
        let sprite_size = sprite.custom_size.unwrap_or(Vec2::new(100.0, 100.0));
        let factor = Vec2::new(1.0, -1.0);
        let size_change = delta * factor;
        let current_scale_2d = Vec2::new(transform.scale.x, transform.scale.y);
        let new_scale = (current_scale_2d * sprite_size + size_change) / sprite_size;
        transform.scale = new_scale.clamp(Vec2::splat(0.1), Vec2::splat(15.0)).extend(transform.scale.z);
        
        commands.trigger(CellResized {
            width: new_scale.x,
            height: new_scale.y,
            row: grid_cell.row,
            col: grid_cell.col,
        });
     
    }
}

fn cleanup_tile_map_grid(
    mut commands: Commands,
    tile_map_grid_entities: Query<Entity, With<TileMapGridEntity>>,
    cameras: Query<Entity, With<Camera2d>>,
) {
    // Despawn all TileMapGrid entities
    for entity in tile_map_grid_entities.iter() {
        commands.entity(entity).despawn();
    }
    
    // Despawn cameras
    for entity in cameras.iter() {
        commands.entity(entity).despawn();
    }
}
