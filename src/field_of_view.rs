use bevy::{
    core_pipeline::{
        clear_color::ClearColorConfig, core_2d,
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    prelude::*,
    render::{
        extract_component::{
            ExtractComponent, ExtractComponentPlugin,
        },
        render_graph::{Node, NodeRunError, RenderGraphApp, RenderGraphContext},
        render_resource::{
            BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
            BindGroupLayoutEntry, BindingResource, BindingType, CachedRenderPipelineId,
            ColorTargetState, ColorWrites, FragmentState, MultisampleState, Operations,
            PipelineCache, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
            TextureFormat, TextureSampleType, TextureViewDimension, Extent3d, TextureDescriptor, TextureDimension, TextureUsages, PrimitiveTopology,
        },
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        view::{ExtractedView, ViewTarget, RenderLayers},
        RenderApp, extract_resource::{ExtractResource, ExtractResourcePlugin}, camera::RenderTarget, render_asset::RenderAssets, mesh::Indices,
    },
    sprite::MaterialMesh2dBundle,
};

use crate::game::{GameState, setup_player, Player};

#[derive(Resource, Clone, Deref, ExtractResource)]
struct FieldOfViewImage(Handle<Image>);

fn vision_cone_texture_setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 1280,
        height: 720,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);

    let image_handle = images.add(image);

    commands.insert_resource(FieldOfViewImage(image_handle))
}

#[derive(Component, Default, Clone, Copy, ExtractComponent)]
pub struct FovMarker;

const FOV_INTENSITY: f32 = 0.8;

fn camera_setup(
    mut commands: Commands,
    fov_image: Res<FieldOfViewImage>,
) {
    let first_pass_layer = RenderLayers::layer(1);

    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::rgb(
                        FOV_INTENSITY, FOV_INTENSITY, FOV_INTENSITY)),
            },
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(fov_image.0.clone()),
                ..default()
            },
            ..default()
        },
        first_pass_layer,
    ));

    // Using this to test if the rendered fov texture is correct
    //commands.spawn((
    //    SpriteBundle {
    //        texture: fov_image.0.clone(),
    //        transform: Transform::from_translation(Vec3::new(-100., 1., 1.)),
    //        ..Default::default()
    //    },
    //));
}

fn fov_mesh_setup(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
                material: materials.add(ColorMaterial::from(Color::rgba(1., 1., 1., 1.))),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..default()
            },
            Name::new("fov"),
            first_pass_layer,
        ));
    });
}

pub struct FieldOfViewPlugin;

impl Plugin for FieldOfViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                ExtractResourcePlugin::<FieldOfViewImage>::default(),
                ExtractComponentPlugin::<FovMarker>::default(),
            ))
            .add_systems(Startup, vision_cone_texture_setup)
            .add_systems(OnEnter(GameState::InGame), (
                apply_deferred,
                camera_setup.after(vision_cone_texture_setup),
                fov_mesh_setup.after(setup_player),
            ).chain());

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<FieldOfViewNode>(
                core_2d::graph::NAME,
                FieldOfViewNode::NAME,
            )
            .add_render_graph_edges(
                core_2d::graph::NAME,
                &[
                    core_2d::graph::node::TONEMAPPING,
                    FieldOfViewNode::NAME,
                    core_2d::graph::node::END_MAIN_PASS_POST_PROCESSING,
                ],
            );
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<FieldOfViewPipeline>();
    }
}

struct FieldOfViewNode {
    query: QueryState<&'static ViewTarget, (With<ExtractedView>, With<FovMarker>)>,
}

impl FieldOfViewNode {
    pub const NAME: &str = "fov_render";
}

impl FromWorld for FieldOfViewNode {
    fn from_world(world: &mut World) -> Self {
        Self {
            query: QueryState::new(world),
        }
    }
}

impl Node for FieldOfViewNode {
    fn update(&mut self, world: &mut World) {
        self.query.update_archetypes(world);
    }

    fn run(
        &self,
        graph_context: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let view_entity = graph_context.view_entity();
        let fov_image_handle = world.resource::<FieldOfViewImage>().0.clone();
        let gpu_images = world.resource::<RenderAssets<Image>>();
        let fov_image = &gpu_images[&fov_image_handle];

        let Ok(view_target) = self.query.get_manual(world, view_entity) else {
            return Ok(());
        };

        let post_process_pipeline = world.resource::<FieldOfViewPipeline>();

        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id) else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let bind_group = render_context
            .render_device()
            .create_bind_group(&BindGroupDescriptor {
                label: Some("post_process_bind_group"),
                layout: &post_process_pipeline.layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        // Make sure to use the source view
                        resource: BindingResource::TextureView(post_process.source),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&post_process_pipeline.sampler),
                    },
                ],
            });

        let fov_bind_group = render_context
            .render_device()
            .create_bind_group(&BindGroupDescriptor {
                label: Some("fov_texture_bind_group"),
                layout: &post_process_pipeline.fov_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&fov_image.texture_view),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&fov_image.sampler),
                    },
                ],
            });

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("fov_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.set_bind_group(1, &fov_bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

#[derive(Resource)]
struct FieldOfViewPipeline {
    layout: BindGroupLayout,
    fov_layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for FieldOfViewPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("fov_view_bind_group_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let fov_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("fov_texture_bind_group_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let shader = world
            .resource::<AssetServer>()
            .load("shaders/fov.wgsl");

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("fov_pipeline".into()),
                layout: vec![layout.clone(), fov_layout.clone()],
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: vec![],
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        Self {
            layout,
            fov_layout,
            sampler,
            pipeline_id,
        }
    }
}
