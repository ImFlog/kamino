use bevy::prelude::*;

use crate::piece::SQUARE_WIDTH;

pub struct PieceBuilder {
    #[allow(dead_code)]
    pub positions: Vec<Vec3>,
}

impl PieceBuilder {
    pub fn new_horizontal_rectangle(
        start_x: i32,
        start_y: i32,
        length: i32,
        z_index: f32,
    ) -> Vec<Vec3> {
        let mut squares = vec![];
        for i in 0..length {
            squares.push(Vec3::new(
                (start_x + i * SQUARE_WIDTH) as f32,
                start_y as f32,
                z_index,
            ))
        }
        squares
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::world::CommandQueue;

    use super::*;
    use crate::piece::{
        board::Board, corner::Corner, l::L, piece::Piece, piece::Position, square::Square, z::Z,
        SQUARE_WIDTH,
    };

    #[test]
    fn test_build_l_piece() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);

        // When
        // *
        // *
        // * *
        let piece = L::new(0, 0);
        let positions = piece.positions();
        let color = piece.color();

        for position in positions.iter() {
            commands.spawn((
                bevy::sprite::Sprite {
                    color,
                    custom_size: Some(Vec2::new(
                        (SQUARE_WIDTH - 1) as f32,
                        (SQUARE_WIDTH - 1) as f32,
                    )),
                    ..default()
                },
                Transform::from_translation(*position),
                Position,
            ));
        }
        command_queue.apply(&mut world);

        // Then
        let results = world
            .query_filtered::<&Transform, With<Position>>()
            .iter(&world)
            .map(|t| t.translation)
            .collect::<Vec<_>>();

