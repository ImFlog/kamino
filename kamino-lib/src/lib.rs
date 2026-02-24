mod cursor;
mod piece;

use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use piece::SQUARE_WIDTH;

// Plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(1., 0.90, 1.)))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "kamino".to_string(),
                    resolution: (800, 600).into(),
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }))
            .add_systems(Startup, setup_camera)
            .add_plugins(cursor::CursorPlugin)
            .add_plugins(piece::board::BoardPlugin)
            .add_plugins(piece::PiecePlugin);
    }
}

// System
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
