use crate::GameState;
use bevy::prelude::*;

/// Configuration for a space that can be selected from the startup menu
#[derive(Clone, Debug)]
pub struct SpaceConfig {
    /// Display name for the space
    pub name: &'static str,
    /// Description of the space
    pub description: &'static str,
    /// The game state to transition to when this space is selected
    pub game_state: GameState,
    /// Optional icon or visual identifier
    pub icon: Option<&'static str>,
}

impl SpaceConfig {
    /// Creates a new space configuration
    pub fn new(name: &'static str, description: &'static str, game_state: GameState) -> Self {
        Self {
            name,
            description,
            game_state,
            icon: None,
        }
    }

    /// Creates a new space configuration with an icon
    pub fn with_icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Collection of all available spaces
#[derive(Resource)]
pub struct SpaceSettings {
    pub spaces: Vec<SpaceConfig>,
}

impl Default for SpaceSettings {
    fn default() -> Self {
        Self {
            spaces: vec![
                SpaceConfig::new("Liner", "Line grid", GameState::Grid),
                SpaceConfig::new("Motored", "Grid + Motors", GameState::GridAndMotors),
                SpaceConfig::new("Flexer", "Flex grid", GameState::Flexer),
            ],
        }
    }
}

impl SpaceSettings {
    /// Returns a reference to all available spaces
    pub fn get_spaces(&self) -> &[SpaceConfig] {
        &self.spaces
    }

    /// Adds a new space to the configuration
    pub fn add_space(&mut self, space: SpaceConfig) {
        self.spaces.push(space);
    }

    /// Removes a space by name
    pub fn remove_space(&mut self, name: &str) {
        self.spaces.retain(|space| space.name != name);
    }
}
