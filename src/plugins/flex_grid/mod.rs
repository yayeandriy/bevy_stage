use bevy::prelude::*;
use crate::GameState;

#[derive(Component)]
struct FlexGridEntity;

pub struct FlexGridPlugin;

impl Plugin for FlexGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::Flexer), 
                spawn_flex_grid
            )
            .add_systems(
                OnExit(GameState::Flexer),
                cleanup_flex_grid
            );
    }
}

fn spawn_flex_grid(mut commands: Commands) {
    info!("Spawning Flex Grid");
    
    // Main red 800x800 sprite
    let main_sprite_transform = Transform::from_xyz(0.0, 0.0, 0.0);
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(800.0, 800.0)),
            ..default()
        },
        main_sprite_transform,
        FlexGridEntity,
    ));
}

fn cleanup_flex_grid(
    mut commands: Commands,
    entities: Query<Entity, With<FlexGridEntity>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}
