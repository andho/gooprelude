use bevy::{prelude::*, utils::HashMap, core_pipeline::clear_color::ClearColorConfig};
use bevy_rapier2d::prelude::{Collider, KinematicCharacterController};

use crate::{loading::{LoadingPlugin, GameAssets}, mouse::MousePlugin, input::{MovementPlugin, Velocity}, camera::CameraPlugin, animator::{AnimationKey, Animator, animation_selection}, animation::{SpriteSheetAnimation, AnimationPlugin}, field_of_view::{FovMarker, FieldOfViewPlugin}, scene::setup_scene, inventory::{InventoryPlugin, Inventory}, };

use std::{f32::consts::TAU, fmt::{Display, Formatter, Result}};

const ANIMATION_FPS: u8 = 12;

#[derive(Component)]
pub struct Player;

#[derive(Default)]
pub struct GamePlugin;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    InGame,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_plugins((
                LoadingPlugin::new(GameState::Loading, GameState::InGame),
                MousePlugin,
                MovementPlugin,
                CameraPlugin,
                AnimationPlugin,
                FieldOfViewPlugin,
                InventoryPlugin,
            ))
            .add_systems(OnEnter(GameState::InGame),
                (
                    setup_background,
                    setup_scene,
                    setup_player,
                ).chain()
            )
            .add_systems(Update, (
                update_animation_data,
                animation_selection::<Animations, AnimationData>,
            ).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Default)]
enum Animations {
    #[default]
    Idle,
    Walk,
}

impl Display for Animations {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl AnimationKey for Animations {}

#[derive(Component, Clone, Default, Debug)]
struct AnimationData {
    moving: bool,
}

#[derive(Component)]
pub struct MainCamera;

fn animation_selector(data: &AnimationData) -> Animations {
    match data.moving {
        true => Animations::Walk,
        false => Animations::Idle,
    }
}

pub fn setup_player(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::GRAY),
            },
            ..default()
        },
        MainCamera,
        FovMarker,
    ));

    let texture_handle = game_assets.player_spritesheet.clone();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 11, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let anim_idle_handle =
        animations.add(SpriteSheetAnimation::from_frames(vec![0], ANIMATION_FPS));
    let anim_walk_handle = animations.add(SpriteSheetAnimation::from_frames(
        (1..11).collect(),
        ANIMATION_FPS,
    ));

    let player = Name::new("Player");
    let animator = Animator::new(
        HashMap::from_iter([
            (Animations::Idle, anim_idle_handle),
            (Animations::Walk, anim_walk_handle),
        ]),
        animation_selector,
        player.clone(),
    );

    commands
        .spawn((
            SpatialBundle::from_transform(
                Transform::from_translation(Vec3::splat(1.)),
            ),
            Player,
            Name::new("Player Entity"),
            animator,
            AnimationData::default(),
            Collider::ball(15.),
            KinematicCharacterController::default(),
        ))
        .with_children(|parent| {
            parent
                .spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform::from_rotation(Quat::from_rotation_z(0.25 * TAU)),
                    ..Default::default()
                })
                .insert(player.clone());
        });
}

fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("background/campsite-improved.png"),
            transform: Transform::from_scale(Vec3::splat(0.5)),
            ..Default::default()
        },
        Name::new("bg"),
    ));
}

fn update_animation_data(mut query: Query<(&Velocity, &mut AnimationData)>) {
    for (velocity, mut anim_data) in query.iter_mut() {
        anim_data.moving = velocity.length() > 0.0;
    }
}
