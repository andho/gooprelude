use bevy::{prelude::*, render::{render_resource::{PrimitiveTopology, VertexFormat, TextureUsages, TextureDimension, TextureFormat, TextureDescriptor, Extent3d}, mesh::{MeshVertexAttribute, Indices}, view::RenderLayers, camera::RenderTarget}, sprite::{Mesh2dHandle, MaterialMesh2dBundle}, core_pipeline::clear_color::ClearColorConfig};

use crate::game::{Player, GameState, setup_player};

pub struct VisionConePlugin;

impl Plugin for VisionConePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (
            apply_deferred,
            vision_cone_setup
        ).chain().after(setup_player));
    }
}

// Entities with this component is rendered before main camera
#[derive(Component)]
struct FirstPassComponent;

fn vision_cone_setup(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(1);

    let fov_angle = 1.2;
    let view_distance = 500.;
    let steps = 32_u32;
    let increment = (fov_angle * 2.) / steps as f32;

    let Ok((entity, transform)) = query.get_single() else {
        return;
    };

    let mut fov_mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut angle_sweeper = transform.clone();
    angle_sweeper.rotate_z(fov_angle);

    let uvs: Vec<[f32; 2]> = vec![[1., 1.]; 34];
    let normals: Vec<[f32; 3]> = vec![[0., 0., 1.]; 34];
    let mut v_pos = vec![[0.0, 0.0, 1.0]];
    let vertex = angle_sweeper.right().truncate() * view_distance;
    v_pos.push([vertex.x, vertex.y, 0.]);

    for _step in 0..steps {
        angle_sweeper.rotate_z(-increment);
        let vertex = angle_sweeper.right().truncate() * view_distance;
        v_pos.push([vertex.x, vertex.y, 1.]);
    }

    fov_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    fov_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    fov_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    let mut v_color: Vec<u32> = vec![Color::BLACK.as_linear_rgba_u32()];
    v_color.extend_from_slice(&[Color::YELLOW.as_linear_rgba_u32(); 33]);
    //fov_mesh.insert_attribute(
    //    MeshVertexAttribute::new("Vertex_Color", 1, VertexFormat::Uint32),
    //    v_color,
    //);

    let mut indices = vec![];
    for i in 1..steps {
        indices.extend_from_slice(&[0, i, i + 1]);
    }
    fov_mesh.set_indices(Some(Indices::U32(indices)));

    commands.entity(entity).with_children(|parent| {
        parent.spawn((
        //commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(fov_mesh).into(),
                material: materials.add(ColorMaterial::from(Color::rgba(0., 0., 0., 0.5))),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..default()
            },
            Name::new("fov"),
            first_pass_layer,
        ));
    });

    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::WHITE),
            },
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        first_pass_layer,
    ));
}

fn vision_cone_gizmo(
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
