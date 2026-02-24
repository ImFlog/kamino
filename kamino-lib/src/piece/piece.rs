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

    /// Moves the piece to the specified position
    #[allow(dead_code)]
    fn move_to_position(&mut self, pos: Vec3);

    /// Moves the piece to follow the cursor position
    fn move_to_cursor(&mut self, cursor: &Res<Cursor>);

    /// Sets whether this piece is currently being moved by the player
    fn set_moving(&mut self, moving: bool);

    /// Sets the current positions of all squares that make up this piece
    fn set_positions(&mut self, positions: Vec<Vec3>);

    /// Returns the remaining duration of the error state (outline)
    fn error_timer(&self) -> f32;

    /// Sets the error timer
    fn set_error_timer(&mut self, duration: f32);

    /// Updates the error timer
    fn update_error_timer(&mut self, delta: f32);

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
