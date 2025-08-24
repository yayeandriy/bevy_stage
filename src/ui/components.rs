use bevy::prelude::*;

#[derive(Component)]
pub struct BackButton;

#[derive(Component)]
pub struct BackButtonColors {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

impl Default for BackButtonColors {
    fn default() -> Self {
        BackButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
            pressed: Color::linear_rgb(0.35, 0.75, 0.35),
        }
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

pub fn spawn_back_button(
    commands: &mut Commands,
    _asset_server: &AssetServer,
    fonts: &crate::systems::loading::FontAssets,
) -> Entity {
    commands
        .spawn((
            Button,
            BackButton,
            Node {
                width: Val::Px(80.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::linear_rgb(0.15, 0.15, 0.15)),
            BorderColor(Color::BLACK),
            BorderRadius::all(Val::Px(5.0)),
            BackButtonColors::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Back"),
                TextFont {
                    font: fonts.geist_regular.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        })
        .id()
}
