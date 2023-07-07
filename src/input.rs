use std::{fmt::Debug, hash::Hash};

use bevy::{
    input::Input,
    math::{Vec2, Vec3},
    prelude::*,
};

use crate::game::{Player, GameState};

const SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Velocity(Vec2);

impl Velocity {
    pub fn length(&self) -> f32 {
        self.0.length()
    }
}

fn player_controller(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<Entity, With<Player>>,
) {
    let mut vec2 = Vec2::default();
    if keyboard_input.pressed(KeyCode::W) {
        vec2.y = 1.0;
    } else if keyboard_input.pressed(KeyCode::S) {
        vec2.y = -1.0;
    } else {
        vec2.y = 0.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        vec2.x = 1.0;
    } else if keyboard_input.pressed(KeyCode::A) {
        vec2.x = -1.0;
    } else {
        vec2.x = 0.0;
    }

    let velocity = Velocity(vec2);

    let entity = query.single();
    commands.entity(entity).insert(velocity);
}

fn player_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    let delta = time.delta_seconds();
    for (mut transform, velocity) in query.iter_mut() {
        let final_velocity = velocity.0 * SPEED * delta;
        transform.translation += Vec3::from((final_velocity, 0.0));
    }
}

#[derive(Default)]
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_controller.in_set(OnUpdate(GameState::InGame)))
            .add_system(player_movement.in_set(OnUpdate(GameState::InGame)));
    }
}