        assert_eq!(
            results,
            vec![
                Vec3::new(0., 0., 1.),
                Vec3::new(SQUARE_WIDTH as f32, 0., 1.),
                Vec3::new(0., SQUARE_WIDTH as f32, 1.),
                Vec3::new(0., 2. * (SQUARE_WIDTH as f32), 1.),
            ]
        );
    }

    #[test]
    fn test_build_z_piece() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);

        // When
        // * *
        //   * *
        let piece = Z::new(0, 0);
        let positions = piece.positions();
        let color = piece.color();

        for position in positions.iter() {
            commands.spawn((
                bevy::sprite::Sprite {
                    color,
                    custom_size: Some(Vec2::new(
                        (SQUARE_WIDTH - 1) as f32,
                        (SQUARE_WIDTH - 1) as f32,
                    )),
                    ..default()
                },
                Transform::from_translation(*position),
                Position,
            ));
        }
        command_queue.apply(&mut world);

        // Then
        let results = world
            .query_filtered::<&Transform, With<Position>>()
            .iter(&world)
            .map(|t| t.translation)
            .collect::<Vec<_>>();

        assert_eq!(
            results,
            vec![
                Vec3::new(0., 0., 1.),
                Vec3::new(SQUARE_WIDTH as f32, 0., 1.),
                Vec3::new(SQUARE_WIDTH as f32, SQUARE_WIDTH as f32, 1.),
                Vec3::new(2. * SQUARE_WIDTH as f32, SQUARE_WIDTH as f32, 1.)
            ]
        );
    }

    #[test]
    fn test_build_corner_piece() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);

        // When
        // *
        // * *
        let piece = Corner::new(0, 0);
        let positions = piece.positions();
        let color = piece.color();

        for position in positions.iter() {
            commands.spawn((
                bevy::sprite::Sprite {
                    color,
                    custom_size: Some(Vec2::new(
                        (SQUARE_WIDTH - 1) as f32,
                        (SQUARE_WIDTH - 1) as f32,
                    )),
                    ..default()
                },
                Transform::from_translation(*position),
                Position,
            ));
        }
        command_queue.apply(&mut world);

        // Then
        let results = world
            .query_filtered::<&Transform, With<Position>>()
            .iter(&world)
            .map(|t| t.translation)
            .collect::<Vec<_>>();
        assert_eq!(
            results,
            vec![
                Vec3::new(0., 0., 1.),
                Vec3::new(SQUARE_WIDTH as f32, 0., 1.),
                Vec3::new(0., SQUARE_WIDTH as f32, 1.),
            ]
        );
    }

    #[test]
    fn test_build_dot_square_piece() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);

        // When
        // *
        let piece = Square::new(0, 0);
        let positions = piece.positions();
        let color = piece.color();

        for position in positions.iter() {
            commands.spawn((
                bevy::sprite::Sprite {
                    color,
                    custom_size: Some(Vec2::new(
                        (SQUARE_WIDTH - 1) as f32,
                        (SQUARE_WIDTH - 1) as f32,
                    )),
                    ..default()
                },
                Transform::from_translation(*position),
                Position,
            ));
        }
        command_queue.apply(&mut world);

        // Then
        let results = world
            .query_filtered::<&Transform, With<Position>>()
            .iter(&world)
            .map(|t| t.translation)
            .collect::<Vec<_>>();
        assert_eq!(results, vec![Vec3::new(0., 0., 1.),]);
    }

    #[test]
    fn test_click_piece_saves_drag_start() {
        use bevy::ecs::system::RunSystemOnce;
        use crate::piece::mod_tests::click_piece;
        use crate::piece::GameState;
        use crate::piece::rectangle::Rectangle;
        use crate::cursor::Cursor;

        // Given
        let mut world = World::default();
        let piece = Rectangle::new(0, 0);
        let initial_pos = piece.positions();
        let game_state = GameState {
            pieces: vec![Box::new(piece)],
            drag_start: None,
        };
        world.insert_non_send_resource(game_state);

        let mut input = ButtonInput::<MouseButton>::default();
        input.press(MouseButton::Left);
        world.insert_resource(input);

        let cursor = Cursor {
            current_pos: Vec2::ZERO,
            last_click_pos: Vec2::ZERO,
            is_pressed: true,
        };
        world.insert_resource(cursor);

        // When
        let _ = world.run_system_once(click_piece);

        // Then
        let game_state = world.get_non_send_resource::<GameState>().unwrap();
        assert_eq!(game_state.drag_start, Some(initial_pos));
        assert!(game_state.pieces[0].is_moving());
    }

    #[test]
    fn test_embed_in_board() {
        use bevy::ecs::system::RunSystemOnce;
        use crate::piece::mod_tests::embed_in_board;
        use crate::piece::GameState;
        use crate::piece::rectangle::Rectangle;

        // Given
        let mut world = World::default();
        let mut board = Board::new();
        // Manually fill one position
        board.positions[0].set_filled(true);
        let filled_coords = board.positions[0].coordinates();
        world.insert_resource(board);

        let mut piece = Rectangle::new(1000, 1000); // Far away
        let initial_pos = piece.positions();
        piece.set_moving(true);
        let game_state = GameState {
            pieces: vec![Box::new(piece)],
            drag_start: Some(initial_pos),
        };
        world.insert_non_send_resource(game_state);

        let mut input = ButtonInput::<MouseButton>::default();
        input.press(MouseButton::Left);
        input.release(MouseButton::Left);
        world.insert_resource(input);

        // When - piece is far away, nothing should happen
        let _ = world.run_system_once(embed_in_board);

        let game_state = world.get_non_send_resource::<GameState>().unwrap();
        let piece = &game_state.pieces[0];
        assert_eq!(piece.positions()[0], Vec3::new(1000., 1000., 1.));

        // When - piece is over the board but on filled position
        let mut game_state = world.remove_non_send_resource::<GameState>().unwrap();
        game_state.pieces[0].move_to_position(filled_coords);
        world.insert_non_send_resource(game_state);

        let mut input = world.get_resource_mut::<ButtonInput<MouseButton>>().unwrap();
        input.press(MouseButton::Left);
        input.release(MouseButton::Left);

        let _ = world.run_system_once(embed_in_board);

        let game_state = world.get_non_send_resource::<GameState>().unwrap();
        let piece = &game_state.pieces[0];
        // Should be reset to initial position (1000, 1000, 1)
        assert_eq!(piece.positions()[0], Vec3::new(1000., 1000., 1.));

        // When - piece is over empty board position
        let mut board = world.get_resource_mut::<Board>().unwrap();
        board.positions[0].set_filled(false);
        let empty_coords = board.positions[1].coordinates();

        let mut game_state = world.remove_non_send_resource::<GameState>().unwrap();
        game_state.pieces[0].move_to_position(empty_coords);
        world.insert_non_send_resource(game_state);

        let mut input = world.get_resource_mut::<ButtonInput<MouseButton>>().unwrap();
        input.press(MouseButton::Left);
        input.release(MouseButton::Left);

        let _ = world.run_system_once(embed_in_board);

        let game_state = world.get_non_send_resource::<GameState>().unwrap();
        let piece = &game_state.pieces[0];
        // Should be snapped (already at snapped position)
        assert_eq!(piece.positions()[0], empty_coords);

        // Board position should be filled now
        let board = world.get_resource::<Board>().unwrap();
        assert!(board.positions.iter().find(|bp| bp.coordinates().xy().distance(empty_coords.xy()) < 0.1).unwrap().is_filled());
    }

    #[test]
    fn test_unsnap_from_board() {
        use bevy::ecs::system::RunSystemOnce;
        use crate::piece::mod_tests::embed_in_board;
        use crate::piece::GameState;
        use crate::piece::rectangle::Rectangle;

        // Given
        let mut world = World::default();
        let board = Board::new();
        world.insert_resource(board);

        // Piece starts on board (at 0,0,1)
        let mut piece = Rectangle::new(0, 0);
        let snapped_pos = piece.positions();
        piece.set_moving(true);

        // We moved it far away
        piece.move_to_position(Vec3::new(1000., 1000., 1.));

        let game_state = GameState {
            pieces: vec![Box::new(piece)],
            drag_start: Some(snapped_pos), // Drag started on board
        };
        world.insert_non_send_resource(game_state);

        let mut input = ButtonInput::<MouseButton>::default();
        input.press(MouseButton::Left);
        input.release(MouseButton::Left);
        world.insert_resource(input);

        // When
        let _ = world.run_system_once(embed_in_board);

        // Then
        let game_state = world.get_non_send_resource::<GameState>().unwrap();
        let piece = &game_state.pieces[0];
        // FAIL EXPECTED: piece should stay at (1000, 1000, 1), but current logic will reset it to (0, 0, 1)
        assert_eq!(piece.positions()[0], Vec3::new(1000., 1000., 1.), "Piece should stay outside the board");
    }
}
