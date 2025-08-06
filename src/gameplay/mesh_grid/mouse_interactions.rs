use super::{components::GridCell, events::*, resources::*};
use bevy::prelude::*;
use bevy::render::mesh::MeshAabb;

/// Helper struct for mouse interaction calculations
pub struct MouseInteraction {
    pub cursor_pos: Vec2,
    pub is_within_bounds: bool,
    pub cell_bounds: (f32, f32, f32, f32), // x_min, x_max, y_min, y_max
}

impl MouseInteraction {
    pub fn new(cursor_pos: Vec2, grid_pos: Vec2, width: f32, height: f32) -> Self {
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        let bounds = (
            grid_pos.x - half_width,
            grid_pos.x + half_width,
            grid_pos.y - half_height,
            grid_pos.y + half_height,
        );
        
        let is_within_bounds = (bounds.0..=bounds.1).contains(&cursor_pos.x) &&
                              (bounds.2..=bounds.3).contains(&cursor_pos.y);
        
        Self {
            cursor_pos,
            is_within_bounds,
            cell_bounds: bounds,
        }
    }
}

/// Get cursor position in world coordinates
pub fn get_cursor_world_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
}

/// Get cell dimensions from mesh
pub fn get_cell_dimensions(mesh2d: &Mesh2d, meshes: &Assets<Mesh>) -> (f32, f32) {
    if let Some(mesh) = meshes.get(&mesh2d.0) {
        if let Some(aabb) = mesh.compute_aabb() {
            (aabb.half_extents.x * 2.0, aabb.half_extents.y * 2.0)
        } else {
            (20.0, 20.0) // fallback dimensions
        }
    } else {
        (20.0, 20.0) // fallback dimensions
    }
}

/// Find cell at a specific position
pub fn find_cell_at_position(
    cursor_pos: Vec2,
    grid_query: &Query<(Entity, &Transform, &GridCell, &Mesh2d), With<GridCell>>,
    meshes: &Assets<Mesh>,
) -> Option<(usize, usize)> {
    for (_entity, transform, grid_cell, mesh2d) in grid_query.iter() {
        let grid_pos = transform.translation.truncate();
        let (width, height) = get_cell_dimensions(mesh2d, meshes);
        let interaction = MouseInteraction::new(cursor_pos, grid_pos, width, height);
        
        if interaction.is_within_bounds {
            return Some((grid_cell.row, grid_cell.col));
        }
    }
    None
}

/// Check if cursor is within cell bounds using simple fixed bounds
pub fn is_cursor_in_cell_bounds(cursor_pos: Vec2, grid_pos: Vec2, bound_size: f32) -> bool {
    (grid_pos.x - bound_size..=grid_pos.x + bound_size).contains(&cursor_pos.x) &&
    (grid_pos.y - bound_size..=grid_pos.y + bound_size).contains(&cursor_pos.y)
}

/// Handle drag start logic
pub fn handle_drag_start(
    mouse_button_input: &ButtonInput<MouseButton>,
    mouse_drag_state: &mut DragState,
    cursor_pos: Vec2,
) -> bool {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        mouse_drag_state.is_dragging = true;
        mouse_drag_state.start_position = cursor_pos;
        log::info!("Mouse drag started at {:?}", cursor_pos);
        return true;
    }
    false
}

/// Handle drag end logic
pub fn handle_drag_end(
    mouse_button_input: &ButtonInput<MouseButton>,
    mouse_drag_state: &mut DragState,
) -> bool {
    if mouse_button_input.just_released(MouseButton::Left) {
        mouse_drag_state.is_dragging = false;
        mouse_drag_state.drag_type = None;
        log::info!("Mouse drag ended");
        return true;
    }
    false
}

/// Check if we're in resize drag mode (shift key held)
pub fn is_resize_drag(keyboard_input: &ButtonInput<KeyCode>) -> bool {
    keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.just_pressed(KeyCode::ShiftLeft)
}

/// Handle resize drag behavior
pub fn handle_resize_drag(
    commands: &mut Commands,
    mouse_drag_state: &DragState,
    cursor_pos: Vec2,
    grid_query: &Query<(Entity, &Transform, &mut GridCell, &Mesh2d), With<GridCell>>,
    meshes: &Assets<Mesh>,
) {
    let delta_pos = mouse_drag_state.start_position - cursor_pos;
    log::info!("Mouse drag with Shift key held down - delta: {:?}", delta_pos);
    
    // Find the cell that was clicked initially
    let mut col = 0;
    let mut row = 0;
    
    for (_entity, transform, grid_cell, mesh2d) in grid_query.iter() {
        let grid_pos = transform.translation.truncate();
        let width = if let Some(mesh) = meshes.get(&mesh2d.0) {
            if let Some(aabb) = mesh.compute_aabb() {
                aabb.half_extents.x
            } else {
                10.0 // fallback width
            }
        } else {
            10.0 // fallback width
        };
        
        // Check if the start position is within this cell
        if (grid_pos.x - width..=grid_pos.x + width).contains(&mouse_drag_state.start_position.x) &&
           (grid_pos.y - 10.0..=grid_pos.y + 10.0).contains(&mouse_drag_state.start_position.y) {
            col = grid_cell.col;
            row = grid_cell.row;
            break;
        }
    }
    
    if col != 0 || row != 0 {
        log::info!("Dragging cell at row: {}, col: {}", row, col);
        let delta_x = delta_pos.x;
        let delta_y = delta_pos.y;
        
        commands.trigger(ResizeCell {
            row,
            col,
            delta_x,
            delta_y,
        });
    }
}

/// Handle toggle drag behavior (normal dragging)
pub fn handle_toggle_drag(
    commands: &mut Commands,
    cursor_pos: Vec2,
    grid_query: &mut Query<(Entity, &Transform, &mut GridCell, &Mesh2d), With<GridCell>>,
) {
    grid_query.iter_mut().for_each(|(_entity, transform, mut grid_cell, _mesh2d)| {
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
