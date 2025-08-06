use bevy::prelude::*;

pub const INIT_N_COLS: usize = 63;
pub const INIT_N_ROWS: usize = 64;
pub const GRID_GAP: f32 = 4.0; // Gap between grid cells in pixels

#[derive(Resource, Default)]
pub struct DragState {
    pub is_dragging: bool,
    pub drag_type: Option<DragType>,
    pub start_position: Vec2,
    pub border_index: usize,
}

#[derive(Clone)]
pub enum DragType {
    ColumnBorder(usize), // dragging column border at index
    RowBorder(usize),    // dragging row border at index
}

#[derive(Resource, Default)]
pub struct MeshGridState {
    pub cols: Vec<f64>,
    pub rows: Vec<f64>,
}
