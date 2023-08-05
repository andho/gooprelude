use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{loading::GameAssets, inventory::{ItemOnGround, InventoryItemType, Item}};

pub fn setup_scene(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            SpriteBundle {
                texture: game_assets.tent_texture.clone(),
                transform: Transform::from_translation(Vec3::new(500., -300., 1.)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(1., 83.),
                TransformBundle::from_transform(
                    Transform::from_xyz(0., 5., 1.)
                    .with_rotation(Quat::from_rotation_z(-0.35))
                ),
                CollisionGroups::new(Group::GROUP_1, Group::ALL),
                //Name::new("Tent collider"),
            ));
        });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(50., 50.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
            transform: Transform::from_translation(Vec3::new(200., 0., 1.)),
            ..default()
        },
        Collider::cuboid(25., 25.),
        CollisionGroups::new(Group::GROUP_1, Group::ALL),
        //Name::new("Column"),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(50., 50.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
            transform: Transform::from_translation(Vec3::new(-50., -50., 1.)),
            ..default()
        },
        ItemOnGround {
            item: Item {
                name: "ItemA".into(),
                item_type: InventoryItemType::Consumable,
            },
        },
        Collider::cuboid(15., 15.),
        CollisionGroups::new(Group::GROUP_2, Group::ALL),
        //Name::new("ItemA on ground"),
    ));
}
