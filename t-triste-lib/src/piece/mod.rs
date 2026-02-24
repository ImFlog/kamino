pub mod board;

mod corner;
mod l;
#[allow(clippy::module_inception)]
mod piece;
mod piece_builder;
mod rectangle;
mod square;
mod z;

extern crate t_triste_macro;

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
pub struct GameState(pub Vec<Box<dyn Piece>>);

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(GameState(vec![
            Box::new(Rectangle::new(-300, -200)),
            Box::new(L::new(-100, 100)),
            Box::new(Z::new(200, 200)),
            Box::new(Corner::new(-300, 100)),
            Box::new(Square::new(100, -200)),
        ]))
        .add_systems(PreUpdate, clear)
        .add_systems(Update, (
            click_piece,
            move_piece,
            embed_in_board.before(release_piece),
            release_piece,
            draw_piece
        ));
    }
}

// Systems
fn clear(mut commands: Commands, query: Query<Entity, With<Position>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn draw_piece(mut commands: Commands, mut game_state: NonSendMut<GameState>) {
    for piece in game_state.0.iter_mut() {
        let color = piece.color();
        let positions = piece.positions();
        for position in positions.iter() {
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

fn move_piece(cursor: Res<Cursor>, mut game_state: NonSendMut<GameState>) {
    if cursor.is_pressed {
        game_state
            .0
            .iter_mut()
            .filter(|piece| piece.is_moving())
            .for_each(|piece| {
                piece.move_it(&cursor);
            })
    }
}

fn click_piece(
    cursor: Res<Cursor>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut game_state: NonSendMut<GameState>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        for piece in game_state.0.iter_mut() {
            if piece.is_even_odd(cursor.current_pos) {
                piece.set_moving(true);
                return;
            }
        }
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
        for piece in game_state.0.iter_mut() {
            if piece.is_even_odd(cursor.current_pos) {
                piece.rotate();
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
        .0
        .iter_mut()
        .filter(|piece| piece.is_moving())
        .for_each(|piece| piece.set_moving(false));
}

fn embed_in_board(
    mut game_state: NonSendMut<GameState>,
    board: Option<Res<board::Board>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_button_input.just_released(MouseButton::Left) || board.is_none() {
        return;
    }

    let board = board.unwrap();

    let moving_piece_optional = game_state.0.iter_mut().find(|piece| piece.is_moving());
    if moving_piece_optional.is_none() {
        return;
    }
    let moving_piece = moving_piece_optional.unwrap();

    let tolerance = (SQUARE_WIDTH / 2) as f32;
    let in_board = moving_piece.positions().iter().all(|t| {
        board.min_x - tolerance <= t.x
            && t.x <= board.max_x + tolerance
            && board.min_y - tolerance <= t.y
            && t.y <= board.max_y + tolerance
    });

    if in_board {
        moving_piece.snap();
    }
}
