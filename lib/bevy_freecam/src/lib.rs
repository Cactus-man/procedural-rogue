use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};
use std::f32::consts::PI;

/// Tags an entity as capable of panning and orbiting.
#[derive(Component, Debug, Reflect)]
pub struct ControllableCamera {
    pub speed: f32,
    pub sensitivity: f32,
    cursor_grabbed: bool,
}

impl Default for ControllableCamera {
    fn default() -> Self {
        Self {
            speed: 0.2,
            sensitivity: 0.002,
            cursor_grabbed: false,
        }
    }
}

const MOVES: &[(KeyCode, Vec3)] = &[
    (KeyCode::W, Vec3::new(0.0, 0.0, -1.0)),
    (KeyCode::A, Vec3::new(-1.0, 0.0, 0.0)),
    (KeyCode::S, Vec3::new(0.0, 0.0, 1.0)),
    (KeyCode::D, Vec3::new(1.0, 0.0, 0.0)),
    (KeyCode::Space, Vec3::new(0.0, 0.5, 0.0)),
    (KeyCode::C, Vec3::new(0.0, -0.5, 0.0)),
];

fn grab_or_release_cursor(
    mut query: Query<&mut ControllableCamera>,
    mut windows: Query<&mut Window>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();
    let mut camera = query.single_mut();
    if keys.just_pressed(KeyCode::F) && matches!(window.cursor.grab_mode, CursorGrabMode::None) {
        camera.cursor_grabbed = true;
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;

        let resolution = Vec2::new(window.resolution.height(), window.resolution.width());
        window.set_cursor_position(Some(0.5 * resolution));
        info!("Locked cursor to window \"{}\"", window.title);

        keys.clear_just_pressed(KeyCode::F);
    } else if keys.just_pressed(KeyCode::Escape)
        && matches!(window.cursor.grab_mode, CursorGrabMode::Locked)
    {
        camera.cursor_grabbed = false;
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

fn move_camera(
    mut ev_motion: EventReader<MouseMotion>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&ControllableCamera, &mut Transform)>,
) {
    let (camera, mut transform) = query.single_mut();

    if camera.cursor_grabbed {
        let to_move: Vec3 = MOVES
            .into_iter()
            .filter(|&&(code, _)| keys.pressed(code))
            .map(|&(_, v)| v)
            .sum();

        let to_move = transform.rotation * to_move * camera.speed;
        transform.translation += to_move;

        let mouse_motion: Vec2 = ev_motion.iter().map(|ev| ev.delta).sum();
        let direction_change: Vec2 = -PI * mouse_motion * camera.sensitivity;
        transform.rotate_y(direction_change.x);

        let right = transform.right();
        transform.rotate_axis(right, direction_change.y);
    }

    ev_motion.clear();
}

fn configure_camera(mut commands: Commands) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        ControllableCamera {
            ..Default::default()
        },
    ));
}

#[derive(Debug, Default)]
pub struct FreeCamPlugin {
    pub spawn_camera: bool,
}

impl Plugin for FreeCamPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ControllableCamera>()
            .add_systems(Update, (grab_or_release_cursor, move_camera));

        if self.spawn_camera {
            app.add_systems(Startup, configure_camera);
        }
    }
}
