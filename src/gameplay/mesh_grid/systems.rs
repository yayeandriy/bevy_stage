use super::{components::*, events::*, resources::*};
use bevy::prelude::*;

/// Setup the initial grid state
pub fn setup_grid(mut grid_state: ResMut<MeshGridState>) {
    // Create a grid with equal sizing
    grid_state.cols = (0..INIT_N_COLS).map(|_| 1.0).collect();
    grid_state.cols.push(2.0);
    grid_state.rows = (0..INIT_N_ROWS).map(|_| 1.0).collect();
}

/// Spawn the grid entities
pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid_state: Res<MeshGridState>,
    cells: Query<&GridCell>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    
    log::info!("Window dimensions: {}x{}", window.width(), window.height());
    
    // Calculate available space for each cell (window size minus gaps)
    let num_cols = grid_state.cols.len();
    let num_rows = grid_state.rows.len();
    let available_width = window.width() - GRID_GAP * (num_cols - 1) as f32;
    let available_height = window.height() - GRID_GAP * (num_rows - 1) as f32;
    
    // Calculate the total proportional units
    let total_col_units: f64 = grid_state.cols.iter().sum();
    let total_row_units: f64 = grid_state.rows.iter().sum();
    
    let actual_cols_widths: Vec<f64> = grid_state.cols.iter()
        .map(|&c| (c * available_width as f64 / total_col_units))
        .collect();
    let actual_rows_heights: Vec<f64> = grid_state.rows.iter()
        .map(|&r| (r * available_height as f64 / total_row_units))
        .collect();
    
    // Start from top-left corner of the window
    let start_x = -window.width() / 2.0;
    let start_y = window.height() / 2.0;
    let mut y_offset = start_y;
    
    for (row, &height) in actual_rows_heights.iter().enumerate() {
        y_offset -= height as f32 / 2.0; // Move to center of current row
        let mut x_offset = start_x;
        
        for (col, &width) in actual_cols_widths.iter().enumerate() {
            x_offset += width as f32 / 2.0; // Move to center of current column
            let shape = meshes.add(Rectangle::new(width as f32, height as f32));
            let color = Color::hsl(0.5, 0.8, 0.6);
            let cell = find_cell_at_col_and_row(&cells, col, row)
                .cloned()
                .unwrap_or(GridCell { row, col, is_black: false });
            
            commands.spawn((
                Mesh2d(shape),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(x_offset, y_offset, 0.0),
                cell,
            ));
            
            x_offset += width as f32 / 2.0 + GRID_GAP; // Move to end of current column plus gap
        }
        y_offset -= height as f32 / 2.0 + GRID_GAP; // Move to bottom of current row plus gap
    }
}

/// Observer for toggle cell events
pub fn toggle_cell_observer(
    trigger: Trigger<ToggleCell>,
    grid_query: Query<(Entity, &Transform, &mut MeshMaterial2d<ColorMaterial>, &GridCell), With<GridCell>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let event = trigger.event();
    toggle_cell(grid_query, commands, materials, event.row, event.col);
}

/// Observer for resize cell events
pub fn resize_cell_observer(
    trigger: Trigger<ResizeCell>,
    mut commands: Commands,
    mut grid_state: ResMut<MeshGridState>,
    grid_query: Query<(Entity, &Transform, &GridCell), With<GridCell>>,
) {
    let event = trigger.event();
    resize_cells(commands, grid_query, grid_state, event.row, event.col, event.delta_x, event.delta_y);
}

/// Find a cell at specific column and row
fn find_cell_at_col_and_row<'a>(
    cells: &'a Query<&GridCell>,
    col: usize,
    row: usize,
) -> Option<&'a GridCell> {
    cells.iter().find(|&cell| cell.col == col && cell.row == row)
}

/// Toggle a specific cell's color
fn toggle_cell(
    grid_query: Query<(Entity, &Transform, &mut MeshMaterial2d<ColorMaterial>, &GridCell), With<GridCell>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    row: usize,
    col: usize,
) {
    for (entity, _transform, _mesh_material, grid_cell) in grid_query.iter() {
        if grid_cell.row != row || grid_cell.col != col {
            continue; // Skip if this cell is not the one we want to toggle
        }
        
        let new_color = if grid_cell.is_black {
            Color::BLACK
        } else {
            Color::WHITE
        };
        commands.entity(entity).insert(MeshMaterial2d(materials.add(ColorMaterial::from(new_color))));
    }
}

/// Resize cells in a column and row
fn resize_cells(
    mut commands: Commands,
    mut grid_query: Query<(Entity, &Transform, &GridCell), With<GridCell>>,
    mut grid_state: ResMut<MeshGridState>,
    row: usize,
    col: usize,
    delta_x: f32,
    delta_y: f32,
) {
    // Calculate scale factors
    let scale_factor_x = 1.0 + (delta_x * 0.003);
    let scale_factor_y = 1.0 + (delta_y * 0.003);

    // Clamp scale factors to reasonable bounds
    let clamped_scale_x = scale_factor_x.clamp(0.1, 10.0);
    let clamped_scale_y = scale_factor_y.clamp(0.1, 10.0);
    
    // Update the grid state proportions for the entire column and row
    if col < grid_state.cols.len() {
        grid_state.cols[col] *= clamped_scale_x as f64;
    }
    if row < grid_state.rows.len() {
        grid_state.rows[row] *= clamped_scale_y as f64;
    }

    // Find and update all cells in the same column and row
    for (entity, transform, grid_cell) in grid_query.iter_mut() {
        if grid_cell.col == col || grid_cell.row == row {
            let current_transform = *transform;
            let mut new_transform = current_transform;
            
            // Apply scaling based on whether it's in the affected column/row
            if grid_cell.col == col {
                new_transform.scale.x = (current_transform.scale.x * clamped_scale_x).clamp(0.1, 64.0);
            }
            if grid_cell.row == row {
                new_transform.scale.y = (current_transform.scale.y * clamped_scale_y).clamp(0.1, 64.0);
            }
            
            commands.entity(entity).insert(new_transform);
        }
    }

    log::info!("Resized column {} and row {} with scale factors: ({}, {})", col, row, clamped_scale_x, clamped_scale_y);
}
