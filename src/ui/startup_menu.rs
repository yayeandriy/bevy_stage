use crate::systems::loading::FontAssets;
use crate::ui::font_utils::*;
use crate::ui::settings::SpaceSettings;

use crate::GameState;
use bevy::prelude::*;

pub struct StartupMenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for StartupMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpaceSettings>()
            .add_systems(OnEnter(GameState::Startup), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Startup)))
            .add_systems(OnExit(GameState::Startup), cleanup_menu);
    }
}

#[derive(Component)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct Menu;

#[derive(Component)]
struct StartupMenuCamera;

fn setup_menu(mut commands: Commands, fonts: Res<FontAssets>, space_settings: Res<SpaceSettings>) {
    info!("Starting Spector ID Menu");
    commands.spawn((Camera2d, Msaa::Off, StartupMenuCamera));
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart, // Left align
                justify_content: JustifyContent::FlexStart, // Start from top
                padding: UiRect::all(Val::Px(40.0)), // Increased padding for better spacing
                ..default()
            },
            BackgroundColor(Color::linear_rgb(0.95, 0.95, 0.95)), // Light gray background
            Menu,
        ))
        .with_children(|children| {
            // Title - "Spector ID"
            children.spawn((
                text_geist_regular_with_font("Spector ID", 48.0, Color::BLACK, &fonts),
                Node {
                    margin: UiRect::bottom(Val::Px(60.0)), // Increased margin for better spacing
                    ..default()
                },
            ));

            // Back button


            // Horizontal row of cards
            children
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexStart, // Left align
                    column_gap: Val::Px(24.0), // Space between cards
                    ..default()
                })
                .with_children(|cards_row| {
                    // Iterate through all available spaces and create cards
                    for space_config in &space_settings.spaces {
                        cards_row.spawn((
                            Button,
                            Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::FlexStart, // Left align text
                                justify_content: JustifyContent::SpaceBetween, // Space between title and description
                                padding: UiRect::all(Val::Px(20.0)),
                                width: Val::Px(400.0),
                                height: Val::Px(180.0),
                                
                                ..default()
                            },
                            BackgroundColor(Color::WHITE),
                            BorderColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                            BorderRadius::all(Val::Px(10.0)),
                            ButtonColors {
                                normal: Color::WHITE,
                                hovered: Color::linear_rgb(0.98, 0.98, 0.98),
                            },
                            ChangeState(space_config.game_state.clone()),
                        ))
                        .with_children(|card| {
                            // Card title
                            card.spawn((
                                text_geist_regular_with_font(space_config.name, 24.0, Color::BLACK, &fonts),
                                Node { margin: UiRect::bottom(Val::Px(8.0)), ..default() },
                            ));
                            
                            // Card description - positioned at bottom right
                            card.spawn((
                                text_geist_regular_with_font(space_config.description, 16.0, Color::linear_rgb(0.3, 0.5, 1.0), &fonts),
                                Node { 
                                    margin: UiRect::top(Val::Auto),
                                    ..default()
                                },
                            ));
                        });
                    }
                });
        });
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct OpenLink(&'static str);

fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_state, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open link {error:?}");
                    }
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(
    mut commands: Commands, 
    menu: Query<Entity, With<Menu>>,
    cameras: Query<Entity, With<StartupMenuCamera>>,
) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
    for entity in cameras.iter() {
        commands.entity(entity).despawn();
    }
}
