use bevy::prelude::*;
use crate::systems::loading::FontAssets;

/// Helper function to create text with Geist Regular font
pub fn text_geist_regular(text: &str, font_size: f32, color: Color) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
    )
}

/// Helper function to create text with Geist Medium font
pub fn text_geist_medium(text: &str, font_size: f32, color: Color) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
    )
}

/// Helper function to create text with Geist Bold font
pub fn text_geist_bold(text: &str, font_size: f32, color: Color) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
    )
}

/// Helper function to create text with Geist Regular font and custom font handle
pub fn text_geist_regular_with_font(
    text: &str, 
    font_size: f32, 
    color: Color, 
    font_assets: &FontAssets
) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font_size,
            font: font_assets.geist_regular.clone(),
            ..default()
        },
        TextColor(color),
    )
}

/// Helper function to create text with Geist Medium font and custom font handle
pub fn text_geist_medium_with_font(
    text: &str, 
    font_size: f32, 
    color: Color, 
    font_assets: &FontAssets
) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font_size,
            font: font_assets.geist_medium.clone(),
            ..default()
        },
        TextColor(color),
    )
}

/// Helper function to create text with Geist Bold font and custom font handle
pub fn text_geist_bold_with_font(
    text: &str, 
    font_size: f32, 
    color: Color, 
    font_assets: &FontAssets
) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font_size,
            font: font_assets.geist_bold.clone(),
            ..default()
        },
        TextColor(color),
    )
}


