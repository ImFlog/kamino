use bevy::prelude::*;

use crate::cursor::Cursor;

use crate::SQUARE_WIDTH;

/// Marker component for entities that represent piece positions
#[derive(Component)]
pub struct Position;

/// Trait representing a game piece that can be moved, rotated, and snapped to the board
pub trait Piece {
    /// Returns the current positions of all squares that make up this piece
    fn positions(&self) -> Vec<Vec3>;

    /// Returns the color used to render this piece
    fn color(&self) -> Color;

    /// Rotates the piece 90 degrees clockwise around its first position
    fn rotate(&mut self);

    /// Snaps the piece positions to align with the board grid
    fn snap(&mut self);

    /// Moves the piece to follow the cursor position
    fn move_it(&mut self, cursor: &Res<Cursor>);

    /// Sets whether this piece is currently being moved by the player
    fn set_moving(&mut self, moving: bool);

    /// Returns true if this piece is currently being moved by the player
    fn is_moving(&self) -> bool;

    /// Checks if a cursor position is within any of the piece's squares using
    /// point-in-rectangle collision detection
    fn is_even_odd(&self, current_pos: Vec2) -> bool {
        self.positions().iter().any(|piece_pos| {
            piece_pos.x - (SQUARE_WIDTH / 2) as f32 <= current_pos.x
                && current_pos.x <= piece_pos.x + (SQUARE_WIDTH / 2) as f32
                && piece_pos.y - (SQUARE_WIDTH / 2) as f32 <= current_pos.y
                && current_pos.y <= piece_pos.y + (SQUARE_WIDTH / 2) as f32
        })
    }
}
