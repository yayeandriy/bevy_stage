mod events;
mod observers;

use bevy::prelude::*;
use crate::{GameState, plugins::flex_grid::{events::BackButtonPressed, observers::back_button_pressed_observer}};

#[derive(Component)]
struct BackButton;

#[derive(Component)]
struct FlexGridEntity;

pub struct FlexGridPlugin;

impl Plugin for FlexGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BackButtonPressed>()
            .add_systems(
                OnEnter(GameState::Flexer), 
                startup
            )
            .add_systems(
                Update,
                handle_back_button_click.run_if(in_state(GameState::Flexer))
            )
            .add_systems(
                OnExit(GameState::Flexer),
                cleanup_flex_grid
            )
            .add_observer(back_button_pressed_observer);
    }
}

fn setup_flex_grid(mut commands: Commands) {
    info!("Starting Flex Grid");
    commands.spawn((Camera2d, Msaa::Off, FlexGridEntity));
    
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

    // Back button - positioned at top-left corner of screen
    commands.spawn((
        Button,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            width: Val::Px(100.0),
            height: Val::Px(50.0),
            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(Color::BLACK),
        BackgroundColor(Color::WHITE),
        BackButton,
        FlexGridEntity,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Back"),
            TextColor(Color::BLACK),
            TextFont {
                font_size: 16.0,
                ..default()
            },
        ));
    });
}

fn startup(commands: Commands) {
    setup_flex_grid(commands);
}

fn cleanup_flex_grid(
    mut commands: Commands,
    entities: Query<Entity, With<FlexGridEntity>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}

fn handle_back_button_click(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut commands: Commands,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.trigger(BackButtonPressed);
            }
            _ => {}
        }
    }
}
