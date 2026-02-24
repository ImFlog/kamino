extern crate kamino_macro;
pub mod board;

mod corner;
mod l;
#[allow(clippy::module_inception)]
mod piece;
mod piece_builder;
mod rectangle;
mod square;
mod z;

use bevy::{math::vec3, prelude::*, sprite::Sprite};

use crate::{
    cursor::Cursor,
    piece::{corner::Corner, l::L, rectangle::Rectangle, square::Square, z::Z},
};
use piece::{Piece, Position};

/// Width of each square in the puzzle pieces, measured in pixels
pub const SQUARE_WIDTH: i32 = 50;

/// Plugin that manages piece spawning, movement, rotation, and rendering
pub struct PiecePlugin;

/// Resource containing all game pieces currently in play
pub struct GameState {
    pub pieces: Vec<Box<dyn Piece>>,
    pub drag_start: Option<Vec<Vec3>>,
}

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(GameState {
            pieces: vec![
                Box::new(Rectangle::new(-300, -200)),
                Box::new(L::new(-100, 100)),
                Box::new(Z::new(200, 200)),
                Box::new(Corner::new(-300, 100)),
                Box::new(Square::new(100, -200)),
            ],
            drag_start: None,
        })
        .add_systems(PreUpdate, clear)
        .add_systems(
            Update,
            (
                click_piece,
                move_piece,
                update_timers,
                embed_in_board.before(release_piece),
                release_piece,
                draw_piece,
            ),
        );
    }
}

// Systems
fn clear(mut commands: Commands, query: Query<Entity, With<Position>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn draw_piece(mut commands: Commands, mut game_state: NonSendMut<GameState>) {
    for piece in game_state.pieces.iter_mut() {
        let color = piece.color();
        let positions = piece.positions();
        let is_error = piece.error_timer() > 0.0;
        for position in positions.iter() {
            if is_error {
                // Spawn a slightly larger black square as an outline
                commands.spawn((
                    Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::new(SQUARE_WIDTH as f32, SQUARE_WIDTH as f32)),
                        ..default()
                    },
                    Transform::from_translation(vec3(position.x, position.y, position.z - 0.1)),
                    Position,
                ));
            }
            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(
                        (SQUARE_WIDTH - 1) as f32,
                        (SQUARE_WIDTH - 1) as f32,
                    )),
                    ..default()
                },
                Transform::from_translation(vec3(position.x, position.y, position.z)),
                Position,
            ));
        }
    }
}

fn update_timers(time: Res<Time>, mut game_state: NonSendMut<GameState>) {
    let delta = time.delta_secs();
    for piece in game_state.pieces.iter_mut() {
        piece.update_error_timer(delta);
    }
}

fn move_piece(cursor: Res<Cursor>, mut game_state: NonSendMut<GameState>) {
    if cursor.is_pressed {
        game_state
            .pieces
            .iter_mut()
            .filter(|piece| piece.is_moving())
            .for_each(|piece| {
                piece.move_to_cursor(&cursor);
            })
    }
}

pub fn click_piece(
    cursor: Res<Cursor>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut game_state: NonSendMut<GameState>,
    mut board: Option<ResMut<board::Board>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut selected_piece_positions = None;
        for piece in game_state.pieces.iter_mut() {
            if piece.is_even_odd(cursor.current_pos) {
                selected_piece_positions = Some(piece.positions());
                piece.set_moving(true);

                // Clear the board positions covered by this piece
                if let Some(ref mut board) = board {
                    board.clear_piece(&piece.positions());
                }
                break;
            }
        }
        if let Some(pos) = selected_piece_positions {
            game_state.drag_start = Some(pos);
            return;
        }
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
        for piece in game_state.pieces.iter_mut() {
            if piece.is_even_odd(cursor.current_pos) {
                if piece.is_moving() {
                    piece.rotate();
                } else if let Some(ref mut board) = board {
                    // If the piece is on the board, we must check if rotation is valid
                    if board.fits_piece(&piece.positions()) {
                        board.clear_piece(&piece.positions());
                        piece.rotate();
                        piece.snap();
                        if board.fits_piece(&piece.positions())
                            && board.can_place_piece(&piece.positions())
                        {
                            board.fill_piece(&piece.positions());
                        } else {
                            // Undo rotation: 3 more clockwise rotations = 1 counter-clockwise
                            // TODO: Improve this.
                            piece.rotate();
                            piece.rotate();
                            piece.rotate();
                            piece.snap();
                            board.fill_piece(&piece.positions());
                            piece.set_error_timer(0.5);
                        }
                    } else {
                        piece.rotate();
                    }
                } else {
                    piece.rotate();
                }
                break;
            }
        }
    }
}

fn release_piece(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut game_state: NonSendMut<GameState>,
) {
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    game_state
        .pieces
        .iter_mut()
        .filter(|piece| piece.is_moving())
        .for_each(|piece| piece.set_moving(false));

    game_state.drag_start = None;
}

pub fn embed_in_board(
    mut game_state: NonSendMut<GameState>,
    board: Option<ResMut<board::Board>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_button_input.just_released(MouseButton::Left) || board.is_none() {
        return;
    }

    let mut board = board.unwrap();

    let drag_start = game_state.drag_start.clone();
    let moving_piece_optional = game_state.pieces.iter_mut().find(|piece| piece.is_moving());
    if moving_piece_optional.is_none() {
        return;
    }
    let moving_piece = moving_piece_optional.unwrap();

    if board.fits_piece(&moving_piece.positions()) {
        // Snap the piece before checking availability
        moving_piece.snap();

        // Check if all the positions are available
        if !board.can_place_piece(&moving_piece.positions()) {
            if let Some(pos) = drag_start {
                moving_piece.set_positions(pos);
                // Re-fill the board positions if the piece was originally on the board
                board.fill_piece(&moving_piece.positions());
            }

            moving_piece.set_error_timer(0.5);
        } else {
            // Fill positions on the board
            board.fill_piece(&moving_piece.positions());
        }
    }
}

#[cfg(test)]
pub mod mod_tests {
    pub use super::click_piece;
    pub use super::embed_in_board;
}
