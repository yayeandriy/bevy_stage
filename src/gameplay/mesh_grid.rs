use crate::GameState;
use bevy::prelude::*;

pub struct MeshGridPlugin;

const INIT_N_COLS: usize = 63;
const INIT_N_ROWS: usize = 64;
const GRID_GAP: f32 = 4.0; // Gap between grid cells in pixels

#[derive(Resource,Default)]
#[allow(dead_code)]
struct DragState {
    is_dragging: bool,
    drag_type: Option<DragType>,
    start_position: Vec2,
    border_index: usize,
}

#[derive(Clone)]
#[allow(dead_code)]
enum DragType {
    ColumnBorder(usize), // dragging column border at index
    RowBorder(usize),    // dragging row border at index
}

#[derive(Resource,Default)]
pub struct MeshGridState {
    cols: Vec<f64>,
    rows: Vec<f64>,
}

#[derive(Component)]
pub struct MeshGridContainer;

#[derive(Component, Debug, Clone)]
pub struct GridCell {
    pub row: usize,
    pub col: usize,
    pub is_black: bool, // Flag to indicate if the cell is black
}

impl Default for GridCell {
    fn default() -> Self {
        Self {
            row: 0,
            col: 0,
            is_black: false,
        }
    }
}
#[derive(Event)]
struct ToggleCell {
    #[allow(dead_code)]
    pub row: usize,
    #[allow(dead_code)]
    pub col: usize,
}
/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for MeshGridPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(DragState::default())
        .insert_resource(MeshGridState::default())
        .add_systems(OnEnter(GameState::Drawing), (setup_grid, spawn_grid).chain())
        .add_systems(Update, 
            (handle_mouse_click, handle_mouse_drag).run_if(in_state(GameState::Drawing))
        )
        .add_observer(
            |trigger: Trigger<ToggleCell>,
            commands: Commands,
            grid_query: Query<(Entity, &Transform, &mut MeshMaterial2d<ColorMaterial>, &GridCell), With<GridCell>>,
            materials: ResMut<Assets<ColorMaterial>>,| {
                let event = trigger.event();
                let toggle_cell_data = ToggleCell {
                    row: event.row,
                    col: event.col,
                };
                toggle_cell(
                    grid_query,
                    commands,
                    materials,
                    toggle_cell_data, // Clone the event instead of dereferencing
                );
                // update_all_cells(
                //     commands,
                //     grid_query,
                //     materials,
                // );
            },
        );
    }
}

fn setup_grid(    
     mut grid_state: ResMut<MeshGridState>
) {
   // Create a simple 4x4 grid with equal sizing
   // Use fr units for equal distribution
    grid_state.cols = (0..INIT_N_COLS)
     .map(|_| 1.0)
     .collect();
    grid_state.cols.push(2.0);
   // Same for rows - all equal
    grid_state.rows = (0..INIT_N_ROWS)
       .map(|_| 1.0)
       .collect();
}


fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    grid_state: Res<MeshGridState>,
    cells: Query<&GridCell>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {

    let Ok(window) = windows.single() else {
        return;
    };
    
    // Debug: Print window dimensions
    log::info!("Window dimensions: {}x{}", window.width(), window.height());
    // log::info!("Grid state - cols: {:?}, rows: {:?}", grid_state.cols, grid_state.rows);
    
    // Calculate available space for each cell (window size minus gaps)
    let num_cols = grid_state.cols.len();
    let num_rows = grid_state.rows.len();
    let available_width = window.width() - GRID_GAP * (num_cols - 1) as f32;
    let available_height = window.height() - GRID_GAP * (num_rows - 1) as f32;
    
    // Calculate the total proportional units
    let total_col_units: f64 = grid_state.cols.iter().sum();
    let total_row_units: f64 = grid_state.rows.iter().sum();
    
    let actual_cols_widths: Vec<f64> = grid_state.cols.iter().map(|&c| (c * available_width as f64 / total_col_units)).collect();
    let actual_rows_heights: Vec<f64> = grid_state.rows.iter().map(|&r| (r * available_height as f64 / total_row_units)).collect();
    
    
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
            let color = Color::hsl( 0.5, 0.8, 0.6);
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

fn find_cell_at_col_and_row<'a>(
    cells: &'a Query<&GridCell>,
    col: usize,
    row: usize,
) -> Option<&'a GridCell> {
    cells.iter().find(|&cell| cell.col == col && cell.row == row)
}

