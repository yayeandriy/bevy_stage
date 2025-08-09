mod events;
mod observers;
mod components;

use crate::{gameplay::tile_map_grid::{components::GridCell, events::CellResized, observers::cell_resized_observer}, GameState};
use bevy::prelude::*;
use std::fmt::Debug;


const CELL_SIZE: f32 = 20.0;
const GAP_SIZE: f32 = 0.0;
const N: usize = 20;
pub struct TileMapGridPlugin;

impl Plugin for TileMapGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::TileMapGrid), 
                startup
            )            
            .add_observer(cell_resized_observer);
    }
}




fn startup(
    mut commands: Commands,   
    windows: Query<&Window>,
) {
    if let Ok(window) = windows.single() {
        let window_size = Vec2::new(window.width(), window.height());
        commands.spawn((
            Sprite::from_color(Color::BLACK, Vec2::new(window_size.x, window_size.y)),
            Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        ));
    }
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
    ))
    .observe(resize_on_drag())
    .observe(toggle_color_on::<Pointer<Click>>());
}

fn toggle_color_on<E: Debug + Clone + Reflect>() -> impl Fn(Trigger<E>, Query<&mut Sprite>) {
    move |ev, mut sprites| {
        log::info!("MOUSE MOVED");
        let Ok(mut sprite) = sprites.get_mut(ev.target()) else {
            return;
        };
        if sprite.color == Color::WHITE {
            sprite.color = Color::BLACK;
        } else {
            sprite.color = Color::WHITE;
        }
     
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
