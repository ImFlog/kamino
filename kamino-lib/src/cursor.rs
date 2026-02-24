use bevy::prelude::*;

/// Plugin that handles cursor tracking and mouse button state
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Cursor {
            current_pos: Vec2::default(),
            last_click_pos: Vec2::default(),
            is_pressed: false,
        })
        .add_systems(PreUpdate, cursor_state);
    }
}

/// Resource that tracks the current cursor state
#[derive(Resource)]
pub struct Cursor {
    /// The current position of the cursor in screen coordinates
    pub current_pos: Vec2,
    /// The position where the left mouse button was last pressed
    pub last_click_pos: Vec2,
    /// Whether the left mouse button is currently pressed
    pub is_pressed: bool,
}

fn cursor_state(
    mut cursor_moved_event: MessageReader<CursorMoved>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut cursor: ResMut<Cursor>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };

    for event in cursor_moved_event.read() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, event.position) {
            cursor.current_pos = world_pos;

            if mouse_button_input.just_pressed(MouseButton::Left) {
                cursor.last_click_pos = world_pos;
                cursor.is_pressed = true;
            }
        }

        if mouse_button_input.just_released(MouseButton::Left) {
            cursor.is_pressed = false;
        }
    }
}
