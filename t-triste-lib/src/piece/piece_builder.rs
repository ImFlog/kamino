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
    fn test_board_positions() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);

        // When
        let board = Board::new();
        commands.insert_resource(board);
        command_queue.apply(&mut world);

        // Then - Check that board has correct bounds
        let board = world.get_resource::<Board>().unwrap();
        assert_eq!(board.min_x, -100f32);
        assert_eq!(board.min_y, -50f32);
        // Board is 3 rows x 5 cols, so max values should account for that
        // Max is calculated as start + (count * SQUARE_WIDTH), not (count - 1) * SQUARE_WIDTH
        assert_eq!(board.max_x, 100f32);
        assert_eq!(board.max_y, 50f32);

        // Check that we have correct number of positions (3 rows * 5 cols = 15)
        assert_eq!(board.positions.len(), 15);
    }
}
