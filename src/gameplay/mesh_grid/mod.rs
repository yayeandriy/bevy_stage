mod components;
mod events;
mod mouse_handlers;
mod mouse_interactions;
mod resources;
mod systems;

pub use components::*;
pub use events::*;
pub use resources::*;

use crate::GameState;
use bevy::prelude::*;
use mouse_handlers::*;
use systems::*;

pub struct MeshGridPlugin;

impl Plugin for MeshGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DragState::default())
            .insert_resource(MeshGridState::default())
            .add_systems(
                OnEnter(GameState::MeshGrid), 
                (setup_grid, spawn_grid).chain()
            )
            .add_systems(
                Update, 
                (handle_mouse_click, handle_mouse_drag)
                    .run_if(in_state(GameState::MeshGrid))
            )
            .add_observer(toggle_cell_observer)
            .add_observer(resize_cell_observer);
    }
}
