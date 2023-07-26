use std::{fmt::Debug, hash::Hash};

use bevy::{
    math::{Quat, Vec2},
    prelude::*,
};

use crate::game::{Player, GameState, MainCamera};

fn mouse_look(
    windows: Query<&Window>,
    cam_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut transform_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    (|| {
        let wnd = windows.single();

        let (camera, camera_transform) = cam_query.get_single().ok()?;

        let mut transform = transform_query.get_single_mut().ok()?;

        let mouse_pos_2d = wnd.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))?;

        let transform_2d = transform.translation.truncate();
        let v = mouse_pos_2d - transform_2d;
        let b = v.normalize();
        let a = Vec2::new(1.0, 0.0);
        let new_rotation = Quat::from_rotation_arc_2d(a, b).normalize();

        let old_rotation = transform.rotation;
        transform.rotation = old_rotation.lerp(new_rotation, 1. - f32::powf(0.002, time.delta_seconds()));
        //transform.rotation = new_rotation;

        Some(())
    })();
}

pub trait MouseState: Debug + Clone + Copy + PartialEq + Eq + Hash + Sync + Send {}

#[derive(Default)]
pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, mouse_look.run_if(in_state(GameState::InGame)));
    }
}
