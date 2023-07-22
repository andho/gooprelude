use bevy::prelude::*;

use crate::game::Player;

fn vision_cone_setup() {
}

pub fn vision_cone_gizmo(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<Player>>,
) {
    let fov_angle = 1.2;
    if let Ok(transform) = query.get_single() {
        gizmos.ray_2d(
            transform.translation.truncate(),
            transform.right().truncate() * 500.,
            Color::GREEN
        );

        let mut ccw_transform = transform.clone();
        ccw_transform.rotate_z(fov_angle);
        let mut cw_transform = transform.clone();
        cw_transform.rotate_z(-fov_angle);
        gizmos.ray_2d(
            transform.translation.truncate(),
            ccw_transform.right().truncate() * 500.,
            Color::GREEN
        );
        gizmos.ray_2d(
            transform.translation.truncate(),
            cw_transform.right().truncate() * 500.,
            Color::GREEN
        );
    }
}
