use std::{fmt::Debug, hash::Hash};

use bevy::{
    math::Vec2,
    prelude::*,
};

use crate::game::{Player, GameState};

fn camera_movement(
    mut transforms: ParamSet<(
        Query<&mut Transform, With<Camera>>,
        Query<&Transform, With<Player>>,
    )>,
    mut player_position: Local<Vec2>,
) {
    for player_transform in transforms.p1().iter() {
        *player_position = Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y,
        );
    }

    for mut camera_transform in transforms.p0().iter_mut() {
        *camera_transform = Transform::from_xyz(player_position.x, player_position.y, camera_transform.translation.z);
    }
}

pub trait CameraState: Debug + Clone + Copy + PartialEq + Eq + Hash + Sync + Send {}

#[derive(Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(camera_movement.in_set(OnUpdate(GameState::InGame)));
    }
}
