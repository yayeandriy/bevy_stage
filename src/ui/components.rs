use bevy::prelude::*;

#[derive(Component)]
pub struct BackButton;

#[derive(Component)]
pub struct BackButtonUi;

#[derive(Component)]
pub struct BackButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for BackButtonColors {
    fn default() -> Self {
        BackButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
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
