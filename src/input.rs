use bevy::{
    input::Input,
    math::{Vec2, Vec3},
    prelude::*, gizmos,
};
use bevy_rapier2d::prelude::{RapierContext, Collider, QueryFilter};

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

fn player_movement(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Velocity, Option<&Collider>)>,
    rapier_context: Res<RapierContext>,
    mut gizmos: Gizmos,
) {
    let delta = time.delta_seconds();
    for (entity, mut transform, velocity, collider) in query.iter_mut() {
        let mut final_velocity = velocity.0 * SPEED * delta;

        if let Some(collider) = collider {
            let horizontal = Vec2::from((final_velocity.x, 0.));
            let vertical = Vec2::from((0., final_velocity.y));
            let filter = QueryFilter::new().exclude_collider(entity);

            // horizontal shape cast
            if let Some((entity, hit)) = rapier_context.cast_shape(
                transform.translation.truncate(),
                0.,
                horizontal,
                collider,
                4.,
                filter,
            ) {
                if hit.toi <= 1. {
                    final_velocity = Vec2::from((0., final_velocity.y));
                }
            }

            // vertical shape cast
            if let Some((entity, hit)) = rapier_context.cast_shape(
                transform.translation.truncate(),
                0.,
                vertical,
                collider,
                4.,
                filter,
            ) {
                if hit.toi <= 1. {
                    final_velocity = Vec2::from((final_velocity.x, 0.));
                }
            }
        }

        transform.translation += Vec3::from((final_velocity, 0.0));
    }
}

#[derive(Default)]
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ( player_controller, player_movement ).run_if(in_state(GameState::InGame))
        );
    }
}
