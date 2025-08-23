use bevy::prelude::*;

use crate::plugins::motors::{Motor, ToggleMotor};




// pub fn toggle_motor_observer(
//     trigger: Trigger<ToggleMotor>,
//     mut commands: Commands,
//     mut grid_query: Query<(Entity, &mut Transform, &Motor, &mut Sprite)>,
//     time: Res<Time>,
// ) {
//     let event = trigger.event();
//     for (entity, mut transform, motor, mut sprite) in grid_query.iter_mut() {
//         let freq = motor.freq;
//         let time_factor = (time.elapsed_secs() * freq as f32).sin();
//         let hue = (time_factor * 360.0).abs() % 360.0;
//         let new_color = Color::hsl(hue, 0.8, 0.6);

//         sprite.color = new_color;
//     }
// }
