use bevy::prelude::*;
use kamino_macro::PieceBehavior;

use crate::piece::{piece_builder::PieceBuilder, SQUARE_WIDTH};

#[derive(PieceBehavior)]
pub struct Rectangle {
    positions: Vec<Vec3>,
    color: Color,
    moving: bool,
    error_timer: f32,
}

impl Rectangle {
    pub fn new(start_x: i32, start_y: i32) -> Self {
        let mut positions = vec![];
        // TODO: Use horizontal rectangle from pieceBuilder ?
        for i in 0..3 {
            positions.append(&mut PieceBuilder::new_horizontal_rectangle(
                start_x,
                start_y + (i * SQUARE_WIDTH),
                1,
                1.,
            ));
        }
        Rectangle {
            positions,
            color: Color::srgb(0.68, 0.1, 1.03),
            moving: false,
            error_timer: 0.0,
        }
    }
}
