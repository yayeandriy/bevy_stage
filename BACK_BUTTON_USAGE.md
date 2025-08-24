# Back Button Usage Guide

This project now includes a reusable back button component that uses the `assets/ui/back.png` file.

## Components

### BackButton
A marker component that identifies back button entities.

### BackButtonColors
Defines the normal and hover colors for the back button:
```rust
BackButtonColors {
    normal: Color::linear_rgb(0.15, 0.15, 0.15),
    hovered: Color::linear_rgb(0.25, 0.25, 0.25),
}
```

## Functions

### spawn_back_button
Creates a simple back button with just the PNG icon:
```rust
use crate::ui::components::*;
use crate::ui::assets::UiAssets;

spawn_back_button(
    parent,
    &ui_assets,
    32.0, // size
    BackButtonColors::default(),
);
```

### spawn_back_button_with_text
Creates a back button with both the PNG icon and text:
```rust
spawn_back_button_with_text(
    parent,
    &ui_assets,
    &fonts,
    "Back",
    32.0, // size
    BackButtonColors::default(),
);
```

## Usage in Plugins

To use the back button in your plugin:

1. Add the UI assets as a resource:
```rust
use crate::ui::assets::UiAssets;

fn setup_my_ui(mut commands: Commands, ui_assets: Res<UiAssets>, fonts: Res<FontAssets>) {
    // Your UI setup code
}
```

2. Add the interaction system:
```rust
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_back_button_interactions);
    }
}
```

3. Spawn the back button:
```rust
parent.with_children(|children| {
    spawn_back_button_with_text(
        children,
        &ui_assets,
        &fonts,
        "Back",
        32.0,
        BackButtonColors::default(),
    );
});
```

## Customization

You can customize the back button by:
- Changing colors with `BackButtonColors`
- Adjusting size (affects both icon and text scaling)
- Modifying the text label
- Styling the container node

## Example Implementation

See `src/ui/startup_menu.rs` for a complete example of how the back button is integrated into the startup menu.

## Spaces Integration

The back button is also integrated into all game spaces (Grid, Flexer, GridAndMotors) using the PNG asset instead of the previous white square sprites.
