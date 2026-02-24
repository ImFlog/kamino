use bevy::prelude::*;
use bevy::sprite::Sprite;

use crate::piece::SQUARE_WIDTH;

/// Plugin that creates and renders the game board
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::new())
            .add_systems(Startup, draw_board);
    }
}

/// Represents the game board where pieces can be placed.
/// Currently a fixed 3x5 grid:
/// ```text
/// * * * * *
/// * * * * *
/// * * * * *
/// ```
#[derive(Resource)]
pub struct Board {
    /// Positions of all squares that make up the board
    pub positions: Vec<BoardPosition>,
    /// Minimum X coordinate of the board (left edge)
    pub min_x: f32,
    /// Minimum Y coordinate of the board (bottom edge)
    pub min_y: f32,
    /// Maximum X coordinate of the board (right edge)
    pub max_x: f32,
    /// Maximum Y coordinate of the board (top edge)
    pub max_y: f32,
}

impl Board {
    pub fn new() -> Self {
        let nb_rows = 3;
        let nb_cols = 5;
        let mut positions = vec![];

        // Center the board by adjusting start_x and start_y
        // We ensure start_x and start_y are aligned with SQUARE_WIDTH
        let start_x = -(nb_cols / 2) * SQUARE_WIDTH;
        let start_y = -(nb_rows / 2) * SQUARE_WIDTH;

        for r in 0..nb_rows {
            for c in 0..nb_cols {
                let x = (start_x + c * SQUARE_WIDTH) as f32;
                let y = (start_y + r * SQUARE_WIDTH) as f32;
                positions.push(BoardPosition::new(Vec3::new(x, y, 0.)));
            }
        }
        Board {
            positions,
            min_x: (start_x) as f32,
            min_y: (start_y) as f32,
            max_x: (start_x + (nb_cols - 1) * SQUARE_WIDTH) as f32,
            max_y: (start_y + (nb_rows - 1) * SQUARE_WIDTH) as f32,
        }
    }

    /// Checks if a piece (defined by its square positions) is within the board's boundaries
    pub fn fits_piece(&self, piece_positions: &[Vec3]) -> bool {
        let tolerance = SQUARE_WIDTH as f32 / 2.0;
        piece_positions.iter().all(|t| {
            self.min_x - tolerance <= t.x
                && t.x <= self.max_x + tolerance
                && self.min_y - tolerance <= t.y
                && t.y <= self.max_y + tolerance
        })
    }

    /// Checks if all positions occupied by a piece are available (not already filled)
    pub fn can_place_piece(&self, piece_positions: &[Vec3]) -> bool {
        !piece_positions.iter().any(|t| {
            self.positions
                .iter()
                .any(|bp| bp.is_filled() && bp.coordinates().xy().distance(t.xy()) < 0.1)
        })
    }

    /// Marks board positions as filled based on the piece's square positions
    pub fn fill_piece(&mut self, piece_positions: &[Vec3]) {
        for position in piece_positions {
            if let Some(bp) = self
                .positions
                .iter_mut()
                .find(|bp| bp.coordinates().xy().distance(position.xy()) < 0.1)
            {
                bp.set_filled(true);
            }
        }
    }

    /// Marks board positions as empty based on the piece's square positions
    pub fn clear_piece(&mut self, piece_positions: &[Vec3]) {
        for position in piece_positions {
            if let Some(bp) = self
                .positions
                .iter_mut()
                .find(|bp| bp.coordinates().xy().distance(position.xy()) < 0.1)
            {
                bp.set_filled(false);
            }
        }
    }
}

// Systems
fn draw_board(board: Res<Board>, mut commands: Commands) {
    let color = Color::srgb(0.60, 0.40, 0.);
    board.positions.iter().for_each(|position| {
        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(
                    (SQUARE_WIDTH - 1) as f32,
                    (SQUARE_WIDTH - 1) as f32,
                )),
                ..default()
            },
            Transform::from_translation(position.coordinates),
        ));
    });
}

pub struct BoardPosition {
    pub coordinates: Vec3,
    pub filled: bool,
}

impl BoardPosition {
    pub fn new(coordinates: Vec3) -> Self {
        BoardPosition {
            coordinates,
            filled: false,
        }
    }

    pub fn coordinates(&self) -> Vec3 {
        self.coordinates
    }

    pub fn is_filled(&self) -> bool {
        self.filled
    }

    pub fn set_filled(&mut self, filled: bool) {
        self.filled = filled;
    }
}
