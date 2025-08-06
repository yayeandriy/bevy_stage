use super::{components::*, events::*, mouse_interactions::*, resources::*};
use bevy::prelude::*;

/// Handle mouse click events
pub fn handle_mouse_click(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut grid_query: Query<(Entity, &Transform, &mut GridCell), With<GridCell>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let (camera, camera_transform) = *camera;
    
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let Some(cursor_pos) = get_cursor_world_position(window, camera, camera_transform) else {
            return;
        };
        
        if grid_query.is_empty() {
            log::warn!("No grid cells found! Cannot handle click.");
            return;
        }
        
        grid_query.iter_mut().for_each(|(_entity, transform, mut grid_cell)| {
            let grid_pos = transform.translation.truncate();
            
            if is_cursor_in_cell_bounds(cursor_pos, grid_pos, 10.0) {
                grid_cell.is_black = !grid_cell.is_black;
                commands.trigger(ToggleCell {
                    row: grid_cell.row,
                    col: grid_cell.col,
                });
            }
        });
    }
}

/// Handle mouse drag events
pub fn handle_mouse_drag(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_drag_state: ResMut<DragState>,
    windows: Query<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut grid_query: Query<(Entity, &Transform, &mut GridCell, &Mesh2d), With<GridCell>>,
    meshes: Res<Assets<Mesh>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let (camera, camera_transform) = *camera;
    let Some(cursor_pos) = get_cursor_world_position(window, camera, camera_transform) else {
        return;
    };

    // Handle drag start
    if handle_drag_start(&mouse_button_input, &mut mouse_drag_state, cursor_pos) {
        return;
    }
    
    // Handle drag end
    if handle_drag_end(&mouse_button_input, &mut mouse_drag_state) {
        return;
    }

    // Handle ongoing drag
    if mouse_drag_state.is_dragging {
        if is_resize_drag(&keyboard_input) {
            handle_resize_drag(
                &mut commands,
                &mouse_drag_state,
                cursor_pos,
                &grid_query,
                &meshes,
            );
        } else {
            handle_toggle_drag(
                &mut commands,
                cursor_pos,
                &mut grid_query,
            );
        }
    }
}