#[allow(dead_code)]
fn update_all_cells(
    grid_query: Query<(Entity, &Transform, &mut MeshMaterial2d<ColorMaterial>, &mut GridCell), With<GridCell>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, _transform, _mesh_material, grid_cell) in grid_query.iter() {
        // Example logic to update cell
        let new_color = if grid_cell.is_black {
            Color::BLACK
        } else {
            Color::WHITE
        };
        commands.entity(entity).insert(MeshMaterial2d(materials.add(ColorMaterial::from(new_color))));
    }
}

fn toggle_cell(
   grid_query: Query<(Entity, &Transform, &mut MeshMaterial2d<ColorMaterial>, &GridCell), With<GridCell>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ToggleCell { row, col }: ToggleCell,
) {
    for (entity, _transform, _mesh_material, grid_cell) in grid_query.iter() {
        if grid_cell.row != row || grid_cell.col != col {
            continue; // Skip if this cell is not the one we want to toggle
        }
        // Example logic to update cell
        let new_color = if grid_cell.is_black {
            Color::BLACK
        } else {
            Color::WHITE
        };
        commands.entity(entity).insert(MeshMaterial2d(materials.add(ColorMaterial::from(new_color))));
    }
} 


fn handle_mouse_click(
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
        let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
        else {
            return;
        };
        if grid_query.is_empty() {
            log::warn!("No grid cells found! Cannot handle click.");
            return;
        }
        let commands = &mut commands;
        grid_query.iter_mut().for_each(|(_entity, transform, mut grid_cell)| {
            let grid_pos = transform.translation.truncate();
            // Check if cursor is within the bounds of the grid cell
            if (grid_pos.x - 10.0..=grid_pos.x + 10.0).contains(&cursor_pos.x) &&
               (grid_pos.y - 10.0..=grid_pos.y + 10.0).contains(&cursor_pos.y) {
                grid_cell.is_black = !grid_cell.is_black; // Toggle black state
                commands.trigger(ToggleCell {
                    row: grid_cell.row,
                    col: grid_cell.col,
                });
            }
        });
        
    }
   
}

fn handle_mouse_drag(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_drag_state: ResMut<DragState>,
    windows: Query<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut grid_query: Query<(Entity, &Transform, &mut GridCell), With<GridCell>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let (camera, camera_transform) = *camera;
    let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
        else {
            return;
        };

    if mouse_button_input.just_pressed(MouseButton::Left) {
        
        mouse_drag_state.is_dragging = true;
        mouse_drag_state.start_position = cursor_pos;
        log::info!("Mouse drag started at {:?}", cursor_pos);
       return; 
        
    }
    if mouse_button_input.just_released(MouseButton::Left) {
        mouse_drag_state.is_dragging = false;
        mouse_drag_state.drag_type = None;
        log::info!("Mouse drag ended");
        return;
    }


    if mouse_drag_state.is_dragging && (keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.just_pressed(KeyCode::ShiftLeft)) {   
        let delta_pos = mouse_drag_state.start_position - cursor_pos;
        log::info!("Mouse drag with Shift key held down - delta: {:?}", delta_pos);
        let mut col = 0;
        let mut row = 0;
        grid_query.iter().for_each(|(_entity, transform, grid_cell)| {
           let grid_pos = transform.translation.truncate();
           // Check if cursor is within the bounds of the grid cell
           if (grid_pos.x - 10.0..=grid_pos.x + 10.0).contains(&mouse_drag_state.start_position.x) &&
               (grid_pos.y - 10.0..=grid_pos.y + 10.0).contains(&mouse_drag_state.start_position.y) {
              col = grid_cell.col;
              row = grid_cell.row;
           }
       });
       if col != 0 || row != 0 {
           log::info!("Dragging cell at row: {}, col: {}", row, col);
           let delta_x = delta_pos.x / (window.width() / grid_query.iter().count() as f32);
           let delta_y = delta_pos.y / (window.height() / grid_query.iter().count() as f32);
           // Here you can implement the logic to resize the grid cell
           // For now, let's just log the action
       }
        // Add your special shift+drag behavior here
        return;
    }

    if mouse_drag_state.is_dragging {
       grid_query.iter_mut().for_each(|(_entity, transform, mut grid_cell)| {
           let grid_pos = transform.translation.truncate();
           // Check if cursor is within the bounds of the grid cell
           if (grid_pos.x - 10.0..=grid_pos.x + 10.0).contains(&cursor_pos.x) &&
               (grid_pos.y - 10.0..=grid_pos.y + 10.0).contains(&cursor_pos.y) {
               grid_cell.is_black = !grid_cell.is_black; // Toggle black state
               commands.trigger(ToggleCell {
                   row: grid_cell.row,
                   col: grid_cell.col,
               });
           }
       });
    }

   
}