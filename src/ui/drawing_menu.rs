use crate::systems::loading::TextureAssets;
use crate::ui::startup_menu::ButtonColors;
use crate::GameState;
use bevy::prelude::*;

pub struct DrawingMenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for DrawingMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::TileMapGrid), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::TileMapGrid)))
            .add_systems(OnExit(GameState::TileMapGrid), cleanup_menu);
    }
}


#[derive(Component)]
struct Menu;

#[derive(Component)]
struct DrawingMenuCamera;

fn setup_menu(mut commands: Commands, textures: Res<TextureAssets>) {
    info!("menu");
    commands.spawn((Camera2d, Msaa::Off, DrawingMenuCamera));
    // commands
    //     .spawn((
    //         Node {
    //             width: Val::Percent(100.0),
    //             height: Val::Percent(100.0),
    //             flex_direction: FlexDirection::Column,
    //             align_items: AlignItems::Center,
    //             justify_content: JustifyContent::Center,
    //             ..default()
    //         },
    //         Menu,
    //     ))
    //     .with_children(|children| {
    //         let button_colors = ButtonColors::default();
    //         children
    //             .spawn((
    //                 Button,
    //                 Node {
    //                     width: Val::Px(140.0),
    //                     height: Val::Px(50.0),
    //                     justify_content: JustifyContent::Center,
    //                     align_items: AlignItems::Center,
    //                     ..Default::default()
    //                 },
    //                 BackgroundColor(button_colors.normal),
    //                 button_colors,
    //                 ChangeState(GameState::Startup),
    //             ))
    //             .with_child((
    //                 Text::new("Back"),
    //                 TextFont {
    //                     font_size: 40.0,
    //                     ..default()
    //                 },
    //                 TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
    //             ));
           
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
    cameras: Query<Entity, With<DrawingMenuCamera>>,
) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
    for entity in cameras.iter() {
        commands.entity(entity).despawn();
    }
}
