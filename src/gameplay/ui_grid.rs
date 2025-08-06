use crate::GameState;
use bevy::{color::palettes::css::*, prelude::*};

pub struct UIGridPlugin;

const INIT_N_COLS: usize = 4;
const INIT_N_ROWS: usize = 4;

#[derive(Resource,Default)]
struct DragState {
    is_dragging: bool,
    drag_type: Option<DragType>,
    start_position: Vec2,
    border_index: usize,
}

#[derive(Clone)]
enum DragType {
    ColumnBorder(usize), // dragging column border at index
    RowBorder(usize),    // dragging row border at index
}

#[derive(Resource,Default)]
pub struct GridState {
    grid_template_columns: Vec<RepeatedGridTrack>,
    grid_template_rows: Vec<RepeatedGridTrack>,
}

#[derive(Component)]
pub struct GridContainer;

#[derive(Component)]
pub struct GridCell {
    pub row: usize,
    pub col: usize,
}

#[derive(Component)]
pub struct GridBorder {
    pub border_type: BorderType,
    pub index: usize, // which column or row border this is
}

#[derive(Clone)]
pub enum BorderType {
    Vertical,   // between columns
    Horizontal, // between rows
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for UIGridPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(DragState::default())
        .insert_resource(GridState::default())
        .add_systems(OnEnter(GameState::MeshGrid), (setup_grid, spawn_grid).chain())
        .add_systems(Update, 
            handle_drag_border.run_if(in_state(GameState::MeshGrid))
        );
    }
}

fn setup_grid(    
     mut grid_state: ResMut<GridState>
) {
   // Create a simple 4x4 grid with equal sizing
   // Use fr units for equal distribution
   grid_state.grid_template_columns = (0..INIT_N_COLS)
       .map(|_| GridTrack::fr(1.0))
       .collect();
   // Same for rows - all equal
   grid_state.grid_template_rows = (0..INIT_N_ROWS)
       .map(|_| GridTrack::fr(1.0))
       .collect();
}


fn spawn_grid(
    mut commands: Commands,
    grid_state: Res<GridState>,
) {
    // Camera is already spawned by DrawingMenuPlugin - no need to spawn another one
    
    // Top-level grid (app frame)
    commands
        .spawn((
            Node {
                // Use the CSS Grid algorithm for laying out this node
                display: Display::Grid,
                // Make node fill the entirety of its parent (in this case the window)
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // Simple 5x5 grid with equal sizing
                grid_template_columns: grid_state.grid_template_columns.clone(),
                grid_template_rows: grid_state.grid_template_rows.clone(),                          
                // Add gaps between grid cells
                column_gap: Val::Px(2.0),
                row_gap: Val::Px(2.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.8, 0.8)), // Light gray background for container
            GridContainer,
        ))
        .with_children(|builder| {
            // Create grid cells with explicit positioning
            for row in 0..4 {
                for col in 0..4 {
                    builder.spawn((
                        Node {
                            display: Display::Block,
                            // Explicitly position this item in the grid
                            grid_column: GridPlacement::start_end(col as i16 + 1, col as i16 + 2),
                            grid_row: GridPlacement::start_end(row as i16 + 1, row as i16 + 2),
                            border: UiRect::all(Val::Px(2.0)), // White border
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.5, 0.5, 0.5)), // Gray background for cells
                        BorderColor(Color::WHITE), // White border color
                        GridCell {
                            row,
                            col,
                        },
                    ));
                }
            }
        });

}

/// Create a colored rectangle node. The node has size as it is assumed that it will be
/// spawned as a child of a Grid container with `AlignItems::Stretch` and `JustifyItems::Stretch`
/// which will allow it to take its size from the size of the grid area it occupies.
fn item_rect(builder: &mut ChildSpawnerCommands, color: Srgba) {
    builder
        .spawn((
            Node {
                display: Display::Block,
                width: Val::Px(40.0), // Fill the grid cell completely
                height: Val::Px(40.0), // Fill the grid cell completely
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(color.into()),
        ));
        // .with_children(|builder| {
        //     builder.spawn((Node::default(), BackgroundColor(color.into())));
        // });
}
// fn spawn_nested_text_bundle(builder: &mut ChildSpawnerCommands, text: &str) {
//     builder.spawn((
//         Text::new(text),
//         TextFont::default(),
//         TextColor::BLACK,
//     ));
// }

fn handle_drag_border(
    mut drag_state: ResMut<DragState>,
    mut grid_state: ResMut<GridState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    grid_query: Query<&Node, With<GridContainer>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let (camera, camera_transform) = *camera;
    
    // Get cursor position in world space
    let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    else {
        return;
    };


    if mouse_button_input.just_pressed(MouseButton::Left) {
        // Start dragging - detect which border we're near
        if grid_query.single().is_ok() {
            // Check if we're near a column border
            // This is a simplified detection - in a real implementation you'd need
            // to calculate the actual grid line positions
            log::info!("Detected drag near column border at position: {:?}", cursor_pos);

            drag_state.is_dragging = true;
            drag_state.start_position = cursor_pos;
            
            // For now, let's assume we're dragging the first column border
            drag_state.drag_type = Some(DragType::ColumnBorder(0));
            drag_state.border_index = 0;
            
            log::info!("Started dragging at position: {:?}", cursor_pos);
        }
    } else if mouse_button_input.just_released(MouseButton::Left) {
        if drag_state.is_dragging {
            drag_state.is_dragging = false;
            drag_state.drag_type = None;
            log::info!("Stopped dragging");
        }
    } else if drag_state.is_dragging {
        // Handle dragging - update grid dimensions
        let delta = cursor_pos - drag_state.start_position;
        
        if let Some(drag_type) = &drag_state.drag_type {
            match drag_type {
                DragType::ColumnBorder(col_index) => {
                    // Update column width based on drag delta
                    if *col_index < grid_state.grid_template_columns.len() {
                        // For now, just modify the first column
                        if *col_index == 0 && grid_state.grid_template_columns.len() > 0 {
                            // Simple approach: replace with a new fr value based on delta
                            let new_fr_value = 2.0 + (delta.x * 0.01);
                            grid_state.grid_template_columns[0] = GridTrack::fr(new_fr_value.max(0.1));
                            log::info!("Updated column {} width to {}fr", col_index, new_fr_value);
                        }
                    }
                }
                DragType::RowBorder(row_index) => {
                    // Update row height based on drag delta
                    if *row_index < grid_state.grid_template_rows.len() {
                        if *row_index == 0 && grid_state.grid_template_rows.len() > 0 {
                            let new_px_value = 80.0 + delta.y;
                            grid_state.grid_template_rows[0] = GridTrack::px(new_px_value.max(30.0));
                            log::info!("Updated row {} height to {}px", row_index, new_px_value);
                        }
                    }
                }
            }
        }
    }
}
