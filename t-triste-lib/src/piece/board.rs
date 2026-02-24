use bevy::prelude::*;
use bevy::sprite::Sprite;

use crate::piece::SQUARE_WIDTH;

use super::piece_builder::PieceBuilder;

/// Plugin that creates and renders the game board
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::new())
            .add_systems(Startup, draw_board);
    }
}

/// Marker component for entities that are part of the board
#[derive(Component)]
struct BoardPosition;

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
    pub positions: Vec<Vec3>,
    /// Minimum X coordinate of the board (left edge)
    pub min_x: f32,
    /// Minimum Y coordinate of the board (bottom edge)
    pub min_y: f32,
    /// Maximum X coordinate of the board (right edge)
    pub max_x: f32,
    /// Maximum Y coordinate of the board (top edge)
    pub max_y: f32,
    // TODO: Track which positions are filled - vec[bool[]] ?
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

        for i in 0..nb_rows {
            positions.append(&mut PieceBuilder::new_horizontal_rectangle(
                start_x,
                start_y + (i * SQUARE_WIDTH),
                nb_cols,
                0.,
            ));
        }
        Board {
            positions,
            min_x: (start_x) as f32,
            min_y: (start_y) as f32,
            max_x: (start_x + (nb_cols - 1) * SQUARE_WIDTH) as f32,
            max_y: (start_y + (nb_rows - 1) * SQUARE_WIDTH) as f32,
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
            Transform::from_translation(*position),
            BoardPosition,
        ));
    });
}
