use bevy::prelude::*;

use crate::loading::GameAssets;

pub fn setup_scene(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    commands.spawn((
        SpriteBundle {
            texture: game_assets.tent_texture.clone(),
            transform: Transform::from_translation(Vec3::new(500., -300., 1.)),
            ..default()
        },
    ));
}
