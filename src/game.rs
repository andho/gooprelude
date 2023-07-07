use bevy::{prelude::*, utils::HashMap};

use crate::{loading::LoadingPlugin, mouse::MousePlugin, input::{MovementPlugin, Velocity}, camera::CameraPlugin, animator::{AnimationKey, Animator, animation_selection}, animation::{SpriteSheetAnimation, AnimationPlugin}};

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
            .add_plugin(LoadingPlugin::new(GameState::Loading, GameState::InGame))
            .add_plugin(MousePlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(AnimationPlugin)
            .add_system(setup_background.in_schedule(OnEnter(GameState::InGame)))
            .add_system(setup_player.in_schedule(OnEnter(GameState::InGame)))
            .add_system(
                animation_selection::<Animations, AnimationData>.in_set(OnUpdate(GameState::InGame)),
            )
            .add_system(update_animation_data.in_set(OnUpdate(GameState::InGame)));
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

fn animation_selector(data: AnimationData) -> Animations {
    match data.moving {
        true => Animations::Walk,
        false => Animations::Idle,
    }
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("character/character-sheet.png");
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
        .spawn(SpatialBundle::from_transform(
            Transform::from_translation(Vec3::splat(1.)),
        ))
        .insert(Player)
        .insert(animator)
        .insert(AnimationData::default())
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
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background/campsite-improved.png"),
        transform: Transform::from_scale(Vec3::splat(0.5)),
        ..Default::default()
    });
}

fn update_animation_data(mut query: Query<(&Velocity, &mut AnimationData)>) {
    for (velocity, mut anim_data) in query.iter_mut() {
        anim_data.moving = velocity.length() > 0.0;
    }
}
