use crate::systems::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct StartupMenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for StartupMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Startup), setup_menu)
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

fn setup_menu(mut commands: Commands, textures: Res<TextureAssets>) {
    info!("Starting Grid Demo Menu");
    commands.spawn((Camera2d, Msaa::Off, StartupMenuCamera));
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::linear_rgb(0.1, 0.1, 0.15)),
            Menu,
        ))
        .with_children(|children| {
            // Title
            children.spawn((
                Text::new("Grid Demo"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(1.0, 1.0, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            // Subtitle
            children.spawn((
                Text::new("Choose a grid type to explore:"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(0.8, 0.8, 0.8)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Grid of buttons for different grid types
            children
                .spawn(Node {
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::fr(1.0), GridTrack::fr(1.0)],
                    grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::auto()],
                    column_gap: Val::Px(20.0),
                    row_gap: Val::Px(20.0),
                    width: Val::Percent(80.0),
                    max_width: Val::Px(600.0),
                    ..default()
                })
                .with_children(|grid| {
                    // Mesh Grid Button
                    // TileMap Grid Button
                    let button_colors = ButtonColors {
                        normal: Color::linear_rgb(0.8, 0.4, 0.2).with_alpha(0.8),
                        hovered: Color::linear_rgb(0.8, 0.4, 0.2),
                    };
                    grid.spawn((
                        Button,
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::all(Val::Px(20.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            min_height: Val::Px(120.0),
                            ..default()
                        },
                        BackgroundColor(button_colors.normal),
                        BorderColor(Color::linear_rgb(0.8, 0.4, 0.2)),
                        button_colors,
                        ChangeState(GameState::GridAndMotors),
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new("Grid + Motors Space"),
                            TextFont { font_size: 20.0, ..default() },
                            TextColor(Color::WHITE),
                            Node { margin: UiRect::bottom(Val::Px(8.0)), ..default() },
                        ));
                        button.spawn((
                            Text::new("Clickable tilemap grid with\nselection and motor effects"),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                            TextLayout { justify: JustifyText::Center, ..default() },
                        ));
                    });

                    // UI Grid Button
                    // Line Grid Button
                    let button_colors = ButtonColors {
                        normal: Color::linear_rgb(0.8, 0.3, 0.8).with_alpha(0.8),
                        hovered: Color::linear_rgb(0.8, 0.3, 0.8),
                    };
                    grid.spawn((
                        Button,
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::all(Val::Px(20.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            min_height: Val::Px(120.0),
                            ..default()
                        },
                        BackgroundColor(button_colors.normal),
                        BorderColor(Color::linear_rgb(0.8, 0.3, 0.8)),
                        button_colors,
                        ChangeState(GameState::Grid),
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new("Grid Space"),
                            TextFont { font_size: 20.0, ..default() },
                            TextColor(Color::WHITE),
                            Node { margin: UiRect::bottom(Val::Px(8.0)), ..default() },
                        ));
                        button.spawn((
                            Text::new("Line-based grid system\nfor geometric patterns"),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                            TextLayout { justify: JustifyText::Center, ..default() },
                        ));
                    });

                    // Flexer Space Button
                    let button_colors = ButtonColors {
                        normal: Color::linear_rgb(0.2, 0.6, 0.8).with_alpha(0.8),
                        hovered: Color::linear_rgb(0.2, 0.6, 0.8),
                    };
                    grid.spawn((
                        Button,
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::all(Val::Px(20.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            min_height: Val::Px(120.0),
                            ..default()
                        },
                        BackgroundColor(button_colors.normal),
                        BorderColor(Color::linear_rgb(0.2, 0.6, 0.8)),
                        button_colors,
                        ChangeState(GameState::Flexer),
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new("Flexer Space"),
                            TextFont { font_size: 20.0, ..default() },
                            TextColor(Color::WHITE),
                            Node { margin: UiRect::bottom(Val::Px(8.0)), ..default() },
                        ));
                        button.spawn((
                            Text::new("Flexible space system\nfor dynamic interactions"),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                            TextLayout { justify: JustifyText::Center, ..default() },
                        ));
                    });

                });
        });
    // commands
    //     .spawn((
    //         Node {
    //             flex_direction: FlexDirection::Row,
    //             align_items: AlignItems::Center,
    //             justify_content: JustifyContent::SpaceAround,
    //             bottom: Val::Px(5.),
    //             width: Val::Percent(100.),
    //             position_type: PositionType::Absolute,
    //             ..default()
    //         },
    //         Menu,
    //     ))
    //     .with_children(|children| {
    //         children
    //             .spawn((
    //                 Button,
    //                 Node {
    //                     width: Val::Px(170.0),
    //                     height: Val::Px(50.0),
    //                     justify_content: JustifyContent::SpaceAround,
    //                     align_items: AlignItems::Center,
    //                     padding: UiRect::all(Val::Px(5.)),
    //                     ..Default::default()
    //                 },
    //                 BackgroundColor(Color::NONE),
    //                 ButtonColors {
    //                     normal: Color::NONE,
    //                     ..default()
    //                 },
    //                 OpenLink("https://bevyengine.org"),
    //             ))
    //             .with_children(|parent| {
    //                 parent.spawn((
    //                     Text::new("Made with 2 Bevy"),
    //                     TextFont {
    //                         font_size: 15.0,
    //                         ..default()
    //                     },
    //                     TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
    //                 ));
    //                 parent.spawn((
    //                     ImageNode {
    //                         image: textures.bevy.clone(),
    //                         ..default()
    //                     },
    //                     Node {
    //                         width: Val::Px(32.),
    //                         ..default()
    //                     },
    //                 ));
    //             });
    //         children
    //             .spawn((
    //                 Button,
    //                 Node {
    //                     width: Val::Px(170.0),
    //                     height: Val::Px(50.0),
    //                     justify_content: JustifyContent::SpaceAround,
    //                     align_items: AlignItems::Center,
    //                     padding: UiRect::all(Val::Px(5.)),
    //                     ..default()
    //                 },
    //                 BackgroundColor(Color::NONE),
    //                 ButtonColors {
    //                     normal: Color::NONE,
    //                     hovered: Color::linear_rgb(0.25, 0.25, 0.25),
    //                 },
    //                 OpenLink("https://github.com/NiklasEi/bevy_game_template"),
    //             ))
    //             .with_children(|parent| {
    //                 parent.spawn((
    //                     Text::new("Open source"),
    //                     TextFont {
    //                         font_size: 15.0,
    //                         ..default()
    //                     },
    //                     TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
    //                 ));
    //                 parent.spawn((
    //                     ImageNode::new(textures.github.clone()),
    //                     Node {
    //                         width: Val::Px(32.),
    //                         ..default()
    //                     },
    //                 ));
    //             });
    //     });
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
