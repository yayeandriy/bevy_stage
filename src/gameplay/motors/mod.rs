mod components;
mod events;
mod interactions;
mod observers;
mod plugin;
mod systems;

// Re-export the plugin for easy access
pub use plugin::MotorsPlugin;

// Re-export commonly used components and events if needed by other modules
pub use components::*;
pub use events::*;
