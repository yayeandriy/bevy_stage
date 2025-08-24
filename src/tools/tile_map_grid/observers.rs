use bevy::prelude::*;

use crate::tools::tile_map_grid::{components::GridCell, events::CellResized};


pub fn cell_resized_observer(
    trigger: Trigger<CellResized>,
    mut commands: Commands,
    mut grid_query: Query<(Entity, &mut Transform, &GridCell, &Sprite)>,
) {
    let event = trigger.event();
    log::info!("Cell resized event received: col={}; row={};", event.col, event.row);
    log::info!("Cell resized: width = {}, height = {}", event.width, event.height);
    for (entity, mut transform, grid_cell, sprite) in grid_query.iter_mut() {
        
        // if grid_cell.col == event.col  {
        //     transform.scale = Vec3::new(event.width,  transform.scale.y, 1.0); // Skip cells that are not affected by this resize event
        // }
        // if grid_cell.row == event.row {
        //     transform.scale = Vec3::new(transform.scale.x, event.height, 1.0); // Skip cells that are not affected by this resize event
        // }
        // if grid_cell.col == event.col && grid_cell.row == event.row {
        //     continue; // Skip cells that are not affected by this resize event
        // }
        // let d_col = (grid_cell.col as f32 - event.col as f32).abs() + 1.0;
        // let d_row = (grid_cell.row as f32 - event.row as f32).abs() + 1.0;

        // let scale_x = transform.scale.x + (event.width - transform.scale.x)/(d_col*160.0);
        // transform.scale = Vec3::new(scale_x,  transform.scale.y, 1.0); // Skip cells that are not affected by this resize event
        // let scale_y = transform.scale.y/(d_row*4.0);
        // transform.scale = Vec3::new(scale_x, scale_y, 1.0);
        if grid_cell.col == event.col {
            // let scale_x = 1.0 / event.width as f32;
            transform.scale = Vec3::new(event.width, transform.scale.y, 1.0);
        }
        if grid_cell.col == event.col + 1 || grid_cell.col == event.col - 1 {
            let scale_x = 1.0 / event.width as f32;
            transform.scale = Vec3::new(scale_x, transform.scale.y, 1.0);
        }
        if grid_cell.col == event.col + 2 || grid_cell.col == event.col - 2 {
            let scale_x = 0.75 * event.width as f32;
            transform.scale = Vec3::new(scale_x, transform.scale.y, 1.0);
        }

        if grid_cell.row == event.row {
            // let scale_y = 1.0 / event.height as f32;
            transform.scale = Vec3::new(transform.scale.x, event.height, 1.0);
        }
        if grid_cell.row == event.row + 1 || grid_cell.row == event.row - 1 {
            let scale_y = 1.0 / event.height as f32;
            transform.scale = Vec3::new(transform.scale.x, scale_y, 1.0);
        }
        if grid_cell.row == event.row + 2 || grid_cell.row == event.row - 2 {
            let scale_y = 0.75 * event.height as f32;
            transform.scale = Vec3::new(transform.scale.x, scale_y, 1.0);
        }

    }
}
