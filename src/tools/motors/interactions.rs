use bevy::prelude::*;
use bevy_picking::prelude::{Pointer, Drag, Click};
use std::fmt::Debug;

use crate::tools::motors::Motor;

pub fn toggle_motor_on<E: Debug + Clone + Reflect>() -> impl Fn(Trigger<E>, Query<&mut Motor>) {
    move |ev, mut motors| {
       
    }
}
