use bevy::{
    input::Input,
    math::{Vec2, Vec3},
    prelude::*, gizmos,
};
use bevy_rapier2d::prelude::{RapierContext, Collider, QueryFilter, KinematicCharacterController};

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

// @deprecated because I couldn't handle "corner" cases XD
fn system_manual_player_movement(
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

            // diagonal shape cast
            if let Some((entity, hit)) = rapier_context.cast_shape(
                transform.translation.truncate(),
                0.,
                final_velocity,
                collider,
                4.,
                filter,
            ) {
                println!("Hit entity {:?} with config {:?}", entity, hit);
                gizmos.circle_2d(hit.witness1, 10., Color::GREEN);
                gizmos.circle_2d(hit.witness2, 10., Color::GREEN);

                if hit.toi <= 4. {
                    final_velocity = Vec2::from((0., 0.));
                }
            }

            // horizontal shape cast
            if let Some((entity, hit)) = rapier_context.cast_shape(
                transform.translation.truncate(),
                0.,
                horizontal,
                collider,
                4.,
                filter,
            ) {
                println!("Hit entity {:?} with config {:?}", entity, hit);
                gizmos.circle_2d(hit.witness1, 10., Color::GREEN);
                gizmos.circle_2d(hit.witness2, 10., Color::GREEN);

                if hit.toi <= 4. {
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
                println!("Hit entity {:?} with config {:?}", entity, hit);
                gizmos.circle_2d(hit.witness1, 10., Color::GREEN);
                gizmos.circle_2d(hit.witness2, 10., Color::GREEN);

                if hit.toi <= 4. {
                    final_velocity = Vec2::from((final_velocity.x, 0.));
                }
            }
        }

        transform.translation += Vec3::from((final_velocity, 0.0));
    }
}

fn system_kinematic_movement(
    mut controllers: Query<(&mut KinematicCharacterController, &Velocity), With<Player>>,
    time: Res<Time>,
) {
    let Ok((mut controller, velocity)) = controllers.get_single_mut() else {
        return;
    };

    let delta = time.delta_seconds();
    let velocity = velocity.0 * SPEED * delta;
    controller.translation = Some(velocity);
}

#[derive(Default)]
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_controller,
                //system_manual_player_movement,
                system_kinematic_movement,
            ).run_if(in_state(GameState::InGame))
        );
    }
}
