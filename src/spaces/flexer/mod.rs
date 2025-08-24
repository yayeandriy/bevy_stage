use bevy::prelude::*;
use crate::plugins::FlexGridPlugin;

pub struct FlexerSpacePlugin;

impl Plugin for FlexerSpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FlexGridPlugin);
    }
}
