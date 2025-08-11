use bevy::prelude::*;
use bevy_picking::prelude::{Pickable, Pointer, Click};
use crate::gameplay::motors::{Background, Motor, MotorButton};



pub fn startup(
    mut commands: Commands,   
    windows: Query<&Window>,
) {
    if let Ok(window) = windows.single() {
        let window_size = Vec2::new(window.width(), window.height());
        commands.spawn((
            Sprite::from_color(Color::hsla(90.0, 0.6, 0.2, 1.0), Vec2::new(window_size.x, window_size.y)),
            Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
            Background,
        ));
        commands.spawn((
            Text2d::new("Motors"),
            Transform::from_translation(Vec3::new(0.0, window_size.y / 2.0 - 50.0, 0.0)),
        ));
        
        // Create multiple motor buttons with different frequencies and colors
        let motor_configs = [
            (0.05, Color::hsla(20.0, 0.6, 0.2, 1.0)),   // Slow red motor
            (0.15, Color::hsla(120.0, 0.6, 0.2, 1.0)),  // Medium green motor
            (0.3, Color::hsla(240.0, 0.6, 0.2, 1.0)),   // Fast blue motor
        ];
        
        let start_x = -window_size.x / 2.0 + 100.0;
        for (i, (freq, color)) in motor_configs.iter().enumerate() {
            let x = start_x + (i as f32 * 120.0);
            commands.spawn((
                Sprite::from_color(*color, Vec2::new(100.0, 100.0)),
                Transform::from_translation(Vec3::new(x, 0.0, -1.0)),
                Pickable::default(),
                MotorButton { freq: *freq },
                Motor { freq: *freq }, // Each motor button has its own motor effect
            ))
             .observe(click_on_motor());
        }
    }
}



pub fn motors_update(
    mut motor_button_query: Query<(Entity, &Motor, &mut Sprite), With<MotorButton>>,
    mut grid_cell_query: Query<(Entity, &Motor, &mut Sprite), (Without<MotorButton>, With<crate::gameplay::tile_map_grid::components::GridCell>)>,
    time: Res<Time>,
) {
    // Update motor buttons with full color animation (same as grid cells)
    for (_entity, motor, mut sprite) in motor_button_query.iter_mut() {
        let freq = motor.freq;
        let time_factor = (time.elapsed_secs() * freq as f32).sin();
        let hue = (time_factor * 360.0).abs() % 360.0;
        let new_color = Color::hsl(hue, 0.8, 0.6);
        sprite.color = new_color;
    }
    
    // Update grid cells with motors (with logging)
    for (entity, motor, mut sprite) in grid_cell_query.iter_mut() {
        let freq = motor.freq;
        let time_factor = (time.elapsed_secs() * freq as f32).sin();
        let hue = (time_factor * 360.0).abs() % 360.0;
        let new_color = Color::hsl(hue, 0.8, 0.6);
        sprite.color = new_color;
        log::info!("Motor update (grid cell): entity={:?}, freq={}", entity, freq);
    }
}


fn click_on_motor() -> impl Fn(Trigger<Pointer<Click>>, Commands, Query<&MotorButton>, Query<(Entity, Option<&Motor>), (With<crate::gameplay::tile_map_grid::components::SelectedCell>, With<crate::gameplay::tile_map_grid::components::GridCell>, With<Sprite>)>) {
    move |ev, mut commands, motor_buttons, selected_grid_cells| {
        let clicked_entity = ev.target();
        log::info!("Motor button clicked: {:?}", clicked_entity);
        
        // Get the motor button's parameters
        if let Ok(motor_button) = motor_buttons.get(clicked_entity) {
            // Find selected grid cell sprites and toggle Motor component
            for (selected_entity, existing_motor) in selected_grid_cells.iter() {
                if let Some(_motor) = existing_motor {
                    // Motor exists, remove it
                    commands.entity(selected_entity).remove::<Motor>();
                    log::info!("Removed Motor component from selected grid cell sprite: {:?}", selected_entity);
                } else {
                    // Motor doesn't exist, add it
                    commands.entity(selected_entity).insert(Motor { freq: motor_button.freq });
                    log::info!("Added Motor component to selected grid cell sprite: {:?} with freq: {}", selected_entity, motor_button.freq);
                }
            }
        } else {
            log::warn!("Clicked entity is not a motor button: {:?}", clicked_entity);
        }
    }
}