use super::Move;
use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
pub struct UsesGamepad(Gamepad);

#[derive(Component, Debug, Default, Reflect)]
pub struct Player(u8);

pub fn update_movement_with_gamepad(
    mut query: Query<(&mut Move, &UsesGamepad), With<Player>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    for (mut movement, &UsesGamepad(gamepad)) in query.iter_mut() {
        let desired_velocity = Vec2::from((
            axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
                .filter(|&x| x > 0.2)
                .unwrap_or_default(),
            axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
                .filter(|&x| x > 0.2)
                .unwrap_or_default(),
        ));
        movement.desired_velocity = desired_velocity.clamp_length(3.0, 3.0);
    }
}

pub fn update_movement(
    mut query: Query<&mut Move, (With<Player>, Without<UsesGamepad>)>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut movement = query.single_mut();
    let mut v = Vec2::ZERO;

    if keyboard.pressed(KeyCode::Left) {
        v.x -= 1.0
    };
    if keyboard.pressed(KeyCode::Right) {
        v.x += 1.0
    };
    if keyboard.pressed(KeyCode::Down) {
        v.y += 1.0
    };
    if keyboard.pressed(KeyCode::Up) {
        v.y -= 1.0
    };

    movement.desired_velocity = (3.0 * v).clamp_length_max(3.0);
}
