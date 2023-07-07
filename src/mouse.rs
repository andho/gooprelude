use std::f32::consts::{FRAC_PI_2, PI};
use std::{fmt::Debug, hash::Hash};

use bevy::{
    math::{Quat, Vec2, Vec3},
    prelude::*,
    render::camera::RenderTarget,
};

use crate::game::{Player, GameState};

fn mouse_look(
    windows: Query<&Window>,
    mut params: ParamSet<(
        Query<(&Camera, &GlobalTransform)>,
        Query<&mut Transform, With<Player>>,
    )>,
    mut mouse_pos: Local<Option<Vec3>>,
) {
    let wnd = windows.single();

    for (camera, camera_transform) in params.p0().iter() {
        if let Some(screen_pos) = wnd.cursor_position() {
            //*mouse_pos = Some(screen_pos.extend(-1.0));
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            //eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
            *mouse_pos = Some(Vec3::new(world_pos.x, world_pos.y, 1.0));
        } else {
            *mouse_pos = None;
        }
    }

    if let Some(mouse_pos) = *mouse_pos {
        for mut transform in params.p1().iter_mut() {
            let mouse_pos_2d = mouse_pos.truncate();
            let transform_2d = transform.translation.truncate();
            let v = mouse_pos_2d - transform_2d;
            let b = v.normalize();
            let a = Vec2::new(1.0, 0.0);
            let new_rotation = Quat::from_rotation_arc_2d(a, b).normalize();

            let old_rotation = transform.rotation;
            transform.rotation = old_rotation.lerp(new_rotation, 0.2);
        }
    }
}

pub trait MouseState: Debug + Clone + Copy + PartialEq + Eq + Hash + Sync + Send {}

#[derive(Default)]
pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(mouse_look.in_set(OnUpdate(GameState::InGame)));
    }
}
