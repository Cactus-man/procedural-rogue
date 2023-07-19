mod player;

pub use player::Player;
use player::{update_movement, update_movement_with_gamepad};

use bevy::prelude::*;

#[derive(Clone, Component, Debug, Reflect)]
pub struct Move {
    pub desired_velocity: Vec2,
    velocity: Vec2,
}

impl Move {
    pub fn velocity(&self) -> Vec3 {
        Vec3::new(self.velocity.x, 0.0, self.velocity.y)
    }
}

impl Default for Move {
    fn default() -> Self {
        Self {
            desired_velocity: Vec2::ZERO,
            velocity: Vec2::ZERO,
        }
    }
}

fn apply_acceleration(mut query: Query<&mut Move>, time: Res<Time>) {
    for mut moving in query.iter_mut() {
        let dv = moving.desired_velocity - moving.velocity;
        moving.velocity += 3.0 * dv * time.delta_seconds();
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Move)>, time: Res<Time>) {
    for (mut transform, moving) in query.iter_mut() {
        let moved = moving.velocity * time.delta_seconds();
        transform.translation += Vec3::new(moved.x, 0.0, moved.y);
    }
}

#[derive(Debug)]
pub struct MovementPlugin();

impl Default for MovementPlugin {
    fn default() -> Self {
        Self()
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<Move>()
            .add_systems(
                Update,
                (
                    (update_movement, update_movement_with_gamepad).before(apply_acceleration),
                    apply_acceleration,
                    (apply_velocity).after(apply_acceleration),
                ),
            );
    }
}
