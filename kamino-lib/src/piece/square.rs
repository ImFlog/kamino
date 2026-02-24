use bevy::{math::vec3, prelude::*};
use kamino_macro::PieceBehavior;
use std::vec;

#[derive(PieceBehavior)]
pub struct Square {
    positions: Vec<Vec3>,
    color: Color,
    moving: bool,
    error_timer: f32,
}

impl Square {
    pub fn new(start_x: i32, start_y: i32) -> Self {
        let positions = vec![vec3(start_x as f32, start_y as f32, 1.)];
        Square {
            positions,
            color: Color::srgb(0.01, 1.0, 0.425_367_7),
            moving: false,
            error_timer: 0.0,
        }
    }
}
